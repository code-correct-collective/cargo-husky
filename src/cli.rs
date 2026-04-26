use clap::{Args, Parser, Subcommand};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

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
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// The default directory to install the git hooks.
    #[arg(short, long, default_value_t = String::from(".husky"))]
    directory: String
}

impl InstallArgs {
    pub fn execute(&self) {
        let hook = Assets::get("hook").unwrap();
        
        println!("the script is a {:?}", std::str::from_utf8(hook.data.as_ref()));
        todo!("create the function to initialize github");
    }
}
