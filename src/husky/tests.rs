use std::path::PathBuf;

use crate::husky::repository::{HuskyRepository, MockHuskyRepository};

#[test]
pub fn list_task_actions() {
    // arrange
    let mut mock_repo = MockHuskyRepository::new();
    mock_repo
        .expect_get_husky_path()
        .returning(|| Ok(PathBuf::from(".husky/task_runner.json")));

    let r = mock_repo.get_husky_path().unwrap();
    assert_eq!(r, PathBuf::from(".husky/task_runner.json"))
}
