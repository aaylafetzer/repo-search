mod load_regex;
mod target_path;

use clap::{App, load_yaml};

fn main() {
    // Set up Clap
    let yaml_args = load_yaml!("arguments.yaml");
    let matches = App::from(yaml_args).get_matches();

    // Process arguments
    let expressions_path: &str = matches.value_of("EXPRESSIONS").expect("Error getting input argument");
    let repo_path: &str = matches.value_of("REPO").expect("Error getting repo argument");
    // Read regular expressions from file
    let expressions = load_regex::load_regex(expressions_path);
    // Find the provided path
    let is_git = target_path::test_for_git(repo_path).expect("uwu");
    match is_git {
        "git" => {
            println!("Uwu!");
        }
        "folder" => {
            println!("Owo");
        }
        "file" => {
            println!("uguu");
        }
        _ => {
            panic!("How did you get here?");
        }
    }
}