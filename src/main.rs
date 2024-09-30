use automesh::{Abaqus, Voxels};
use clap::Parser;
use std::{path::Path, time::Instant};

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
    #[arg(short, long)]
    input: String,

    /// Name of the Abaqus (.inp) output file.
    #[arg(short, long)]
    output: String,

    /// Pass to quiet the output.
    #[arg(short, long, action)]
    quiet: bool,

    /// Voxel IDs to remove from the mesh [default: 0].
    #[arg(short = 'r', long)]
    remove: Option<Vec<u8>>,

    /// Number of voxels in the x-direction.
    #[arg(short = 'x', long, default_value_t = 0)]
    nelx: usize,

    /// Number of voxels in the y-direction.
    #[arg(short = 'y', long, default_value_t = 0)]
    nely: usize,

    /// Number of voxels in the z-direction.
    #[arg(short = 'z', long, default_value_t = 0)]
    nelz: usize,

    /// Scaling in the x-direction.
    #[arg(long, default_value_t = 1.0)]
    xscale: f64,

    /// Scaling in the y-direction.
    #[arg(long, default_value_t = 1.0)]
    yscale: f64,

    /// Scaling in the z-direction.
    #[arg(long, default_value_t = 1.0)]
    zscale: f64,

    /// Translation in the x-direction.
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    xtranslate: f64,

    /// Translation in the y-direction.
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    ytranslate: f64,

    /// Translation in the z-direction.
    #[arg(long, default_value_t = 0.0, allow_negative_numbers = true)]
    ztranslate: f64,
}

fn validate(args: &Args) {
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
        _ => panic!("Input must be of type .npy or .spn"),
    }
    let output_path = Path::new(&args.output);
    let extension = output_path.extension().and_then(|ext| ext.to_str());
    match extension {
        Some("inp") => {}
        _ => panic!("Output must be of type .inp"),
    }
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

fn main() {
    let time = Instant::now();
    let args = Args::parse();
    if !args.quiet {
        println!(
            "\x1b[1m    {} {}\x1b[0m",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        );
    }
    validate(&args);
    let input = match Path::new(&args.input)
        .extension()
        .and_then(|ext| ext.to_str())
    {
        Some("npy") => {
            if !args.quiet {
                println!("     \x1b[1;96mReading\x1b[0m {}", args.input);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
            Voxels::from_npy(&args.input)
        }
        Some("spn") => Voxels::from_spn(&args.input, [args.nelx, args.nely, args.nelz]),
        _ => panic!("Invalid input ({}) specified.", args.input),
    };
    if !args.quiet {
        println!("\x1b[F       \x1b[1;92mInput\x1b[0m {}", args.input);
    }
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
            if !args.quiet {
                println!("     \x1b[1;96mMeshing\x1b[0m {}", args.output);
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
            fea.write_inp(&args.output);
            if !args.quiet {
                println!("\x1b[F      \x1b[1;92mOutput\x1b[0m {}", args.output);
            }
        }
        _ => panic!("Invalid output ({}) specified.", args.output),
    };
    if !args.quiet {
        println!("        \x1b[1;92mTime\x1b[0m {:?}", time.elapsed());
    }
}
