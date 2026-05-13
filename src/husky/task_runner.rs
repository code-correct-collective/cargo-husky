//! This module contains the code that handles running tasks.
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::{Command, Output},
    rc::Rc,
};

use globset::{Glob, GlobSetBuilder};
use serde::{Deserialize, Serialize};

use crate::husky::{
    error::{HuskyError, HuskyResult, UnitHuskyResult},
    filesystem_manager::HuskyFilesystemManager,
    repository::HuskyRepository,
};

/// The definition of a task from the `task-runner.json` file
#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    /// The name of the task, can be used in the `cargo husky run -n (name)` command.
    pub name: Rc<str>,
    /// The group  name of the task, can be used to run a group of tasks with `cargo husky run -g
    /// (group)`
    pub group: Option<Rc<str>>,
    /// The shell command to execute
    pub command: Rc<str>,
    /// The current working directory to run the task within.
    pub cwd: Option<Rc<str>>,

    /// The arguments to pass to the command.
    pub args: Option<Vec<String>>,

    /// A glob pattern of files that at least one must exist to run the task
    pub include: Option<Vec<String>>,

    /// A glob pattern of files that if any exist the task will be skipped.
    pub exclude: Option<Vec<String>>,
}

/// The trait that wraps the IO process from the remainder of the code.
pub trait TaskRunner {
    /// Handles executing the process.
    ///
    /// ## Parameters
    /// - `&self` - A referene to the implementation type
    /// - `task` - The task to attempt to run.
    /// - `repository` - The git repository for context.
    ///
    /// ## Returns
    /// The [Output] of the command.
    fn run(&self, task: &Task, repository: &impl HuskyRepository) -> HuskyResult<Output>;

    /// Tests if the task should run based on if the file globs and the list of paths
    ///
    /// ## Parameters
    /// - `&self` - A reference to the implementation type
    /// - `globs` - A list of glob patterns from either the include/exclude task definition
    /// - `paths` - The source of paths used to filter against
    ///
    /// ## Returns
    /// A [bool] when at least one of the globs matched the incoming paths.
    fn should_run(&self, globs: &[String], paths: &[String]) -> HuskyResult<bool>;
}

/// The default struct to implement the [TaskRunner] trait.
pub struct HuskyTaskRunner;

impl TaskRunner for HuskyTaskRunner {
    fn run(&self, task: &Task, repository: &impl HuskyRepository) -> HuskyResult<Output> {
        let mut command = Command::new(&*task.command);

        let staged_files = repository.get_staged_files()?;

        if let Some(include) = &task.include {
            if !self.should_run(include, &staged_files)? {
                return Err(HuskyError::TaskSkipped(String::from(
                    "Included file(s) not found",
                )));
            }
        }

        if let Some(exclude) = &task.exclude {
            if self.should_run(exclude, &staged_files)? {
                return Err(HuskyError::TaskSkipped(String::from(
                    "Excluded file(s) found",
                )));
            }
        }

        if let Some(args) = &task.args {
            command.args(args);
        }

        if let Some(cwd) = &task.cwd {
            command.current_dir(PathBuf::from(cwd as &str));
        }

        Ok(command.output()?)
    }

    fn should_run(&self, globs: &[String], paths: &[String]) -> HuskyResult<bool> {
        let mut builder = GlobSetBuilder::new();
        for glob in globs {
            builder.add(Glob::new(glob)?);
        }

        let matcher = builder.build()?;

        for path in paths {
            if matcher.is_match(path) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

/// The collection of tasks deserialized from the `task-runner.json` file
#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    /// The JSON schema reference
    #[serde(rename = "$schema")]
    pub schema: String,
    /// The list of [Task] objects.
    pub tasks: Vec<Task>,
}

impl TaskList {
    /// Deserialized the `task-runner.json` into an object.
    ///
    /// ## Parameters
    /// - `path` - The [Path] to the `task-runner.json` file
    /// - `filesystem_manager` - An instance of the [HuskyFilesystemManager]
    ///
    /// ## Returns
    /// The deserialized instance of the [TaskList]
    pub fn open(
        path: &Path,
        filesystem_manager: &impl HuskyFilesystemManager,
    ) -> HuskyResult<TaskList> {
        if !filesystem_manager.exists(path).unwrap_or(false) {
            return Err(HuskyError::InvalidTaskRunnerFile);
        }

        let Some(json_tasks) = fs::read_to_string(path).ok() else {
            return Err(HuskyError::InvalidTaskRunnerFile);
        };

        let task_list: TaskList = serde_json::from_str(&json_tasks)?;

        Ok(task_list)
    }
}

/// Displays the tasks defined in the `task-runner.json` file.
///
/// ## Parameters
/// - `task_list` - The list of tasks defined in the `task-runner.json` file.
pub fn display_tasks(task_list: &TaskList) -> UnitHuskyResult {
    let sep = "=".repeat(80);
    writeln!(io::stdout(), "{}", sep)?;
    writeln!(io::stdout(), "🚀 Husky Task List")?;
    writeln!(io::stdout(), "{}", sep)?;
    for task in &task_list.tasks {
        let group = match &task.group {
            Some(g) => g.clone(),
            None => "[Unspecified]".into(),
        };

        writeln!(io::stdout(), "📋 {} 👥 {}", task.name, group)?;
    }
    Ok(())
}

/// Runs a specific tasks
///
/// ## Parameters
/// - `task` - The specific task to run
/// - `task_runner` - The [TaskRunner] instance that handles the process.
/// - `repository` - The [HuskyRepository] instance that represents the git repository
pub fn run_task(
    task: &Task,
    task_runner: &impl TaskRunner,
    repository: &impl HuskyRepository,
) -> UnitHuskyResult {
    write_task_header(&task.name)?;

    let output = task_runner.run(task, repository)?;

    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);

        if !out.is_empty() {
            writeln!(io::stdout(), "{}", out)?;
        }

        if !err.is_empty() {
            writeln!(io::stderr(), "{}", err)?;
        }

        writeln!(io::stdout(), "✔️ 📋 {} succeeded", task.name)?;
        Ok(())
    } else {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);

        if !out.is_empty() {
            writeln!(io::stdout(), "{}", out)?;
        }

        if !err.is_empty() {
            writeln!(io::stderr(), "{}", err)?;
        }

        writeln!(io::stderr(), "🚫 📋 {} failed", task.name)?;

        Err(HuskyError::TaskFailed)
    }
}

/// Runs the specified list of tasks
///
/// ## Parameters
/// - `tasks` - The list of tasks to execute
/// - `tassk_runner` - The [TaskRunner] instance to handle the processes
/// - `repository` - The [HuskyRepository] of the current git repository.
pub fn run_tasks(
    tasks: &Vec<&Task>,
    task_runner: &impl TaskRunner,
    repository: &impl HuskyRepository,
) -> UnitHuskyResult {
    for task in tasks {
        match run_task(task, task_runner, repository) {
            Ok(_) => continue,
            Err(HuskyError::TaskSkipped(message)) => {
                writeln!(io::stderr(), "↪️ 📋 {} skipped: '{}'", task.name, message)?;
                continue;
            }
            Err(err) => return Err(err),
        }
    }
    Ok(())
}

/// Runs the list of tasks that match a group name.
///
/// ## Parameters
/// - `tasks` - The list of tasks to filter.
/// - `group` - The group name to use when filtering.
/// - `task_runner` - The [TaskRunner] instance to execute the task.
/// - `repository` = The [HuskyRepository] of the current git repository.
pub fn run_tasks_by_group(
    tasks: &[Task],
    group: &str,
    task_runner: &impl TaskRunner,
    repository: &impl HuskyRepository,
) -> UnitHuskyResult {
    writeln!(
        io::stdout(),
        "⌛ Preparing to run tasks in group 👥 {}",
        group
    )?;
    let groups: Vec<&Task> = tasks
        .iter()
        .filter(|t| match &t.group {
            Some(g) => g.eq_ignore_ascii_case(group),
            _ => false,
        })
        .collect();

    if groups.is_empty() {
        writeln!(io::stdout(), "⚠️ 👥 {} was not found", group)?;
    }

    run_tasks(&groups, task_runner, repository)?;

    Ok(())
}

/// Runs the specific task that match the specified task name.
///
/// ## Parameters
/// - `tasks` - The list of tasks to filter.
/// - `name` - The task name to use when filtering.
/// - `task_runner` - The [TaskRunner] instance to execute the task.
/// - `repository` = The [HuskyRepository] of the current git repository.
pub fn run_task_by_name(
    tasks: &[Task],
    name: &str,
    task_runner: &impl TaskRunner,
    repository: &impl HuskyRepository,
) -> UnitHuskyResult {
    let named: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.name.eq_ignore_ascii_case(name))
        .collect();

    if let Some(t) = named.first() {
        run_task(t, task_runner, repository)
    } else {
        write_task_header(name)?;

        writeln!(io::stderr(), "⚠️ 📋 {} was not found", name)?;

        Err(HuskyError::TaskNotFound)
    }
}

fn write_task_header(task_name: &str) -> UnitHuskyResult {
    let sep = "=".repeat(80);

    writeln!(io::stdout(), "\n{}", sep)?;
    writeln!(io::stdout(), "🚀 Running 📋 {}", task_name)?;
    writeln!(io::stdout(), "{}", sep)?;
    Ok(())
}
