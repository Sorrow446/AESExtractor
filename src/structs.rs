use std::path::PathBuf;

use clap::Parser;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "AESExtractor", version = env!("CARGO_PKG_VERSION"))]
pub struct Args {
    #[clap(short, long, help = "Input path of game binary.")]
    pub in_path: Option<PathBuf>,

    #[clap(long, help = "Suppress all other prints and write JSON to stdout. Exit code 0 = OK.")]
    pub json: bool,

    #[clap(long, help = "Disable printing in colour.")]
    pub no_colour: bool,

    #[clap(long, help = "Minimum key entropy float (default: 3).")]
    pub entropy: Option<f64>,

    pub dropped_in_path: Option<PathBuf>,
}

#[derive(Clone, Serialize)]
pub struct KeyResult {
    pub key: String,
    pub entropy: f64,
}