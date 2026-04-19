use clap::Parser;
use c3_cargo_husky::cli::HuskyArgs;

fn main() {
    let args = HuskyArgs::parse();
    dbg!(args);
}
