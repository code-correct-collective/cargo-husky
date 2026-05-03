use clap::{Args, Parser, Subcommand};

/// A program to install git pre-commit ant pre-push hooks in a
/// cross platform supported way.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct HuskyArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Installs the husky hooks into the git repository.
    Install(InstallArgs),
    /// Uninstalls the husky hooks from the git repository.
    Uninstall,
    /// Creates or appends a git hook (set pre-commit -c "echo cargo husky is awesome!")
    Set(SetArgs),
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// The default directory to install the git hooks.
    #[arg(short, long, default_value_t = String::from(".husky"))]
    pub directory: String,
}

#[derive(Args, Debug)]
pub struct SetArgs {
    /// The git hook to add (pre-commit, pre-push, etc) 
    pub hook: String,

    #[arg(short, long, default_value_t = String::from(""))]
    pub command: String,
}


