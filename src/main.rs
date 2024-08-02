use automesh::Spn;
use clap::Parser;

/// Automatic hexahedral finite element mesh generation from a voxelized segmentation.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the NumPy input file.  Example: --input <input_file>.npy   (TODO: work in progress <input_file>.spn)
    #[arg(short, long)]
    input: String,

    /// Name of the Exodus output file. Example: --output <output_file>.exo
    #[arg(short, long)]
    output: String,
}

fn main() {
    let args = Args::parse();
    let input = if args.input.ends_with(".npy") {
        Spn::from_npy(&args.input)
    } else {
        panic!("Invalid input {} specified.", args.input)
    };
    let output = if args.output.ends_with(".exo") {
        input.into_exodus()
    } else {
        panic!("Invalid output {} specified.", args.output)
    };
    output.write(&args.output);
}
