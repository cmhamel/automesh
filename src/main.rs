use automesh::{Abaqus, FiniteElements, Voxels};
use clap::{Parser, Subcommand};
use ndarray_npy::{ReadNpyError, WriteNpyError};
use std::{io::Error, path::Path, time::Instant};

#[derive(Parser)]
#[command(about = format!("

     @@@@@@@@@@@@@@@@
      @@@@  @@@@@@@@@@
     @@@@  @@@@@@@@@@@
    @@@@  @@@@@@@@@@@@    \x1b[1;4m{}: Automatic mesh generation\x1b[0m
      @@    @@    @@      {}
      @@    @@    @@      {}
    @@@@@@@@@@@@  @@@
    @@@@@@@@@@@  @@@@     \x1b[1;4mNotes:\x1b[0m
    @@@@@@@@@@ @@@@@ @    - Input/output file types are inferred.
     @@@@@@@@@@@@@@@@     - Scaling is applied before translation.",
env!("CARGO_PKG_NAME"),
env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[0],
env!("CARGO_PKG_AUTHORS").split(":").collect::<Vec<&str>>()[1]
), arg_required_else_help = true, long_about = None, version)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Pass to quiet the output.
    #[arg(action, global = true, long, short)]
    quiet: bool,
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
    },

    /// Creates a finite element mesh from a segmentation
    Mesh {
        /// Name of the NumPy (.npy) or SPN (.spn) input file.
        #[arg(long, short, value_name = "FILE")]
        input: String,

        /// Name of the Abaqus (.inp) output file.
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

        /// Voxel IDs to remove from the mesh [default: 0].
        #[arg(short = 'r', long, value_name = "ID")]
        remove: Option<Vec<u8>>,

        /// Scaling (> 0.0) in the x-direction.
        #[arg(long, default_value_t = 1.0, value_name = "SCALE")]
        xscale: f64,

        /// Scaling (> 0.0) in the y-direction.
        #[arg(long, default_value_t = 1.0, value_name = "SCALE")]
        yscale: f64,

        /// Scaling (> 0.0) in the z-direction.
        #[arg(long, default_value_t = 1.0, value_name = "SCALE")]
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
    },

    /// Applies smoothing to an existing mesh file
    Smooth {},
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
        }) => convert(input, output, nelx, nely, nelz, args.quiet),
        Some(Commands::Smooth {}) => {
            todo!()
        }
        Some(Commands::Mesh {
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
        }) => mesh(
            input, output, nelx, nely, nelz, remove, xscale, yscale, zscale, xtranslate,
            ytranslate, ztranslate, args.quiet,
        ),
        None => Ok(Err("Need to specify a command".to_string())?),
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
    // should validate args using fns for each subcommand
    // validate(&args)?;

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
    let time = Instant::now();
    let output_type = input_type.into_finite_elements(
        remove,
        &[xscale, yscale, zscale],
        &[xtranslate, ytranslate, ztranslate],
    );
    if !quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time.elapsed());
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

// fn validate(args: &Args) -> Result<(), String> {
//     assert!(args.xscale > 0.0, "Need to specify xscale > 0.0");
//     assert!(args.yscale > 0.0, "Need to specify yscale > 0.0");
//     assert!(args.zscale > 0.0, "Need to specify zscale > 0.0");
//     let input_path = Path::new(&args.input);
//     let extension = input_path.extension().and_then(|ext| ext.to_str());
//     match extension {
//         Some("npy") => {}
//         Some("spn") => {
//             assert!(args.nelx >= 1, "Need to specify nelx > 0");
//             assert!(args.nely >= 1, "Need to specify nely > 0");
//             assert!(args.nelz >= 1, "Need to specify nelz > 0");
//         }
//         _ => Err("Input must be of type .npy or .spn".to_string())?,
//     }
//     let output_path = Path::new(&args.output);
//     let extension = output_path.extension().and_then(|ext| ext.to_str());
//     match extension {
//         Some("inp") => Ok(()),
//         _ => Err("Output must be of type .inp".to_string()),
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn default_args() -> Args {
//         Args {
//             input: "foo.spn".to_string(),
//             output: "bar.inp".to_string(),
//             quiet: false,
//             remove: None,
//             nelx: 1,
//             nely: 1,
//             nelz: 1,
//             xscale: 1.0,
//             yscale: 1.0,
//             zscale: 1.0,
//             xtranslate: 0.0,
//             ytranslate: 0.0,
//             ztranslate: 0.0,
//         }
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify xscale > 0.0")]
//     fn test_xscale_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             xscale: 0.0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify yscale > 0.0")]
//     fn test_yscale_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             yscale: 0.0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify zscale > 0.0")]
//     fn test_zscale_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             zscale: 0.0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify nelx > 0")]
//     fn test_nelx_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             nelx: 0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify nely > 0")]
//     fn test_nely_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             nely: 0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Need to specify nelz > 0")]
//     fn test_nelz_zero() {
//         let default_args = default_args();
//         let args_bad = Args {
//             nelz: 0,
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Input must be of type .npy or .spn")]
//     fn test_input_not_npy_or_spn() {
//         let default_args = default_args();
//         let args_bad = Args {
//             input: "bad_extension.bad".to_string(),
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }

//     #[test]
//     #[should_panic(expected = "Output must be of type .inp")]
//     fn test_output_not_inp() {
//         let default_args = default_args();
//         let args_bad = Args {
//             output: "bad_extension.bad".to_string(),
//             ..default_args
//         };
//         validate(&args_bad).unwrap();
//     }
// }
