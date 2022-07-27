use std::error::Error;
use std::fs;

use serde::Deserialize;
use serde_json;

pub struct RegexConfig {
    pub regex_pattern: serde_json::Value
}

pub struct Config {
    pub project_dir: String,
}


impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Too less arguements!")
        }
        else if args.len() > 2 {
            return Err("Required exactly 1 argurment. i.e Project Path!")
        }
        Ok(Config{
            project_dir: args[1].clone()
        })
    }
}

impl RegexConfig {
    pub fn new(json_str: &'static str) -> Result<RegexConfig, &str> {
        let regex_json_result = serde_json::from_str(json_str).expect("Failed to load string JSON");
        Ok(RegexConfig {
            regex_pattern: regex_json_result
        })
    }
}