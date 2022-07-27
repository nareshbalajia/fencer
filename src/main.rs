use std::env;
use std::process;

use fencer::Config;
use fencer::RegexConfig;


fn main() {
    // this will be static string because it wont ever change
    let regex_json_string: &'static str = r#"{
        "aws": "A[SK]IA[0-9A-Z]{16}",
        "ssh_rsa": "-----BEGIN RSA PRIVATE KEY-----",
        "ssh_ec": "-----BEGIN EC PRIVATE KEY-----"
    }"#;

    let args: Vec<String> = env::args().collect();
    // get the args parsed from parse_args
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("\nNot able to parse args {}", err);
        process::exit(1);
    });
    println!("The project path is {:?}", config.project_dir);

    let regex_json = RegexConfig::new(regex_json_string).unwrap();

    println!("The regex string is {:?}", regex_json.regex_pattern);
}
