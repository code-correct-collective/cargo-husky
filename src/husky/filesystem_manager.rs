use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

#[cfg(test)]
use mockall::automock;

use rust_embed::Embed;

use crate::husky::error::{HuskyResult, UnitHuskyResult};

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

#[cfg_attr(test, automock)]
pub trait HuskyFilesystemManager {
    fn write_asset_file(&self, directory: &Path, asset_name: &str) -> HuskyResult<PathBuf>;
    fn write_asset_filename(
        &self,
        directory: &Path,
        filename: &str,
        asset_name: &str,
    ) -> HuskyResult<PathBuf>;
    fn write_file(&self, file_path: &Path, content: &str) -> UnitHuskyResult;
    fn set_execute_permissions(&self, path: &Path) -> UnitHuskyResult;
    fn create_install_path(&self, git_root_path: &Path, install_dir: &str) -> HuskyResult<PathBuf>;
    fn exists(&self, path: &Path) -> HuskyResult<bool>;
    fn remove_dir_all(&self, path: &Path) -> UnitHuskyResult;
    fn write_hook_file(&self, hook_file_path: &Path, command: &str) -> UnitHuskyResult;
}

pub struct LocalFilesystem {}

impl HuskyFilesystemManager for LocalFilesystem {
    fn write_asset_file(&self, directory: &Path, asset_name: &str) -> HuskyResult<PathBuf> {
        self.write_asset_filename(directory, asset_name, asset_name)
    }

    fn write_asset_filename(
        &self,
        directory: &Path,
        filename: &str,
        asset_name: &str,
    ) -> HuskyResult<PathBuf> {
        let Some(ref file_data) = Assets::get(asset_name) else {
            unreachable!()
        };

        let file_path = directory.join(filename);

        let content = str::from_utf8(file_data.data.as_ref()).ok().unwrap_or("");

        self.write_file(&file_path, content)?;

        Ok(file_path)
    }

    fn write_file(&self, file_path: &Path, content: &str) -> UnitHuskyResult {
        if !file_path.exists() {
            fs::write(file_path, content)?;
        }

        Ok(())
    }

    fn set_execute_permissions(&self, path: &Path) -> UnitHuskyResult {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let mut perms = fs::metadata(path)?.permissions();
            perms.set_mode(perms.mode() | 0o100);
            fs::set_permissions(path, perms)?;
        }

        Ok(())
    }

    fn create_install_path(&self, git_root_path: &Path, install_dir: &str) -> HuskyResult<PathBuf> {
        let install_path = git_root_path.join(install_dir);
        let underscore_path = install_path.join("_");

        fs::create_dir_all(&underscore_path)?;

        Ok(underscore_path)
    }

    fn exists(&self, path: &Path) -> HuskyResult<bool> {
        Ok(fs::exists(path)?)
    }

    fn remove_dir_all(&self, path: &Path) -> UnitHuskyResult {
        Ok(fs::remove_dir_all(path)?)
    }

    fn write_hook_file(&self, hook_file_path: &Path, command: &str) -> UnitHuskyResult {
        let mut hook_file = OpenOptions::new().append(true).open(hook_file_path)?;

        Ok(hook_file.write_all(command.as_bytes())?)
    }
}
