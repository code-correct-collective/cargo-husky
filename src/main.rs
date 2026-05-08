#![doc = include_str!("../README.md")]

use c3_cargo_husky::husky::filesystem_manager::LocalFilesystem;
use c3_cargo_husky::husky::repository::{GitRepository, HuskyRepository};
use c3_cargo_husky::husky::task_runner::HuskyTaskRunner;
use clap::Parser;

use c3_cargo_husky::cli;
use c3_cargo_husky::husky;
use c3_cargo_husky::husky::error;

fn main() -> Result<(), error::HuskyError> {
    let args = cli::HuskyArgs::parse();

    let repository = GitRepository::open_repository()?;
    let file_manager = LocalFilesystem {};
    let task_runner = HuskyTaskRunner {};

    match args.command {
        cli::Commands::Install(ref args) => {
            husky::install(&args.directory, &repository, &file_manager)
        }
        cli::Commands::Uninstall => husky::uninstall(&repository, &file_manager),
        cli::Commands::Set(ref args) => {
            husky::set_hook(&args.hook, &args.command, &repository, &file_manager)
        }
        cli::Commands::Run(ref args) => husky::run(args, &repository, &file_manager, &task_runner),
        cli::Commands::List => husky::list(&repository, &file_manager),
    }
}
