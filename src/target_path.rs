use std::error::Error;
use std::fs::metadata;
use git2::Repository;

/// Tests if the given path is a git repository
/// # Arguments
/// 1. path - The path in question
/// # Examples
/// ```rs
/// let is_git = test_for_git("folder").unwrap();
/// ```
pub(crate) fn test_for_git(path: &str) -> Result<&str, Box<dyn Error>> {
     let md = metadata(path)?;
     if md.is_file() {
        return Ok("file");
     }
     let repo = match Repository::open(path) {
         Ok(repo) => {
            // Path is a git repository
            return Ok("git")
         }
         Err(e) => {
             // Path is not a git repo
            return Ok("folder")
         }
     };
}