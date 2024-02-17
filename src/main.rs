use triplet_finder::args::Args;
use clap::Parser;

fn main() {
    // handle args
    let args = Args::parse();

    triplet_finder::run(args.threads, args.strict);
}
