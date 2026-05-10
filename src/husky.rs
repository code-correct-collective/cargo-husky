//! The core library for the husky git hooks and configuration

pub mod error;
pub mod filesystem_manager;
pub mod repository;
pub mod task_runner;

#[cfg(test)]
mod tests;

use std::io::{self, Write};

use crate::cli::RunArgs;
use crate::husky::error::UnitHuskyResult;
use crate::husky::filesystem_manager::HuskyFilesystemManager;
use crate::husky::repository::HuskyRepository;
use crate::husky::task_runner::{TaskList, TaskRunner};

const ASSETS_HOOK: &str = "hook";
const ASSETS_TASK_RUNNER: &str = "task-runner.json";
const ASSETS_HUSKY_SH: &str = "husky.sh";

/// Installs the git hooks in the specified directory
///
/// ## Arguments
///
/// - `directory` - the destination to install and configure the git hooks
/// - `repository` - the husky wrapper around the git repository
/// - `file_manager` - the husky wrapper around the file system.
pub fn install(
    directory: &str,
    repository: &impl HuskyRepository,
    file_manager: &impl HuskyFilesystemManager,
) -> UnitHuskyResult {
    writeln!(io::stdout(), "⚡ Installing husky to {}..", &directory)?;

    let underscore_path =
        file_manager.create_install_path(&repository.get_repository_root_path()?, directory)?;

    repository.set_hook_path(directory)?;

    let Some(install_path) = underscore_path.parent() else {
        unreachable!()
    };

    let husky_path = file_manager.write_asset_file(&underscore_path, ASSETS_HUSKY_SH)?;
    file_manager.write_asset_file(install_path, ASSETS_TASK_RUNNER)?;

    let gitignore_path = underscore_path.join(underscore_path.join(".gitignore"));
    file_manager.write_file(&gitignore_path, "*")?;

    file_manager.set_execute_permissions(&husky_path)?;

    writeln!(io::stdout(), "✔️ Husky installed")?;

    Ok(())
}

/// Uninstall the git hooks the husky configuration files
///
/// ## Arguments
///
/// - `repository` - the husky wrapper around the git repository
/// - `file_manager - the husky wrapper around the file system
pub fn uninstall(
    repository: &impl HuskyRepository,
    file_manager: &impl HuskyFilesystemManager,
) -> UnitHuskyResult {
    let directory = repository.get_husky_path()?;

    writeln!(
        io::stdout(),
        "🗑️ Uninstalling husky from {}",
        directory.display()
    )?;

    let git_parent = repository.get_repository_root_path()?;

    let path = git_parent.join(directory);

    if !file_manager.exists(&path)? {
        writeln!(io::stdout(), "⚠️ Husky already removed")?;
        return Ok(()); // nothing to delete
    }

    file_manager.remove_dir_all(&path)?;
    repository.remove_hook_path()?;

    writeln!(io::stdout(), "✔️ Husky removed")?;

    Ok(())
}

/// Creates the git hook and append the command to the hook script
///
/// If the git hook script does not exist, the file is created, the execute permission is enabled
/// and the optional command is appended to the script.
/// If the git hook exists, the specified command will be appended to the script.
///
/// ## Parameters
///
/// - `hook_name` - the name of the git hook (`pre-commit`, `pre-push`, `pre-receive`, etc)
/// - `command` - the shell command to append to the hook script
/// - `repository` - the husky wrapper around the git repository
/// - `file_manager` - the husky wrapper around the file system.
pub fn set_hook(
    hook_name: &str,
    command: &str,
    repository: &impl HuskyRepository,
    file_manager: &impl HuskyFilesystemManager,
) -> UnitHuskyResult {
    writeln!(
        io::stdout(),
        "🛠️ Setting the command '{}' on the {} hook.",
        command,
        hook_name
    )?;

    let install_path = repository.get_husky_path()?;

    file_manager.write_asset_filename(&install_path, hook_name, ASSETS_HOOK)?;

    let hook_path = install_path.join(hook_name);
    file_manager.set_execute_permissions(&hook_path)?;

    if !command.trim().is_empty() {
        let command = format!("{}\n", command.trim());
        file_manager.write_hook_file(&hook_path, &command)?;
    }

    writeln!(io::stdout(), "✔️ {} hook updated", hook_name)?;

    Ok(())
}

/// List the available commands defined in the task runner
///
/// ## Parameters
///
/// - `repository` - the husky wrapper around the git repository
/// - `file_manager` - the husky wrapper around the file system.
pub fn list(
    repository: &impl HuskyRepository,
    file_manager: &impl HuskyFilesystemManager,
) -> UnitHuskyResult {
    let task_list_file = repository.get_husky_path()?.join(ASSETS_TASK_RUNNER);

    let task_list = TaskList::open(task_list_file.as_path(), file_manager)?;

    task_runner::display_tasks(&task_list)
}

/// Runs a task defined in the task-runner.json file.
/// ## Parameters
///
/// - `args`: The task or group name to run.
/// - `repository` - the husky wrapper around the git repository
/// - `file_manager` - the husky wrapper around the file system.
/// - `task_runner` - The husky wrapper around the spawned command
pub fn run(
    args: &RunArgs,
    repository: &impl HuskyRepository,
    file_manager: &impl HuskyFilesystemManager,
    task_runner: &impl TaskRunner,
) -> UnitHuskyResult {
    let install_path = repository.get_husky_path()?.join(ASSETS_TASK_RUNNER);
    let task_list = TaskList::open(install_path.as_path(), file_manager)?;

    match (&args.name, &args.group) {
        (None, None) => task_runner::run_tasks(&task_list.tasks.iter().collect(), task_runner)?,
        (Some(name), _) => task_runner::run_task_by_name(&task_list.tasks, name, task_runner)?,
        (_, Some(group)) => task_runner::run_tasks_by_group(&task_list.tasks, group, task_runner)?,
    };

    Ok(())
}
