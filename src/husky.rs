use std::io;
use std::fs;
use std::path;

use git2::{Repository, RepositoryOpenFlags};
use rust_embed::Embed;


pub mod error;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

pub fn install(directory: &str) -> Result<(), error::HuskyError> {
    let hook = Assets::get("hook").unwrap();
    let repository = Repository::open_ext(".", RepositoryOpenFlags::empty(), &[] as &[&std::ffi::OsStr])?;
    
    create_install_path(repository.path(), directory)?;

    println!("Add script to {:?} to {directory}", std::str::from_utf8(hook.data.as_ref()).unwrap());

    Ok(())
}

fn create_install_path(git_path: &path::Path, install_dir: &str) -> Result<(), error::HuskyError> {
    let path = git_path
        .parent().ok_or_else(|| std::io::Error::new(io::ErrorKind::Other, ".git has no parent"))?;
        
    let path = path.join(&install_dir).join("_");

    fs::create_dir_all(path)?;

    Ok(())
}
