mod cli;

use clap::Parser;
use crate::cli::HuskyArgs;

fn main() {
    let args = HuskyArgs::parse();
    dbg!(args);
}
