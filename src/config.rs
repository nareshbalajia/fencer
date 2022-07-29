
use serde_json;
use std::collections::HashMap;

use crate::REGEX_JSON_STRING;

// struct for loading regex config JSON from main.rs
pub struct RegexConfig {
    pub regex_pattern: HashMap<String, String>
}


// struct for storing results about the scan
pub struct ScanResults {
    pub scan_type: String,
    pub file_name: String,
    pub line_number: i32
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