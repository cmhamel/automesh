use automesh::{Abaqus, Voxels};
use clap::Parser;
use ndarray_npy::ReadNpyError;
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
    /// Name of the NumPy (.npy) or SPN (.spn) input file.
    #[arg(short, long, value_name = "FILE")]
    input: String,

    /// Name of the Abaqus (.inp) output file.
    #[arg(short, long, value_name = "FILE")]
    output: String,

    /// Voxel IDs to remove from the mesh [default: 0].
    #[arg(short = 'r', long, value_name = "ID")]
    remove: Option<Vec<u8>>,

    /// Number of voxels in the x-direction.
    #[arg(short = 'x', long, default_value_t = 0, value_name = "NEL")]
    nelx: usize,

    /// Number of voxels in the y-direction.
    #[arg(short = 'y', long, default_value_t = 0, value_name = "NEL")]
    nely: usize,

    /// Number of voxels in the z-direction.
    #[arg(short = 'z', long, default_value_t = 0, value_name = "NEL")]
    nelz: usize,

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

    /// Pass to quiet the output.
    #[arg(short, long, action)]
    quiet: bool,
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

fn validate(args: &Args) -> Result<(), String> {
    assert!(args.xscale > 0.0, "Need to specify xscale > 0.0");
    assert!(args.yscale > 0.0, "Need to specify yscale > 0.0");
    assert!(args.zscale > 0.0, "Need to specify zscale > 0.0");
    let input_path = Path::new(&args.input);
    let extension = input_path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("npy") => {}
        Some("spn") => {
            assert!(args.nelx >= 1, "Need to specify nelx > 0");
            assert!(args.nely >= 1, "Need to specify nely > 0");
            assert!(args.nelz >= 1, "Need to specify nelz > 0");
        }
        _ => Err("Input must be of type .npy or .spn.".to_string())?,
    }
    let output_path = Path::new(&args.output);
    let extension = output_path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("inp") => Ok(()),
        _ => Err("Output must be of type .inp.".to_string()),
    }
}

fn main() -> Result<(), ErrorWrapper> {
    let time_0 = Instant::now();
    let args = Args::parse();
    if !args.quiet {
        println!(
            "\x1b[1m    {} {}\x1b[0m",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }
    validate(&args)?;
    if !args.quiet {
        print!("     \x1b[1;96mReading\x1b[0m {}", args.input);
    }
    let input = match Path::new(&args.input)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("npy") => {
            if !args.quiet {
                println!();
            }
            Voxels::from_npy(&args.input)?
        }
        Some("spn") => {
            if !args.quiet {
                println!(
                    " [nelx: {}, nely: {}, nelz: {}]",
                    args.nelx, args.nely, args.nelz
                );
            }
            Voxels::from_spn(&args.input, [args.nelx, args.nely, args.nelz])?
        }
        _ => panic!("unreachable since validate() checks"),
    };
    if !args.quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time_0.elapsed());
        let entirely_default = args.xscale == 1.0
            && args.yscale == 1.0
            && args.zscale == 1.0
            && args.xtranslate == 0.0
            && args.ytranslate == 0.0
            && args.ztranslate == 0.0;
        print!("     \x1b[1;96mMeshing\x1b[0m {}", args.output);
        if !entirely_default {
            print!(" [");
        }
        if args.xscale != 1.0 {
            print!("xscale: {}, ", args.xscale);
        }
        if args.yscale != 1.0 {
            print!("yscale: {}, ", args.yscale);
        }
        if args.zscale != 1.0 {
            print!("zscale: {}, ", args.zscale);
        }
        if args.xtranslate != 0.0 {
            print!("xtranslate: {}, ", args.xtranslate);
        }
        if args.ytranslate != 0.0 {
            print!("ytranslate: {}, ", args.ytranslate);
        }
        if args.ztranslate != 0.0 {
            print!("ztranslate: {}, ", args.ztranslate);
        }
        if !entirely_default {
            print!("\x1b[2D]");
        }
        println!();
    }
    let time_1 = Instant::now();
    let fea = input.into_finite_elements(
        args.remove,
        &[args.xscale, args.yscale, args.zscale],
        &[args.xtranslate, args.ytranslate, args.ztranslate],
    );
    match Path::new(&args.output)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("inp") => {
            fea.write_inp(&args.output)?;
        }
        _ => panic!("unreachable since validate() checks"),
    };
    if !args.quiet {
        println!("        \x1b[1;92mDone\x1b[0m {:?}", time_1.elapsed());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_args() -> Args {
        Args {
            input: "foo.spn".to_string(),
            output: "bar.inp".to_string(),
            quiet: false,
            remove: None,
            nelx: 1,
            nely: 1,
            nelz: 1,
            xscale: 1.0,
            yscale: 1.0,
            zscale: 1.0,
            xtranslate: 0.0,
            ytranslate: 0.0,
            ztranslate: 0.0,
        }
    }

    #[test]
    #[should_panic(expected = "Need to specify xscale > 0.0")]
    fn test_xscale_zero() {
        let default_args = default_args();
        let args_bad = Args {
            xscale: 0.0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Need to specify yscale > 0.0")]
    fn test_yscale_zero() {
        let default_args = default_args();
        let args_bad = Args {
            yscale: 0.0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Need to specify zscale > 0.0")]
    fn test_zscale_zero() {
        let default_args = default_args();
        let args_bad = Args {
            zscale: 0.0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Need to specify nelx > 0")]
    fn test_nelx_zero() {
        let default_args = default_args();
        let args_bad = Args {
            nelx: 0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Need to specify nely > 0")]
    fn test_nely_zero() {
        let default_args = default_args();
        let args_bad = Args {
            nely: 0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Need to specify nelz > 0")]
    fn test_nelz_zero() {
        let default_args = default_args();
        let args_bad = Args {
            nelz: 0,
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Input must be of type .npy or .spn")]
    fn test_input_not_npy_or_spn() {
        let default_args = default_args();
        let args_bad = Args {
            input: "bad_extension.bad".to_string(),
            ..default_args
        };
        validate(&args_bad);
    }

    #[test]
    #[should_panic(expected = "Output must be of type .inp")]
    fn test_output_not_inp() {
        let default_args = default_args();
        let args_bad = Args {
            output: "bad_extension.bad".to_string(),
            ..default_args
        };
        validate(&args_bad);
    }
}
