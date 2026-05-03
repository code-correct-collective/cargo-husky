pub mod error;
pub mod task_runner;
mod utils;

use std::fs::{self, OpenOptions};
use std::io::{self, Write};

use crate::cli::RunArgs;
use crate::husky::task_runner::TaskList;
use crate::husky::utils::{ASSETS_TASK_RUNNER, UnitHuskyResult};

pub fn install(directory: &str) -> UnitHuskyResult {
    writeln!(io::stdout(), "⚡ Installing husky to {}..", &directory)?;
    let repository = utils::open_repository()?;

    let underscore_path = utils::create_install_path(repository.path(), directory)?;
    let mut config = repository.config()?;

    config.set_str(utils::HOOKS_PATH, directory)?;

    let Some(install_path) = underscore_path.parent() else {
        unreachable!()
    };

    let husky_path = utils::write_asset_file(&underscore_path, utils::ASSETS_HUSKY_SH)?;
    utils::write_asset_file(install_path, utils::ASSETS_TASK_RUNNER)?;

    let gitignore_path = underscore_path.join(underscore_path.join(".gitignore"));
    utils::write_file(&gitignore_path, "*")?;

    utils::set_execute_permissions(&husky_path)?;

    writeln!(io::stdout(), "✔️ Husky installed")?;

    Ok(())
}

pub fn uninstall() -> UnitHuskyResult {
    let repository = utils::open_repository()?;
    let mut config = repository.config()?;
    // let directory = config.get_str(HOOKS_PATH).unwrap_or(DEFAULT_DIRECTORY);
    let directory = utils::get_husky_path(&repository)?;

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
    config.remove(utils::HOOKS_PATH)?;

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

    let repository = utils::open_repository()?;
    let install_path = utils::get_husky_path(&repository)?;

    utils::write_asset_filename(&install_path, hook_name, utils::ASSETS_HOOK)?;

    let hook_path = install_path.join(hook_name);
    utils::set_execute_permissions(&hook_path)?;

    if !command.trim().is_empty() {
        let command = format!("{}\n", command.trim());

        let mut hook_file = OpenOptions::new().append(true).open(hook_path)?;

        hook_file.write_all(command.as_bytes())?;
    }
    writeln!(io::stdout(), "✔️ {} hook updated", hook_name)?;
    Ok(())
}
pub fn list() -> UnitHuskyResult {
    let repository = utils::open_repository()?;
    let task_list_file = utils::get_husky_path(&repository)?.join(ASSETS_TASK_RUNNER);

    let task_list = TaskList::open(task_list_file.as_path())?;

    task_runner::display_tasks(&task_list)?;

    Ok(())
}

pub fn run(args: &RunArgs) -> UnitHuskyResult {
    let repository = utils::open_repository()?;
    let install_path = utils::get_husky_path(&repository)?.join(ASSETS_TASK_RUNNER);
    let task_list = TaskList::open(install_path.as_path())?;

    match (&args.name, &args.group) {
        (None, None) => task_runner::run_tasks(&task_list.tasks.iter().collect())?,
        (Some(name), _) => task_runner::run_task_by_name(&task_list.tasks, name)?,
        (_, Some(group)) => task_runner::run_tasks_by_group(&task_list.tasks, group)?,
    };

    Ok(())
}
