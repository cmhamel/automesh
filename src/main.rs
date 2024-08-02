use automesh::Spn;
use clap::Parser;

#[derive(Parser)]
#[command(about, arg_required_else_help = true, long_about = None, version)]
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
