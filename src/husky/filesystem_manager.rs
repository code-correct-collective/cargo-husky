//! The wrapper around the filesystem operations.

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

/// The husky file manager trait that wraps the intended file operations.
#[cfg_attr(test, automock)]
pub trait HuskyFilesystemManager {
    /// Writes a file from the embedded assets using the asset name as the filename.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    /// - `directory` - The directory to create the file.
    /// - `asset_name` - The name of the embedded asset
    ///
    /// ## Returns
    /// The [PathBuf] where the file was created.
    fn write_asset_file(&self, directory: &Path, asset_name: &str) -> HuskyResult<PathBuf>;

    /// Writes a file from the embedded assets using the specified filename.
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `directory` - The directory to create the file.
    /// - `filename` - The target filename to write
    /// - `asset_name` - The name of the embedded asset
    ///
    /// ## Returns
    /// The [PathBuf] where the file was created.
    fn write_asset_filename(
        &self,
        directory: &Path,
        filename: &str,
        asset_name: &str,
    ) -> HuskyResult<PathBuf>;

    /// Write a file at the specified path with the specified content
    ///
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `file_path` - The path to the file to create
    /// - `content` - The string content of the destination file.
    ///
    /// ## Returns
    /// The [PathBuf] where the file was created.
    fn write_file(&self, file_path: &Path, content: &str) -> UnitHuskyResult;

    /// Sets the execute permission of the git hook script.
    ///
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `path` - The path to the file.
    fn set_execute_permissions(&self, path: &Path) -> UnitHuskyResult;

    /// Creates the directory an initial configuration files.
    ///
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `git_root_path` - The directory  that contains the `.git` folder
    /// - `install_dir` - The directory to install the husky configuration files
    ///
    /// ## Returns
    /// The [PathBuf] of the directory used to install the husky configuration files.
    fn create_install_path(&self, git_root_path: &Path, install_dir: &str) -> HuskyResult<PathBuf>;

    /// Checks if the specified path exists
    ///
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    ///
    /// ## Returns
    /// `true` if the file exists; otherwise `false`
    fn exists(&self, path: &Path) -> HuskyResult<bool>;

    /// Will remove the specified path and all of its contents.
    ///
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `path` - The directory to remove
    fn remove_dir_all(&self, path: &Path) -> UnitHuskyResult;

    /// Creates that hook script file and sets the execute permission
    /// ## Parameters
    ///
    /// - `&self` - A referene to the implementation type
    /// - `hook_file_path` - The path to the git hook script to be created
    /// - `command` - The command to append to the hook script. i.e. `cargo run -g pre-commit`
    fn write_hook_file(&self, hook_file_path: &Path, command: &str) -> UnitHuskyResult;
}

/// The struct that contains the local filesystem operation
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
