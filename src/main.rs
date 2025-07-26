// src/main.rs
/*
 * Main executable for UltraCyber
 */

use clap::Parser;
use ultracyber::{Result, run};

#[derive(Parser)]
#[command(version, about = "UltraCyber - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
