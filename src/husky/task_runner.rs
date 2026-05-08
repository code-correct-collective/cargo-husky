use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    rc::Rc,
};

use serde::{Deserialize, Serialize};

use crate::husky::{
    error::{HuskyError, UnitHuskyResult},
    filesystem_manager::HuskyFilesystemManager,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: Rc<str>,
    pub group: Option<Rc<str>>,
    pub command: Rc<str>,
    pub cwd: Option<Rc<str>>,
    pub args: Option<Vec<String>>,
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
    ) -> Result<TaskList, HuskyError> {
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

pub fn run_task(task: &Task) -> UnitHuskyResult {
    write_task_header(&task.name)?;

    let mut command = Command::new(&*task.command);

    if let Some(args) = &task.args {
        command.args(args);
    }

    if let Some(cwd) = &task.cwd {
        command.current_dir(PathBuf::from(cwd as &str));
    }

    let output = command.output()?;

    if output.status.success() {
        let out = String::from_utf8_lossy(&output.stdout);
        let err = String::from_utf8_lossy(&output.stderr);

        if !out.is_empty() {
            writeln!(io::stdout(), "{}", out)?;
        }

        if !err.is_empty() {
            writeln!(io::stderr(), "{}", err)?;
        }

        writeln!(io::stdout(), "✔️ task 📋 {} succeeded", task.name)?;
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

        writeln!(io::stderr(), "🚫 task 📋 {} failed", task.name)?;

        Err(HuskyError::TaskFailed)
    }
}

pub fn run_tasks(tasks: &Vec<&Task>) -> UnitHuskyResult {
    for task in tasks {
        run_task(task)?;
    }
    Ok(())
}

pub fn run_tasks_by_group(tasks: &[Task], group: &str) -> UnitHuskyResult {
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
        writeln!(io::stdout(), "⚠️ Group 👥 {} was not found", group)?;
    }

    run_tasks(&groups)?;

    Ok(())
}

pub fn run_task_by_name(tasks: &[Task], name: &str) -> UnitHuskyResult {
    let named: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.name.eq_ignore_ascii_case(name))
        .collect();

    if let Some(t) = named.first() {
        run_task(t)
    } else {
        write_task_header(name)?;

        writeln!(io::stderr(), "⚠️ task 📋 {} was not found", name)?;

        Err(HuskyError::TaskNotFound)
    }
}

fn write_task_header(task_name: &str) -> UnitHuskyResult {
    let sep = "=".repeat(80);

    writeln!(io::stdout(), "\n{}", sep)?;
    writeln!(io::stdout(), "🚀 Running task 📋 {}", task_name)?;
    writeln!(io::stdout(), "{}", sep)?;
    Ok(())
}
