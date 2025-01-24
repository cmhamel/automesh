use automesh::{
    FiniteElements, HexahedralFiniteElements, Nel, Octree, Scale, Smoothing, Tree, Vector, Voxels,
};
use clap::{Parser, Subcommand};
use conspire::math::TensorArray;
use ndarray_npy::{ReadNpyError, WriteNpyError};
use netcdf::Error as ErrorNetCDF;
use std::{io::Error as ErrorIO, path::Path, time::Instant};
use vtkio::Error as ErrorVtk;

macro_rules! about {
    () => {
        format!(
            "

     @@@@@@@@@@@@@@@@
      @@@@  @@@@@@@@@@
     @@@@  @@@@@@@@@@@
    @@@@  @@@@@@@@@@@@    \x1b[1;4m{}: Automatic mesh generation\x1b[0m
      @@    @@    @@      {}
      @@    @@    @@      {}
    @@@@@@@@@@@@  @@@
    @@@@@@@@@@@  @@@@     \x1b[1;4mNotes:\x1b[0m
    @@@@@@@@@@ @@@@@ @    - Input/output file types are inferred
     @@@@@@@@@@@@@@@@     - Scaling is applied before translation",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[0],
            env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[1]
        )
    };
}

#[derive(Parser)]
#[command(about = about!(), arg_required_else_help = true, version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Converts between mesh or segmentation file types:
    /// inp -> (exo | mesh | vtk), npy -> spn, spn -> npy
    Convert {
        /// Mesh (inp) or segmentation (npy | spn) input file
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Mesh (exo | mesh | vtk) or segmentation (npy | spn) output file
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of voxels in the x-direction
        #[arg(long, short = 'x', value_name = "NEL")]
        nelx: Option<usize>,

        /// Number of voxels in the y-direction
        #[arg(long, short = 'y', value_name = "NEL")]
        nely: Option<usize>,

        /// Number of voxels in the z-direction
        #[arg(long, short = 'z', value_name = "NEL")]
        nelz: Option<usize>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },

    /// Defeatures and creates a new segmentation: (npy | spn) -> (npy | spn)
    Defeature {
        /// Segmentation input file (npy | spn)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Defeatured segmentation output file (npz | spn)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Defeature clusters with less than MIN voxels
        #[arg(long, short, value_name = "MIN")]
        min: usize,

        /// Number of voxels in the x-direction
        #[arg(long, short = 'x', value_name = "NEL")]
        nelx: Option<usize>,

        /// Number of voxels in the y-direction
        #[arg(long, short = 'y', value_name = "NEL")]
        nely: Option<usize>,

        /// Number of voxels in the z-direction
        #[arg(long, short = 'z', value_name = "NEL")]
        nelz: Option<usize>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },

    /// Creates a finite element mesh from a segmentation:
    /// (npy | spn) -> (exo | inp | mesh | vtk)
    Mesh {
        #[command(subcommand)]
        meshing: Option<MeshingCommands>,

        /// Segmentation input file (npy | spn)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Mesh output file (exo | inp | mesh | vtk)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Defeature clusters with less than NUM voxels
        #[arg(long, short, value_name = "NUM")]
        defeature: Option<usize>,

        /// Number of voxels in the x-direction
        #[arg(long, short = 'x', value_name = "NEL")]
        nelx: Option<usize>,

        /// Number of voxels in the y-direction
        #[arg(long, short = 'y', value_name = "NEL")]
        nely: Option<usize>,

        /// Number of voxels in the z-direction
        #[arg(long, short = 'z', value_name = "NEL")]
        nelz: Option<usize>,

        /// Voxel IDs to remove from the mesh
        #[arg(long, num_args = 1.., short, value_delimiter = ' ', value_name = "ID")]
        remove: Option<Vec<usize>>,

        /// Scaling (> 0.0) in the x-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        xscale: f64,

        /// Scaling (> 0.0) in the y-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        yscale: f64,

        /// Scaling (> 0.0) in the z-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        zscale: f64,

        /// Translation in the x-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        xtranslate: f64,

        /// Translation in the y-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        ytranslate: f64,

        /// Translation in the z-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        ztranslate: f64,

        /// Name of the quality metrics file
        #[arg(long, value_name = "FILE")]
        metrics: Option<String>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,

        /// Pass to mesh using dualization
        // #[arg(action, hide = true, long, short)]
        // #TODO: Ask MRB:
        // Command mesh: Short option names must be unique for each argument,
        // but '-d' is in use by both 'defeature' and 'dual'
        // #[arg(action, long, short = 'u')]
        #[arg(action, hide = true, long, short = 'u')]
        dual: bool,
    },

    /// Quality metrics for an existing finite element mesh: inp -> csv
    Metrics {
        /// Mesh input file (inp)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Quality metrics output file (csv)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },

    /// Creates a balanced octree from a segmentation:
    /// (npy | spn) -> (exo | inp | mesh | vtk)
    #[command(hide = true)]
    Octree {
        /// Segmentation input file (npy | spn)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Octree output file (exo | inp | mesh | vtk)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of voxels in the x-direction
        #[arg(long, short = 'x', value_name = "NEL")]
        nelx: Option<usize>,

        /// Number of voxels in the y-direction
        #[arg(long, short = 'y', value_name = "NEL")]
        nely: Option<usize>,

        /// Number of voxels in the z-direction
        #[arg(long, short = 'z', value_name = "NEL")]
        nelz: Option<usize>,

        /// Voxel IDs to remove from the mesh
        #[arg(long, num_args = 1.., short, value_delimiter = ' ', value_name = "ID")]
        remove: Option<Vec<usize>>,

        /// Scaling (> 0.0) in the x-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        xscale: f64,

        /// Scaling (> 0.0) in the y-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        yscale: f64,

        /// Scaling (> 0.0) in the z-direction
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        zscale: f64,

        /// Translation in the x-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        xtranslate: f64,

        /// Translation in the y-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        ytranslate: f64,

        /// Translation in the z-direction
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "VAL"
        )]
        ztranslate: f64,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,

        /// Pass to apply pairing
        #[arg(action, long, short)]
        pair: bool,

        /// Pass to apply strong balancing
        #[arg(action, long, short)]
        strong: bool,

        /// Pass to fully refine the boundaries
        #[arg(action, long, short)]
        boundaries: bool,
    },

    /// Applies smoothing to an existing finite element mesh:
    /// inp -> (exo | inp | mesh | vtk)
    Smooth {
        /// Pass to enable hierarchical control
        #[arg(action, long, short = 'c')]
        hierarchical: bool,

        /// Mesh input file (inp)
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Smoothed mesh output file (exo | inp | mesh | vtk)
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of smoothing iterations
        #[arg(default_value_t = 10, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Name of the smoothing method [default: Taubin]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Pass-band frequency for Taubin smoothing
        #[arg(default_value_t = 0.1, long, short = 'k', value_name = "FREQ")]
        pass_band: f64,

        /// Scaling parameter for smoothing
        #[arg(default_value_t = 0.6307, long, short, value_name = "SCALE")]
        scale: f64,

        /// Name of the quality metrics file
        #[arg(long, value_name = "FILE")]
        metrics: Option<String>,

        /// Pass to quiet the terminal output
        #[arg(action, long, short)]
        quiet: bool,
    },
}

#[derive(Subcommand)]
enum MeshingCommands {
    /// Applies smoothing to the mesh before output
    Smooth {
        /// Pass to enable hierarchical control
        #[arg(action, long, short = 'c')]
        hierarchical: bool,

        /// Number of smoothing iterations
        #[arg(default_value_t = 10, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Name of the smoothing method [default: Taubin]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Pass-band frequency for Taubin smoothing
        #[arg(default_value_t = 0.1, long, short = 'k', value_name = "FREQ")]
        pass_band: f64,

        /// Scaling parameter for smoothing
        #[arg(default_value_t = 0.6307, long, short, value_name = "SCALE")]
        scale: f64,
    },
}

struct ErrorWrapper {
    message: String,
}

impl std::fmt::Debug for ErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[1;91m{}.\x1b[0m", self.message)
    }
}

impl From<ErrorIO> for ErrorWrapper {
    fn from(error: ErrorIO) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<ErrorNetCDF> for ErrorWrapper {
    fn from(error: ErrorNetCDF) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<ErrorVtk> for ErrorWrapper {
    fn from(error: ErrorVtk) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<ReadNpyError> for ErrorWrapper {
    fn from(error: ReadNpyError) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

impl From<String> for ErrorWrapper {
    fn from(message: String) -> ErrorWrapper {
        ErrorWrapper { message }
    }
}

impl From<&str> for ErrorWrapper {
    fn from(message: &str) -> ErrorWrapper {
        ErrorWrapper {
            message: message.to_string(),
        }
    }
}

impl From<WriteNpyError> for ErrorWrapper {
    fn from(error: WriteNpyError) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

#[allow(clippy::large_enum_variant)]
enum InputTypes {
    Abaqus(HexahedralFiniteElements),
    Npy(Voxels),
    Spn(Voxels),
}

#[allow(clippy::large_enum_variant)]
enum OutputTypes {
    Abaqus(HexahedralFiniteElements),
    Exodus(HexahedralFiniteElements),
    Mesh(HexahedralFiniteElements),
    Npy(Voxels),
    Spn(Voxels),
    Vtk(HexahedralFiniteElements),
}

fn invalid_output(file: &str, extension: Option<&str>) -> Result<(), ErrorWrapper> {
    Ok(Err(format!(
        "Invalid extension .{} from output file {}",
        extension.unwrap_or("UNDEFINED"),
        file
    ))?)
}

fn main() -> Result<(), ErrorWrapper> {
    let time = Instant::now();
    let is_quiet;
    let args = Args::parse();
    let result = match args.command {
        Some(Commands::Convert {
            input,
            output,
            nelx,
            nely,
            nelz,
            quiet,
        }) => {
            is_quiet = quiet;
            convert(input, output, nelx, nely, nelz, quiet)
        }
        Some(Commands::Defeature {
            input,
            output,
            min,
            nelx,
            nely,
            nelz,
            quiet,
        }) => {
            is_quiet = quiet;
            defeature(input, output, min, nelx, nely, nelz, quiet)
        }
        Some(Commands::Mesh {
            meshing,
            input,
            output,
            defeature,
            nelx,
            nely,
            nelz,
            remove,
            xscale,
            yscale,
            zscale,
            xtranslate,
            ytranslate,
            ztranslate,
            metrics,
            quiet,
            dual,
        }) => {
            is_quiet = quiet;
            mesh(
                meshing, input, output, defeature, nelx, nely, nelz, remove, xscale, yscale,
                zscale, xtranslate, ytranslate, ztranslate, metrics, quiet, dual,
            )
        }
        Some(Commands::Metrics {
            input,
            output,
            quiet,
        }) => {
            is_quiet = quiet;
            metrics(input, output, quiet)
        }
        Some(Commands::Octree {
            input,
            output,
            nelx,
            nely,
            nelz,
            remove,
            xscale,
            yscale,
            zscale,
            xtranslate,
            ytranslate,
            ztranslate,
            quiet,
            pair,
            strong,
            boundaries,
        }) => {
            is_quiet = quiet;
            octree(
                input, output, nelx, nely, nelz, remove, xscale, yscale, zscale, xtranslate,
                ytranslate, ztranslate, quiet, pair, strong, boundaries,
            )
        }
        Some(Commands::Smooth {
            input,
            output,
            iterations,
            method,
            hierarchical,
            pass_band,
            scale,
            metrics,
            quiet,
        }) => {
            is_quiet = quiet;
            smooth(
                input,
                output,
                iterations,
                method,
                hierarchical,
                pass_band,
                scale,
                metrics,
                quiet,
            )
        }
        None => return Ok(()),
    };
    if !is_quiet {
        println!("       \x1b[1;98mTotal\x1b[0m {:?}", time.elapsed());
    }
    result
}

fn convert(
    input: String,
    output: String,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match read_input(&input, nelx, nely, nelz, quiet)? {
        InputTypes::Abaqus(finite_elements) => match output_extension {
            Some("exo") => write_output(output, OutputTypes::Exodus(finite_elements), quiet),
            Some("mesh") => write_output(output, OutputTypes::Mesh(finite_elements), quiet),
            Some("vtk") => write_output(output, OutputTypes::Vtk(finite_elements), quiet),
            _ => invalid_output(&output, output_extension),
        },
        InputTypes::Npy(voxels) | InputTypes::Spn(voxels) => match output_extension {
            Some("spn") => write_output(output, OutputTypes::Spn(voxels), quiet),
            Some("npy") => write_output(output, OutputTypes::Npy(voxels), quiet),
            _ => invalid_output(&output, output_extension),
        },
    }
}

fn defeature(
    input: String,
    output: String,
    min: usize,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match read_input(&input, nelx, nely, nelz, quiet)? {
        InputTypes::Npy(mut voxels) | InputTypes::Spn(mut voxels) => match output_extension {
            Some("npy") => {
                let time = Instant::now();
                if !quiet {
                    println!(" \x1b[1;96mDefeaturing\x1b[0m {}", output);
                }
                voxels = voxels.defeature(min);
                if !quiet {
                    println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
                }
                write_output(output, OutputTypes::Npy(voxels), quiet)
            }
            Some("spn") => {
                let time = Instant::now();
                if !quiet {
                    println!(" \x1b[1;96mDefeaturing\x1b[0m {}", output);
                }
                voxels = voxels.defeature(min);
                if !quiet {
                    println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
                }
                write_output(output, OutputTypes::Spn(voxels), quiet)
            }
            _ => invalid_output(&output, output_extension),
        },
        _ => {
            let input_extension = Path::new(&input).extension().and_then(|ext| ext.to_str());
            Err(format!(
                "Invalid extension .{} from input file {}",
                input_extension.unwrap_or("UNDEFINED"),
                input
            ))?
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn mesh(
    meshing: Option<MeshingCommands>,
    input: String,
    output: String,
    defeature: Option<usize>,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    remove: Option<Vec<usize>>,
    xscale: f64,
    yscale: f64,
    zscale: f64,
    xtranslate: f64,
    ytranslate: f64,
    ztranslate: f64,
    metrics: Option<String>,
    quiet: bool,
    dual: bool,
) -> Result<(), ErrorWrapper> {
    let remove = remove.map(|removed_blocks| {
        removed_blocks
            .into_iter()
            .map(|entry| entry as u8)
            .collect()
    });
    let mut input_type = match read_input(&input, nelx, nely, nelz, quiet)? {
        InputTypes::Npy(voxels) => voxels,
        InputTypes::Spn(voxels) => voxels,
        _ => {
            let input_extension = Path::new(&input).extension().and_then(|ext| ext.to_str());
            Err(format!(
                "Invalid extension .{} from input file {}",
                input_extension.unwrap_or("UNDEFINED"),
                input
            ))?
        }
    };
    if let Some(min_num_voxels) = defeature {
        let time = Instant::now();
        if !quiet {
            println!(" \x1b[1;96mDefeaturing\x1b[0m {}", output);
        }
        input_type = input_type.defeature(min_num_voxels);
        if !quiet {
            println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
        }
    }
    let time = Instant::now();
    let scale = Scale::from([xscale, yscale, zscale]);
    if !quiet {
        let entirely_default = scale.x() == &1.0
            && yscale == 1.0
            && zscale == 1.0
            && xtranslate == 0.0
            && ytranslate == 0.0
            && ztranslate == 0.0;
        print!("     \x1b[1;96mMeshing\x1b[0m {}", output);
        if !entirely_default {
            print!(" [");
        }
        if xscale != 1.0 {
            print!("xscale: {}, ", scale.x());
        }
        if yscale != 1.0 {
            print!("yscale: {}, ", scale.y());
        }
        if zscale != 1.0 {
            print!("zscale: {}, ", scale.z());
        }
        if xtranslate != 0.0 {
            print!("xtranslate: {}, ", xtranslate);
        }
        if ytranslate != 0.0 {
            print!("ytranslate: {}, ", ytranslate);
        }
        if ztranslate != 0.0 {
            print!("ztranslate: {}, ", ztranslate);
        }
        if !entirely_default {
            print!("\x1b[2D]");
        }
        println!();
    }
    let mut output_type = if dual {
        let (_, mut tree) = Octree::from_voxels(input_type);
        tree.balance(true);
        tree.pair();
        tree.into_finite_elements(
            remove,
            scale,
            &Vector::new([xtranslate, ytranslate, ztranslate]),
        )?
    } else {
        input_type.into_finite_elements(
            remove,
            scale,
            &Vector::new([xtranslate, ytranslate, ztranslate]),
        )?
    };
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    if let Some(options) = meshing {
        match options {
            MeshingCommands::Smooth {
                iterations,
                method,
                hierarchical,
                pass_band,
                scale,
            } => {
                apply_smoothing_method(
                    &mut output_type,
                    &output,
                    iterations,
                    method,
                    hierarchical,
                    pass_band,
                    scale,
                    quiet,
                )?;
            }
        }
    }
    if let Some(file) = metrics {
        metrics_inner(&output_type, file, quiet)?
    }
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match output_extension {
        Some("exo") => write_output(output, OutputTypes::Exodus(output_type), quiet)?,
        Some("inp") => write_output(output, OutputTypes::Abaqus(output_type), quiet)?,
        Some("mesh") => write_output(output, OutputTypes::Mesh(output_type), quiet)?,
        Some("vtk") => write_output(output, OutputTypes::Vtk(output_type), quiet)?,
        _ => invalid_output(&output, output_extension)?,
    }
    Ok(())
}

fn metrics(input: String, output: String, quiet: bool) -> Result<(), ErrorWrapper> {
    let output_type = match read_input(&input, None, None, None, quiet)? {
        InputTypes::Abaqus(finite_elements) => finite_elements,
        InputTypes::Npy(_) | InputTypes::Spn(_) => {
            Err(format!("No metrics for segmentation file {}", input))?
        }
    };
    metrics_inner(&output_type, output, quiet)
}

fn metrics_inner(
    fem: &HexahedralFiniteElements,
    output: String,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let time = Instant::now();
    if !quiet {
        println!("     \x1b[1;96mMetrics\x1b[0m {}", output);
    }
    fem.write_metrics(&output)?;
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn octree(
    input: String,
    output: String,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    remove: Option<Vec<usize>>,
    xscale: f64,
    yscale: f64,
    zscale: f64,
    xtranslate: f64,
    ytranslate: f64,
    ztranslate: f64,
    quiet: bool,
    pair: bool,
    strong: bool,
    boundaries: bool,
) -> Result<(), ErrorWrapper> {
    let remove = remove.map(|removed_blocks| {
        removed_blocks
            .into_iter()
            .map(|entry| entry as u8)
            .collect()
    });
    let input_type = match read_input(&input, nelx, nely, nelz, quiet)? {
        InputTypes::Npy(voxels) => voxels,
        InputTypes::Spn(voxels) => voxels,
        _ => {
            let input_extension = Path::new(&input).extension().and_then(|ext| ext.to_str());
            Err(format!(
                "Invalid extension .{} from input file {}",
                input_extension.unwrap_or("UNDEFINED"),
                input
            ))?
        }
    };
    let time = Instant::now();
    if !quiet {
        println!("     \x1b[1;96mMeshing\x1b[0m {}", output);
    }
    let (_, mut tree) = Octree::from_voxels(input_type);
    tree.balance(strong);
    if boundaries {
        tree.boundaries();
    }
    if pair {
        tree.pair();
    }
    tree.prune();
    let output_type = tree.octree_into_finite_elements(
        remove,
        [xscale, yscale, zscale].into(),
        &Vector::new([xtranslate, ytranslate, ztranslate]),
    )?;
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match output_extension {
        Some("exo") => write_output(output, OutputTypes::Exodus(output_type), quiet)?,
        Some("inp") => write_output(output, OutputTypes::Abaqus(output_type), quiet)?,
        Some("mesh") => write_output(output, OutputTypes::Mesh(output_type), quiet)?,
        Some("vtk") => write_output(output, OutputTypes::Vtk(output_type), quiet)?,
        _ => invalid_output(&output, output_extension)?,
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn smooth(
    input: String,
    output: String,
    iterations: usize,
    method: Option<String>,
    hierarchical: bool,
    pass_band: f64,
    scale: f64,
    metrics: Option<String>,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let mut output_type = match read_input(&input, None, None, None, quiet)? {
        InputTypes::Abaqus(finite_elements) => finite_elements,
        InputTypes::Npy(_) | InputTypes::Spn(_) => {
            Err(format!("No smoothing for segmentation file {}", input))?
        }
    };
    apply_smoothing_method(
        &mut output_type,
        &output,
        iterations,
        method,
        hierarchical,
        pass_band,
        scale,
        quiet,
    )?;
    if let Some(file) = metrics {
        metrics_inner(&output_type, file, quiet)?
    }
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match output_extension {
        Some("exo") => write_output(output, OutputTypes::Exodus(output_type), quiet),
        Some("inp") => write_output(output, OutputTypes::Abaqus(output_type), quiet),
        Some("mesh") => write_output(output, OutputTypes::Mesh(output_type), quiet),
        Some("vtk") => write_output(output, OutputTypes::Vtk(output_type), quiet),
        _ => invalid_output(&output, output_extension),
    }
}

#[allow(clippy::too_many_arguments)]
fn apply_smoothing_method(
    output_type: &mut HexahedralFiniteElements,
    output: &str,
    iterations: usize,
    method: Option<String>,
    hierarchical: bool,
    pass_band: f64,
    scale: f64,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let time_smooth = Instant::now();
    let smoothing_method = method.unwrap_or("Taubin".to_string());
    if matches!(
        smoothing_method.as_str(),
        "Gauss"
            | "gauss"
            | "Gaussian"
            | "gaussian"
            | "Laplacian"
            | "Laplace"
            | "laplacian"
            | "laplace"
            | "Taubin"
            | "taubin"
    ) {
        if !quiet {
            println!("   \x1b[1;96mSmoothing\x1b[0m {}", output);
        }
        output_type.node_element_connectivity()?;
        output_type.node_node_connectivity()?;
        if hierarchical {
            output_type.nodal_hierarchy()?;
        }
        output_type.nodal_influencers();
        match smoothing_method.as_str() {
            "Gauss" | "gauss" | "Gaussian" | "gaussian" | "Laplacian" | "Laplace" | "laplacian"
            | "laplace" => {
                output_type.smooth(Smoothing::Laplacian(iterations, scale))?;
            }
            "Taubin" | "taubin" => {
                output_type.smooth(Smoothing::Taubin(iterations, pass_band, scale))?;
            }
            _ => panic!(),
        }
        if !quiet {
            println!("        \x1b[1;92mDone\x1b[0m {:?}", time_smooth.elapsed());
        }
        Ok(())
    } else {
        Err(format!(
            "Invalid smoothing method {} specified",
            smoothing_method
        ))?
    }
}

fn read_input(
    input: &str,
    nelx: Option<usize>,
    nely: Option<usize>,
    nelz: Option<usize>,
    quiet: bool,
) -> Result<InputTypes, ErrorWrapper> {
    let time = Instant::now();
    if !quiet {
        println!(
            "\x1b[1m    {} {}\x1b[0m",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
        print!("     \x1b[1;96mReading\x1b[0m {}", input);
    }
    let input_extension = Path::new(&input).extension().and_then(|ext| ext.to_str());
    let result = match input_extension {
        Some("inp") => {
            if !quiet {
                println!();
            }
            InputTypes::Abaqus(HexahedralFiniteElements::from_inp(input)?)
        }
        Some("npy") => {
            if !quiet {
                println!();
            }
            InputTypes::Npy(Voxels::from_npy(input)?)
        }
        Some("spn") => {
            if nelx.is_none() {
                Err("Argument nelx was required but was not provided")?
            } else if nely.is_none() {
                Err("Argument nely was required but was not provided")?
            } else if nelz.is_none() {
                Err("Argument nelz was required but was not provided")?
            } else {
                let nel = Nel::from([nelx.unwrap(), nely.unwrap(), nelz.unwrap()]);
                if !quiet {
                    println!(" [nelx: {}, nely: {}, nelz: {}]", nel.x(), nel.y(), nel.z(),);
                }
                InputTypes::Spn(Voxels::from_spn(input, nel)?)
            }
        }
        _ => {
            if !quiet {
                println!();
            }
            Err(format!(
                "Invalid extension .{} from input file {}",
                input_extension.unwrap_or("UNDEFINED"),
                input
            ))?
        }
    };
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    Ok(result)
}

fn write_output(output: String, output_type: OutputTypes, quiet: bool) -> Result<(), ErrorWrapper> {
    let time = Instant::now();
    if !quiet {
        println!("     \x1b[1;96mWriting\x1b[0m {}", output);
    }
    match output_type {
        OutputTypes::Abaqus(fem) => fem.write_inp(&output)?,
        OutputTypes::Exodus(fem) => fem.write_exo(&output)?,
        OutputTypes::Mesh(fem) => fem.write_mesh(&output)?,
        OutputTypes::Npy(voxels) => voxels.write_npy(&output)?,
        OutputTypes::Spn(voxels) => voxels.write_spn(&output)?,
        OutputTypes::Vtk(fem) => fem.write_vtk(&output)?,
    }
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    Ok(())
}
