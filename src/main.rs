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

    let regex_json: config::RegexConfig = config::RegexConfig::new().unwrap();
    let results= search::search_for_secrets(&config.project_dir, regex_json).unwrap();

    for result in results {
        println!("file name: {:?} line number: {:?} scan type: {:?}", result.file_name, result.line_number, result.scan_type);
    }    
}
