use clap::{Parser, ValueEnum};

/// Application to concurrently search for Pythagorean Triplets
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Specify a custom number of threads (tip: use your core count)
    #[arg(short = 't', long, default_value_t = 8)]
    pub threads: u32,

    /// Make commands case sensitive. Provides a miniscule performance increase.
    #[arg(short = 's', long, default_value_t = false)]
    pub strict: bool,
}