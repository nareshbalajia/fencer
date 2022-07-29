use std::env;
use std::process;

use fencer::{config, search};

fn main() {
    
    let args: Vec<String> = env::args().collect();
    // get the args parsed from parse_args
    let config = config::Config::new(&args).unwrap_or_else(|err| {
        println!("\nNot able to parse args {}", err);
        process::exit(1);
    });

    println!("{:?}", config.project_dir);
    let regex_json: config::RegexConfig = config::RegexConfig::new().unwrap();

    //search for secrets in the project
    println!("{:?}", regex_json.regex_pattern);

    let result= search::search_for_secrets(&config.project_dir, regex_json).unwrap();
    
    println!("{:?}", result.file_name);
}
