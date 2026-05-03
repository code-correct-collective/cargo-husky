pub mod error;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use git2::{Repository, RepositoryOpenFlags};
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

type HuskyResult<T> = Result<T, error::HuskyError>;
type UnitHuskyResult = HuskyResult<()>;

const DEFAULT_DIRECTORY: &str = ".husky";
const ASSETS_HUSKY_SH: &str = "husky.sh";
const ASSETS_TASK_RUNNER: &str = "task-runner.json";
const ASSETS_HOOK: &str = "hook";

const HOOKS_PATH: &str = "core.hooksPath";

pub fn install(directory: &str) -> UnitHuskyResult {
    writeln!(io::stdout(), "⚡ Installing husky to {}..", &directory)?;
    let repository = open_repository()?;

    let underscore_path = create_install_path(repository.path(), directory)?;
    let mut config = repository.config()?;

    config.set_str(HOOKS_PATH, directory)?;

    let Some(install_path) = underscore_path.parent() else {
        unreachable!()
    };

    let husky_path = write_asset_file(&underscore_path, ASSETS_HUSKY_SH)?;
    write_asset_file(install_path, ASSETS_TASK_RUNNER)?;

    let gitignore_path = underscore_path.join(underscore_path.join(".gitignore"));
    write_file(&gitignore_path, "*")?;

    set_execute_permissions(&husky_path)?;

    writeln!(io::stdout(), "✔️ Husky installed")?;

    Ok(())
}

pub fn uninstall() -> UnitHuskyResult {
    let repository = open_repository()?;
    let mut config = repository.config()?;
    // let directory = config.get_str(HOOKS_PATH).unwrap_or(DEFAULT_DIRECTORY);
    let directory = get_husky_path(&repository)?;

    writeln!(
        io::stdout(),
        "🗑️ Uninstalling husky from {}",
        directory.display()
    )?;

    let Some(git_parent) = repository.path().parent() else {
        unreachable!()
    };

    let path = git_parent.join(directory);

    if !fs::exists(&path)? {
        writeln!(io::stdout(), "⚠️ Husky already removed")?;
        return Ok(()); // nothing to delete
    }

    fs::remove_dir_all(&path)?;
    config.remove(HOOKS_PATH)?;

    writeln!(io::stdout(), "✔️ Husky removed")?;

    Ok(())
}

pub fn set_hook(hook_name: &str, command: &str) -> UnitHuskyResult {
    writeln!(
        io::stdout(),
        "🛠️ Setting the command {} on the {} hook.",
        command,
        hook_name
    )?;

    let repository = open_repository()?;
    let install_path = get_husky_path(&repository)?;

    write_asset_filename(&install_path, hook_name, ASSETS_HOOK)?;

    let hook_path = install_path.join(hook_name);
    set_execute_permissions(&hook_path)?;

    if !command.trim().is_empty() {
        let command = format!("{}\n", command.trim());

        let mut hook_file = OpenOptions::new().append(true).open(hook_path)?;

        hook_file.write_all(command.as_bytes())?;
    }
    writeln!(io::stdout(), "✔️ {} hook updated", hook_name)?;
    Ok(())
}

fn get_husky_path(repository: &Repository) -> HuskyResult<PathBuf> {
    let config = repository.config()?;

    let install_segment = config.get_str(HOOKS_PATH).unwrap_or(DEFAULT_DIRECTORY);

    let Some(path) = repository.path().parent() else {
        unreachable!()
    };

    let path = path.join(install_segment);

    Ok(path)
}

fn write_asset_file(directory: &Path, asset_name: &str) -> HuskyResult<PathBuf> {
    write_asset_filename(directory, asset_name, asset_name)
}

fn write_asset_filename(
    directory: &Path,
    filename: &str,
    asset_name: &str,
) -> HuskyResult<PathBuf> {
    let Some(ref file_data) = Assets::get(asset_name) else {
        unreachable!()
    };

    let file_path = directory.join(filename);

    let content = str::from_utf8(file_data.data.as_ref()).ok().unwrap_or("");

    write_file(&file_path, content)?;

    Ok(file_path)
}

fn write_file(file_path: &Path, content: &str) -> UnitHuskyResult {
    if !file_path.exists() {
        fs::write(file_path, content)?;
    }

    Ok(())
}

fn open_repository() -> HuskyResult<Repository> {
    Ok(Repository::open_ext(
        ".",
        RepositoryOpenFlags::empty(),
        &[] as &[&std::ffi::OsStr],
    )?)
}

fn set_execute_permissions(path: &Path) -> UnitHuskyResult {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(perms.mode() | 0o100);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

fn create_install_path(git_path: &Path, install_dir: &str) -> HuskyResult<PathBuf> {
    let Some(path) = git_path.parent() else {
        unreachable!()
    }; // every .git folder should have a parent.

    let install_path = path.join(install_dir);
    let underscore_path = install_path.join("_");

    fs::create_dir_all(&underscore_path)?;

    Ok(underscore_path)
}
