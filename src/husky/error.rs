#[derive(Debug)]
pub enum HuskyError {
    Git(git2::Error),
    Io(std::io::Error),
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
