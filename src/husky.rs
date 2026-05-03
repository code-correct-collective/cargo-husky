pub mod error;

use std::fs;
use std::io::{self, Write};
use std::path;

use git2::{Repository, RepositoryOpenFlags};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

const DEFAULT_DIRECTORY: &str = ".husky";
const ASSETS_HUSKY_SH: &str = "husky.sh";
const ASSETS_TASK_RUNNER: &str = "task-runner.json";

const HOOKS_PATH: &str = "core.hooksPath";

pub fn install(directory: &str) -> Result<(), error::HuskyError> {
    writeln!(io::stdout(), "⚡ Installing husky to {}..", &directory)?;
    let repository = open_repository()?;

    create_install_path(repository.path(), directory)?;
    let mut config = repository.config()?;

    config.set_str(HOOKS_PATH, directory)?;

    writeln!(io::stdout(), "✔️ Husky installed")?;

    Ok(())
}

pub fn uninstall() -> Result<(), error::HuskyError> {
    let repository = open_repository()?;
    let mut config = repository.config()?;
    let directory = config.get_str(HOOKS_PATH).unwrap_or(DEFAULT_DIRECTORY);

    let Some(git_parent) = repository.path().parent() else {
        unreachable!()
    };

    let path = git_parent.join(directory);

    writeln!(io::stdout(), "🗑️ Uninstalling husky from {}", &directory)?;

    if !fs::exists(&path)? {
        writeln!(io::stdout(), "⚠️ Husky already removed")?;
        return Ok(()); // nothing to delete
    }

    fs::remove_dir_all(&path)?;
    config.remove(HOOKS_PATH)?;
    writeln!(io::stdout(), "✔️ Husky removed")?;

    Ok(())
}

pub fn open_repository() -> Result<Repository, error::HuskyError> {
    Ok(Repository::open_ext(
        ".",
        RepositoryOpenFlags::empty(),
        &[] as &[&std::ffi::OsStr],
    )?)
}

fn set_execute_permissions(path: &path::Path) -> Result<(), error::HuskyError> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(perms.mode() | 0o100);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

fn create_install_path(git_path: &path::Path, install_dir: &str) -> Result<(), error::HuskyError> {
    let Some(path) = git_path.parent() else {
        unreachable!()
    }; // every .git folder should have a parent.

    let install_path = path.join(install_dir);
    let underscore_path = install_path.join("_");

    fs::create_dir_all(&underscore_path)?;

    // I've created these assets, they should exist.
    let Some(ref file_data) = Assets::get(ASSETS_HUSKY_SH) else {
        unreachable!()
    };

    let Some(ref task_runner_data) = Assets::get(ASSETS_TASK_RUNNER) else {
        unreachable!()
    };

    let husky_path = underscore_path.join(underscore_path.join(ASSETS_HUSKY_SH));
    let gitignore_path = underscore_path.join(underscore_path.join(".gitignore"));
    let task_runner_path = install_path.join(install_path.join(ASSETS_TASK_RUNNER));

    fs::write(&husky_path, &file_data.data)?;
    fs::write(&gitignore_path, "*")?;

    if !task_runner_path.exists() {
        fs::write(task_runner_path, &task_runner_data.data)?;
    }


    set_execute_permissions(&husky_path)?;

    Ok(())
}
