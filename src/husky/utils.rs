use crate::husky::error::HuskyError;

pub type HuskyResult<T> = Result<T, HuskyError>;
pub type UnitHuskyResult = HuskyResult<()>;

pub const DEFAULT_DIRECTORY: &str = ".husky";
pub const ASSETS_HUSKY_SH: &str = "husky.sh";
pub const ASSETS_TASK_RUNNER: &str = "task-runner.json";
pub const ASSETS_HOOK: &str = "hook";
pub const GIT_CONFIG_HOOKS_PATH: &str = "core.hooksPath";
