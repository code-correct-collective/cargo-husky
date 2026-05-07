use std::{
    fs,
    path::{Path, PathBuf},
};

use rust_embed::Embed;

use crate::husky::utils::{HuskyResult, UnitHuskyResult};

#[derive(Embed)]
#[folder = "assets"]
struct Assets;

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
}
