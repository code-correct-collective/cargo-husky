use clap::Parser;
use c3_cargo_husky::cli::{Commands, HuskyArgs};

fn main() {
    let args = HuskyArgs::parse();

    dbg!(&args);

    match args.command {
        Commands::Install(ref install) => install.execute(),
        _ => println!("something else")
    };
    
}
