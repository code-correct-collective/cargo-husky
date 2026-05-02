use clap::Parser;

use c3_cargo_husky::cli;
use c3_cargo_husky::husky;
use c3_cargo_husky::husky::error;

fn main() -> Result<(), error::HuskyError> {
    let args = cli::HuskyArgs::parse();

    match args.command {
        cli::Commands::Install(ref install_args) => husky::install(&install_args.directory)?,
        cli::Commands::Uninstall => husky::uninstall()?,
    }

    Ok(())
}
