use automesh::{FiniteElements, Smoothing, Voxels};
use clap::{Parser, Subcommand};
use ndarray_npy::{ReadNpyError, WriteNpyError};
use netcdf::Error as ErrorNetCDF;
use std::{io::Error as ErrorIO, path::Path, time::Instant};

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
        /// Name of the original NumPy (.npy) or SPN (.spn) file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the converted NumPy (.npy) or SPN (.spn) file.
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

        /// Name of the NumPy (.npy) or SPN (.spn) file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the Abaqus (.inp) or Exodus (.exo) file.
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
        /// Pass to enable hierarchical control
        #[arg(action, long, short = 'c')]
        hierarchical: bool,

        /// Name of the Abaqus (.inp) or Exodus (.exo) file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the Abaqus (.inp) or Exodus (.exo) file.
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

        /// Pass to quiet the output.
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
enum OutputTypes {
    Abaqus(FiniteElements),
    Exodus(FiniteElements),
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
            hierarchical,
            pass_band,
            scale,
            quiet,
        }) => {
            let time = Instant::now();
            if !quiet {
                println!(
                    "\x1b[1m    {} {}\x1b[0m",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                );
                println!("     \x1b[1;96mReading\x1b[0m {}", input);
            }
            let mut output_type = FiniteElements::from_inp(&input)?;
            if !quiet {
                println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
            }
            let time_smooth = Instant::now();
            let (smoothing_method_is_valid, smoothing_method) = check_smoothing_method(method)?;
            if smoothing_method_is_valid {
                if !quiet {
                    println!("   \x1b[1;96mSmoothing\x1b[0m {}", output);
                }
                output_type.calculate_node_element_connectivity()?;
                output_type.calculate_node_node_connectivity()?;
                if hierarchical {
                    output_type.calculate_nodal_hierarchy()?;
                }
                output_type.calculate_nodal_influencers();
                match smoothing_method.as_str() {
                    "Laplace" => {
                        output_type.smooth(Smoothing::Laplacian(iterations, scale))?;
                    }
                    "Taubin" => {
                        output_type.smooth(Smoothing::Taubin(iterations, pass_band, scale))?;
                    }
                    _ => panic!(),
                }
                if !quiet {
                    println!("        \x1b[1;92mDone\x1b[0m {:?}", time_smooth.elapsed());
                }
            } else {
                Err(format!(
                    "Invalid smoothing method {} specified",
                    smoothing_method
                ))?;
            }
            let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
            match output_extension {
                Some("exo") => write_output(output, OutputTypes::Exodus(output_type), quiet)?,
                Some("inp") => write_output(output, OutputTypes::Abaqus(output_type), quiet)?,
                _ => Err(format!(
                    "Invalid extension .{} from output file {}",
                    output_extension.unwrap(),
                    output
                ))?,
            }
            Ok(())
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
                hierarchical,
                pass_band,
                scale,
            } => {
                let time_smooth = Instant::now();
                let (smoothing_method_is_valid, smoothing_method) = check_smoothing_method(method)?;
                if smoothing_method_is_valid {
                    if !quiet {
                        println!("   \x1b[1;96mSmoothing\x1b[0m {}", output);
                    }
                    output_type.calculate_node_element_connectivity()?;
                    output_type.calculate_node_node_connectivity()?;
                    if hierarchical {
                        output_type.calculate_nodal_hierarchy()?;
                    }
                    output_type.calculate_nodal_influencers();
                    match smoothing_method.as_str() {
                        "Laplace" => {
                            output_type.smooth(Smoothing::Laplacian(iterations, scale))?;
                        }
                        "Taubin" => {
                            output_type.smooth(Smoothing::Taubin(iterations, pass_band, scale))?;
                        }
                        _ => panic!(),
                    }
                    if !quiet {
                        println!("        \x1b[1;92mDone\x1b[0m {:?}", time_smooth.elapsed());
                    }
                } else {
                    Err(format!(
                        "Invalid smoothing method {} specified",
                        smoothing_method
                    ))?;
                }
            }
        }
    }
    let output_extension = Path::new(&output).extension().and_then(|ext| ext.to_str());
    match output_extension {
        Some("exo") => write_output(output, OutputTypes::Exodus(output_type), quiet)?,
        Some("inp") => write_output(output, OutputTypes::Abaqus(output_type), quiet)?,
        _ => Err(format!(
            "Invalid extension .{} from output file {}",
            output_extension.unwrap(),
            output
        ))?,
    }
    Ok(())
}

fn check_smoothing_method(method: Option<String>) -> Result<(bool, String), ErrorWrapper> {
    let smoothing_method = method.unwrap_or("Taubin".to_string());
    let valid = matches!(
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
    );
    match smoothing_method.as_str() {
        "Gauss" | "gauss" | "Gaussian" | "gaussian" | "Laplacian" | "Laplace" | "laplacian"
        | "laplace" => Ok((valid, "Laplace".to_string())),
        "Taubin" | "taubin" => Ok((valid, "Taubin".to_string())),
        _ => Ok(Err(format!(
            "Invalid smoothing method {} specified",
            smoothing_method
        ))?),
    }
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
        Some("exo") => match output_type {
            OutputTypes::Exodus(fem) => fem.write_exo(&output)?,
            _ => panic!(),
        },
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
