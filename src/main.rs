use c3_cargo_husky::husky::repository::{GitRepository, HuskyRepository};
use c3_cargo_husky::husky::utils::LocalFilesystem;
use clap::Parser;

use c3_cargo_husky::cli;
use c3_cargo_husky::husky;
use c3_cargo_husky::husky::error;

fn main() -> Result<(), error::HuskyError> {
    let args = cli::HuskyArgs::parse();

    let repository = GitRepository::open_repository()?;
    let file_manager = LocalFilesystem {};

    match args.command {
        cli::Commands::Install(ref args) => {
            husky::install(&args.directory, &repository, &file_manager)
        }
        cli::Commands::Uninstall => husky::uninstall(&repository), // need file_manager
        cli::Commands::Set(ref args) => {
            husky::set_hook(&args.hook, &args.command, &repository, &file_manager)
        }
        cli::Commands::Run(ref args) => husky::run(args, &repository),
        cli::Commands::List => husky::list(&repository),
    }
}
