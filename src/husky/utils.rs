use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::husky::error::HuskyError;
use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

pub type HuskyResult<T> = Result<T, HuskyError>;
pub type UnitHuskyResult = HuskyResult<()>;

pub const DEFAULT_DIRECTORY: &str = ".husky";
pub const ASSETS_HUSKY_SH: &str = "husky.sh";
pub const ASSETS_TASK_RUNNER: &str = "task-runner.json";
pub const ASSETS_HOOK: &str = "hook";
pub const GIT_CONFIG_HOOKS_PATH: &str = "core.hooksPath";

pub fn write_asset_file(directory: &Path, asset_name: &str) -> HuskyResult<PathBuf> {
    write_asset_filename(directory, asset_name, asset_name)
}

pub fn write_asset_filename(
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

pub fn write_file(file_path: &Path, content: &str) -> UnitHuskyResult {
    if !file_path.exists() {
        fs::write(file_path, content)?;
    }

    Ok(())
}

pub fn set_execute_permissions(path: &Path) -> UnitHuskyResult {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut perms = fs::metadata(path)?.permissions();
        perms.set_mode(perms.mode() | 0o100);
        fs::set_permissions(path, perms)?;
    }

    Ok(())
}

pub fn create_install_path(git_path: &Path, install_dir: &str) -> HuskyResult<PathBuf> {
    dbg!(&git_path);
    dbg!(&install_dir);
    let install_path = git_path.join(install_dir);
    let underscore_path = install_path.join("_");

    dbg!(&underscore_path);

    fs::create_dir_all(&underscore_path)?;

    Ok(underscore_path)
}
