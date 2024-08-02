use automesh::Spn;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the input file.
    #[arg(short, long)]
    input: String,

    /// Name of the output file.
    #[arg(short, long)]
    output: String,

    /// Number of voxels in the x-direction.
    #[arg(long, default_value_t = 0)]
    nelx: usize,

    /// Number of voxels in the y-direction.
    #[arg(long, default_value_t = 0)]
    nely: usize,

    /// Number of voxels in the z-direction.
    #[arg(long, default_value_t = 0)]
    nelz: usize,
}

fn main() {
    let args = Args::parse();
    let input = if args.input.ends_with(".npy") {
        Spn::from_npy(&args.input)
    } else if args.input.ends_with(".spn") {
        if args.nelx < 1 {
            panic!("Need to specify nelx > 0.")
        } else if args.nely < 1 {
            panic!("Need to specify nely > 0.")
        } else if args.nelz < 1 {
            panic!("Need to specify nelz > 0.")
        } else {
            Spn::new(&args.input, args.nelx, args.nely, args.nelz)
        }
    } else {
        panic!("Invalid input ({}) specified.", args.input)
    };
    let output = if args.output.ends_with(".exo") {
        input.into_exodus()
    } else {
        panic!("Invalid output ({}) specified.", args.output)
    };
    output.write(&args.output);
}
