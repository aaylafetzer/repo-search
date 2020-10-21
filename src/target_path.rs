use std::error::Error;

/// Tests if the given path is a git repository
/// # Arguments
/// 1. path - The path in question
/// # Examples
/// ```rs
/// let is_git = test_for_git("folder").unwrap();
/// ```
pub(crate) fn test_for_git(path: &str) -> Result<&str, Box<dyn Error>> {
     
}