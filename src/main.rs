use automesh::{Abaqus, FiniteElements, Smoothing, Voxels};
use clap::{Parser, Subcommand};
use ndarray_npy::{ReadNpyError, WriteNpyError};
use std::{io::Error, path::Path, time::Instant};

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
    /// Converts between segmentation input file types
    Convert {
        /// Name of the original NumPy (.npy) or SPN (.spn) input file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the converted NumPy (.npy) or SPN (.spn) input file.
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of voxels in the x-direction.
        #[arg(short = 'x', long, default_value_t = 0, value_name = "NEL")]
        nelx: usize,

        /// Number of voxels in the y-direction.
        #[arg(short = 'y', long, default_value_t = 0, value_name = "NEL")]
        nely: usize,

        /// Number of voxels in the z-direction.
        #[arg(short = 'z', long, default_value_t = 0, value_name = "NEL")]
        nelz: usize,

        /// Pass to quiet the output.
        #[arg(action, long, short)]
        quiet: bool,
    },

    /// Creates a finite element mesh from a segmentation
    Mesh {
        #[command(subcommand)]
        meshing: Option<MeshingCommands>,

        /// Name of the NumPy (.npy) or SPN (.spn) input file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the Abaqus (.inp) output file.
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of voxels in the x-direction.
        #[arg(default_value_t = 0, long, short = 'x', value_name = "NEL")]
        nelx: usize,

        /// Number of voxels in the y-direction.
        #[arg(default_value_t = 0, long, short = 'y', value_name = "NEL")]
        nely: usize,

        /// Number of voxels in the z-direction.
        #[arg(default_value_t = 0, long, short = 'z', value_name = "NEL")]
        nelz: usize,

        /// Voxel IDs to remove from the mesh [default: 0].
        #[arg(long, short, value_name = "ID")]
        remove: Option<Vec<u8>>,

        /// Scaling (> 0.0) in the x-direction.
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        xscale: f64,

        /// Scaling (> 0.0) in the y-direction.
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        yscale: f64,

        /// Scaling (> 0.0) in the z-direction.
        #[arg(default_value_t = 1.0, long, value_name = "SCALE")]
        zscale: f64,

        /// Translation in the x-direction.
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "TRANSLATE"
        )]
        xtranslate: f64,

        /// Translation in the y-direction.
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "TRANSLATE"
        )]
        ytranslate: f64,

        /// Translation in the z-direction.
        #[arg(
            long,
            default_value_t = 0.0,
            allow_negative_numbers = true,
            value_name = "TRANSLATE"
        )]
        ztranslate: f64,

        /// Pass to quiet the output.
        #[arg(action, long, short)]
        quiet: bool,
    },

    /// Applies smoothing to an existing mesh file
    Smooth {
        /// Name of the Abaqus (.inp) input file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the Abaqus (.inp) output file.
        #[arg(long, short, value_name = "FILE")]
        output: String,

        /// Number of smoothing iterations
        #[arg(default_value_t = 1, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Name of the smoothing method [default: Laplacian]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Scaling (> 0.0) parameter for smoothing
        #[arg(default_value_t = 1.0, long, short, value_name = "SCALE")]
        scale: f64,
    },
}

#[derive(Subcommand)]
enum MeshingCommands {
    /// Applies smoothing to the mesh before output
    Smooth {
        /// Number of smoothing iterations
        #[arg(default_value_t = 1, long, short = 'n', value_name = "NUM")]
        iterations: usize,

        /// Name of the smoothing method [default: Laplacian]
        #[arg(long, short, value_name = "NAME")]
        method: Option<String>,

        /// Scaling (> 0.0) parameter for smoothing
        #[arg(default_value_t = 1.0, long, short, value_name = "SCALE")]
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

impl From<Error> for ErrorWrapper {
    fn from(error: Error) -> ErrorWrapper {
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

impl From<WriteNpyError> for ErrorWrapper {
    fn from(error: WriteNpyError) -> ErrorWrapper {
        ErrorWrapper {
            message: error.to_string(),
        }
    }
}

enum OutputTypes {
    Abaqus(FiniteElements),
    Npy(Voxels),
    Spn(Voxels),
}

fn main() -> Result<(), ErrorWrapper> {
    let args = Args::parse();
    match args.command {
        Some(Commands::Convert {
            input,
            output,
            nelx,
            nely,
            nelz,
            quiet,
        }) => convert(input, output, nelx, nely, nelz, quiet),
        Some(Commands::Mesh {
            meshing,
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
        }) => mesh(
            meshing, input, output, nelx, nely, nelz, remove, xscale, yscale, zscale, xtranslate,
            ytranslate, ztranslate, quiet,
        ),
        Some(Commands::Smooth {
            input,
            output,
            iterations,
            method,
            scale,
        }) => {
            todo!(
                "{}, {}, {}, {:?}, {}",
                input,
                output,
                iterations,
                method,
                scale
            )
        }
        None => Ok(()),
    }
}

fn convert(
    input: String,
    output: String,
    nelx: usize,
    nely: usize,
    nelz: usize,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let input_extension = Path::new(&input).extension().and_then(|ext| ext.to_str());
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    let input_type = read_input(&input, nelx, nely, nelz, quiet)?;
    match (input_extension, output_extension) {
        (Some("npy"), Some("spn")) => write_output(output, OutputTypes::Spn(input_type), quiet)?,
        (Some("spn"), Some("npy")) => write_output(output, OutputTypes::Npy(input_type), quiet)?,
        _ => Err(format!(
            "Invalid extensions .{} and .{} from input and output files {} and {}",
            input_extension.unwrap(),
            output_extension.unwrap(),
            input,
            output
        ))?,
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn mesh(
    meshing: Option<MeshingCommands>,
    input: String,
    output: String,
    nelx: usize,
    nely: usize,
    nelz: usize,
    remove: Option<Vec<u8>>,
    xscale: f64,
    yscale: f64,
    zscale: f64,
    xtranslate: f64,
    ytranslate: f64,
    ztranslate: f64,
    quiet: bool,
) -> Result<(), ErrorWrapper> {
    let time = Instant::now();
    let input_type = read_input(&input, nelx, nely, nelz, quiet)?;
    if !quiet {
        let entirely_default = xscale == 1.0
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
            print!("xscale: {}, ", xscale);
        }
        if yscale != 1.0 {
            print!("yscale: {}, ", yscale);
        }
        if zscale != 1.0 {
            print!("zscale: {}, ", zscale);
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
    let mut output_type = input_type.into_finite_elements(
        remove,
        &[xscale, yscale, zscale],
        &[xtranslate, ytranslate, ztranslate],
    )?;
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    if let Some(options) = meshing {
        match options {
            MeshingCommands::Smooth {
                iterations,
                method,
                scale,
            } => {
                let smoothing_method = method.unwrap_or("Laplacian".to_string());
                match smoothing_method.as_str() {
                    "Laplacian" | "Laplace" | "laplacian" | "laplace" => {
                        let time_smooth = Instant::now();
                        if !quiet {
                            println!("   \x1b[1;96mSmoothing\x1b[0m {}", output);
                        }
                        output_type.calculate_node_element_connectivity()?;
                        output_type.calculate_node_node_connectivity()?;
                        // Unless a hierarchical smoothing method is specified, no need to do:
                        // - calculate_nodal_hierarchy()
                        // - calculate_node_node_connectivity_boundary()
                        // - calculate_node_node_connectivity_interior()
                        output_type.smooth(Smoothing::Laplacian(iterations, scale))?;
                        if !quiet {
                            println!("        \x1b[1;92mDone\x1b[0m {:?}", time_smooth.elapsed());
                        }
                    }
                    _ => Err(format!(
                        "Invalid smoothing method {} specified",
                        smoothing_method
                    ))?,
                }
            }
        }
    }
    write_output(output, OutputTypes::Abaqus(output_type), quiet)?;
    Ok(())
}

fn read_input(
    input: &str,
    nelx: usize,
    nely: usize,
    nelz: usize,
    quiet: bool,
) -> Result<Voxels, ErrorWrapper> {
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
        Some("npy") => {
            if !quiet {
                println!();
            }
            Voxels::from_npy(input)?
        }
        Some("spn") => {
            if !quiet {
                println!(" [nelx: {}, nely: {}, nelz: {}]", nelx, nely, nelz);
            }
            Voxels::from_spn(input, [nelx, nely, nelz])?
        }
        _ => {
            if !quiet {
                println!();
            }
            Err(format!(
                "Invalid extension .{} from input file {}",
                input_extension.unwrap(),
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
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match output_extension {
        Some("inp") => match output_type {
            OutputTypes::Abaqus(fem) => fem.write_inp(&output)?,
            _ => panic!(),
        },
        Some("npy") => match output_type {
            OutputTypes::Npy(voxels) => voxels.write_npy(&output)?,
            _ => panic!(),
        },
        Some("spn") => match output_type {
            OutputTypes::Spn(voxels) => voxels.write_spn(&output)?,
            _ => panic!(),
        },
        _ => Err(format!(
            "Invalid extension .{} from output file {}",
            output_extension.unwrap(),
            output
        ))?,
    }
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
    }
    Ok(())
}
