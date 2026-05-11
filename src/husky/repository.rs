//! The module that contains the husky git repository operations.
use std::{path::PathBuf, vec};

use git2::{Repository, RepositoryOpenFlags, StatusOptions};

#[cfg(test)]
use mockall::automock;

use crate::husky::error::{HuskyResult, UnitHuskyResult};

const DEFAULT_DIRECTORY: &str = ".husky";
const GIT_CONFIG_HOOKS_PATH: &str = "core.hooksPath";

/// The trait that defined the required git repository operations
#[cfg_attr(test, automock)]
pub trait HuskyRepository {
    /// Opens the repisotry in the current working directory
    ///
    /// ## Returns
    ///
    /// An instance of the repository
    fn open_repository() -> HuskyResult<Self>
    where
        Self: Sized;

    /// Gets the path where the husky configuration files are installed.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    ///
    /// ## Returns
    /// The [PathBuf] where the husky configuration files are stored.
    fn get_husky_path(&self) -> HuskyResult<PathBuf>;

    /// Sets the repository git config `core.hooksPath` value.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    /// - `directory` - The directory that will store the git hook scripts
    fn set_hook_path(&self, directory: &str) -> UnitHuskyResult;

    /// Removes the git config `core.hooksPath` value.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    fn remove_hook_path(&self) -> UnitHuskyResult;

    /// Gets the repository path that contains the `.git` folder.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    ///
    /// ## Returns
    /// The [PathBuf] of the directory that containts the `.git` folder
    fn get_repository_root_path(&self) -> HuskyResult<PathBuf>;

    /// Produces the list of files that are currently staged in the system
    ///
    /// ## Returns
    /// Returns a [Vec<String>] of the paths that are currently staged
    fn get_staged_files(&self) -> HuskyResult<Vec<String>>;
}

/// The struct that is used to implment the [git2] repository operations
pub struct GitRepository {
    repository: Repository,
}

impl HuskyRepository for GitRepository {
    fn open_repository() -> HuskyResult<Self> {
        let repository = Repository::open_ext(
            ".",
            RepositoryOpenFlags::empty(),
            &[] as &[&std::ffi::OsStr],
        )?;

        Ok(GitRepository { repository })
    }

    fn get_husky_path(&self) -> HuskyResult<PathBuf> {
        let config = self.repository.config()?;

        let install_segment = config
            .get_str(GIT_CONFIG_HOOKS_PATH)
            .unwrap_or(DEFAULT_DIRECTORY);

        let Some(path) = self.repository.path().parent() else {
            unreachable!();
        };

        Ok(path.join(install_segment))
    }

    fn set_hook_path(&self, directory: &str) -> UnitHuskyResult {
        let mut config = self.repository.config()?;

        config.set_str(GIT_CONFIG_HOOKS_PATH, directory)?;
        Ok(())
    }

    fn remove_hook_path(&self) -> UnitHuskyResult {
        let mut config = self.repository.config()?;

        config.remove(GIT_CONFIG_HOOKS_PATH)?;
        Ok(())
    }

    fn get_repository_root_path(&self) -> HuskyResult<PathBuf> {
        let Some(path) = self.repository.path().parent() else {
            unreachable!()
        }; // every .git folder should have a parent.

        Ok(path.to_path_buf())
    }

    fn get_staged_files(&self) -> HuskyResult<Vec<String>> {
        let mut opts = StatusOptions::new();

        opts.show(git2::StatusShow::IndexAndWorkdir);

        let statuses = self.repository.statuses(Some(&mut opts))?;

        for entry in statuses.iter() {
            let status = entry.status();
            dbg!(status);
        }

        Ok(vec![])
    }
}
