// src/main.rs
/*
 * Main executable for NftMarketplaceEngineProject
 */

use clap::Parser;
use nftmarketplaceengineproject::{Result, run};

#[derive(Parser)]
#[command(version, about = "NftMarketplaceEngineProject - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
