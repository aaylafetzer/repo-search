use std::error::Error;
use std::fs::metadata;
use git2::Repository;
use walkdir::WalkDir;

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
     match Repository::open(path) {
         Ok(_repo) => {
            // Path is a git repository
            return Ok("git")
         }
         Err(_e) => {
             // Path is not a git repo
            return Ok("folder")
         }
     };
}

/// Walks the given directory and runs each file through 
/// the regular expressions
/// # Arguments
/// 1. path - The path of the directory to walk
/// # Examples
/// ```rs
/// walk_directory("folder/");
/// ```
pub(crate) fn walk_directory(path: &str) -> Result<(), Box<dyn Error>>{
   for entry in WalkDir::new(path) {
      println!("{}", entry?.path().display());
   }
   Ok(())
}