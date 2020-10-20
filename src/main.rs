mod load_regex;

use clap::{App, load_yaml};

fn main() {
    // Set up Clap
    let yaml_args = load_yaml!("arguments.yaml");
    let matches = App::from(yaml_args).get_matches();

    // Process arguments
    let input_path: &str = matches.value_of("INPUT").expect("Error getting input argument");
    // Read regular expressions from file
    let _expressions = load_regex::load_regex(input_path);

}