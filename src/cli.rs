use clap::{Args, Parser, Subcommand};

/// A program to install git pre-commit ant pre-push hooks in a 
/// cross platform supported way.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct HuskyArgs {

    #[command(subcommand)]
    command: Commands,
    
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Installs the husky hooks
    Install(InstallArgs)
}

/// Installs the husky hooks
#[derive(Args, Debug)]
struct InstallArgs {
    #[arg(short, long, default_value_t = String::from(".husky"))]
    directory: String
}
