
use serde_json;
use serde::{Deserialize, Serialize}; 
use std::collections::HashMap;

use crate::REGEX_JSON_STRING;

// struct for loading regex config JSON from main.rs
pub struct RegexConfig {
    pub regex_pattern: HashMap<String, String>
}

// struct for parsing Command line arguements
pub struct Config {
    pub project_dir: String,
}


impl Config {
    pub fn new(args: &Vec<String>) -> Result<Config, &str> {
        // we'll be getting only 1 custom arguement from user apart from Rust's project path arg in 0 index
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

// with this impl, we're loading a static string into the serde json serializer object and building the RegexConfig struct
impl RegexConfig {
    pub fn new() -> Option<Self> {
        let regex_json_result = serde_json::from_str(REGEX_JSON_STRING).expect("Failed to load string JSON");
        Some(
            RegexConfig {
                regex_pattern: regex_json_result
            }
        )
    }
}