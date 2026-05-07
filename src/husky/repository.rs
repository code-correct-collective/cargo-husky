use std::path::{Path, PathBuf};

use git2::{Repository, RepositoryOpenFlags};

use crate::husky::utils::{DEFAULT_DIRECTORY, GIT_CONFIG_HOOKS_PATH, HuskyResult, UnitHuskyResult};

pub trait HuskyRepository {
    fn open_repository() -> HuskyResult<Self>
    where
        Self: Sized;
    fn get_husky_path(&self) -> HuskyResult<PathBuf>;
    fn set_hook_path(&self, directory: &str) -> UnitHuskyResult;
    fn remove_hook_path(&self) -> UnitHuskyResult;
    fn get_repository_root_path(&self) -> HuskyResult<&Path>;
}

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

    fn get_repository_root_path(&self) -> HuskyResult<&Path> {
        let Some(path) = self.repository.path().parent() else {
            unreachable!()
        }; // every .git folder should have a parent.

        Ok(path)
    }
}
