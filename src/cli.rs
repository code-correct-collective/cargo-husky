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
    /// Run tasks
    Run(RunArgs),
    /// List available tasks
    List,
}

#[derive(Args, Debug)]
/// The arguments for installing the git hooks
pub struct InstallArgs {
    /// Define the hooks in the specified directory.
    #[arg(short, long, default_value_t = String::from(".husky"))]
    pub directory: String,
}

#[derive(Args, Debug)]
/// The arguments to add a git hook and append commands to the scripts
pub struct SetArgs {
    /// The git hook to add (pre-commit, pre-push, etc)
    pub hook: String,

    /// The command to write to the hook
    #[arg(short, long, default_value_t = String::from(""))]
    pub command: String,
}

#[derive(Args, Debug)]
/// The arguments to execute tasks defined by the task runner
pub struct RunArgs {
    /// The name of the task to run
    #[arg(short, long)]
    pub name: Option<String>,

    /// The group name of the tasks to run
    #[arg(short, long)]
    pub group: Option<String>,

    /// Pass custom arguments to tasks
    #[arg(short, long)]
    pub args: Option<Vec<String>>,
}
