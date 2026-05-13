//! The module that contains the shared application error

/// Simplified [HuskyError] that contains a result
pub type HuskyResult<T> = Result<T, HuskyError>;

/// Simplified [HuskyError] type contains a unit result.
pub type UnitHuskyResult = HuskyResult<()>;

/// The shared application error object
#[derive(Debug)]
pub enum HuskyError {
    /// Errors returned by the [git2::Error] crate.
    Git(git2::Error),

    /// Errors returned by [std::io::Error]
    Io(std::io::Error),

    /// The task runner was not found or could not be parsed
    InvalidTaskRunnerFile,

    /// The specified task was not found
    TaskNotFound,

    /// The task did complete successfully
    TaskFailed,

    /// The task was skipped due to include or exclude
    TaskSkipped(String),

    /// Errors returned by [serde_json::Error]
    Serde(serde_json::Error),

    /// Errors returned by Globset [globset::Error]
    Globset(globset::Error),
}

impl From<globset::Error> for HuskyError {
    fn from(value: globset::Error) -> Self {
        HuskyError::Globset(value)
    }
}

impl From<git2::Error> for HuskyError {
    fn from(value: git2::Error) -> Self {
        HuskyError::Git(value)
    }
}

impl From<std::io::Error> for HuskyError {
    fn from(value: std::io::Error) -> Self {
        HuskyError::Io(value)
    }
}

impl From<serde_json::Error> for HuskyError {
    fn from(value: serde_json::Error) -> Self {
        HuskyError::Serde(value)
    }
}

// todo: implement Display
