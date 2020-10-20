use std::error::Error;
use std::collections::LinkedList;
use std::io::{self, BufRead};
use std::fs::File;

/// Returns a vector of regular expressions from a given file
/// # Arguments
/// 1. path - The path to the file containing the expressions
/// # Examples
/// ```rs
/// let config = load_regex("regex.txt").unwrap();
/// ```
pub(crate) fn load_regex(path: &str) -> Result<LinkedList<String>, Box<dyn Error>> {
    let mut list: LinkedList<String> = LinkedList::new();

    // Load the list from the given file
    let f = File::open(path)?;
    // Read the lines of the file
    let lines = io::BufReader::new(f).lines();
    for line in lines {
        if let Ok(ip) = line {
            list.push_back(ip);
        }
    }
    Ok(list)
}