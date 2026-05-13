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

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: Rc<str>,
    pub group: Option<Rc<str>>,
    pub command: Rc<str>,
    pub cwd: Option<Rc<str>>,
    pub args: Option<Vec<String>>,
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

pub trait TaskRunner {
    fn run(&self, task: &Task, repository: &impl HuskyRepository) -> HuskyResult<Output>;
    fn should_run(&self, globs: &[String], paths: &[String]) -> HuskyResult<bool>;
}

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

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub tasks: Vec<Task>,
}

impl TaskList {
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
