use clap::Parser;
use triplet_finder::args::Args;

fn main() {
    // handle args
    let args = Args::parse();

    triplet_finder::run(args.threads, args.strict);
}
