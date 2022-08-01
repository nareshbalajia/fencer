use std::fs;
use std::error::Error;
use indicatif::ProgressBar;

use regex::Regex;

use crate::config::{RegexConfig, ScanResults};
use crate::utilities::{read_dir_recurse};

pub struct SearchResult {
    pub file_name: String,
    pub line_number: i32,
    pub line_content: String
}

impl SearchResult {
    pub fn new(file_name: &str, line_number: i32, line_content: &str) -> Option<SearchResult> {
        if file_name.is_empty() || line_content.is_empty() || line_number <= 0 {
            return None
        }
        Some(
            SearchResult {
                file_name: file_name.to_owned(),
                line_number: line_number,
                line_content: line_content.to_owned()
            }
        )
    }
}

pub fn search_for_secrets(project_dir: &str, excluded_paths: &Vec<String>, regex_config: RegexConfig) -> Result<Vec<ScanResults>, Box<dyn Error>> {
    // prepare vec for recursive finding of project dir files
    let mut files_path_vec: Vec<String> = Vec::new();

    // why this must be declared as mut? remove this and error will be thrown at vec push statement
    let mut scan_results_vec: Vec<ScanResults> = Vec::new();

    // get the recursed project file names
    read_dir_recurse(project_dir, excluded_paths, &mut files_path_vec).unwrap();
    
    // now read all the files and check for secrets:
    let pb = ProgressBar::new(1024);
    println!("\u{1b}[33mPro Tip - if the project dir is too big, consider giving the sub dirs paths for efficient scanning\u{1b}[39m");
    println!("Sit tight! Scanning the files..");
    for file in files_path_vec {
        pb.inc(1);
        let file_read = fs::read_to_string(&file);
        let file_contents = match file_read {
            //issue is here
            Ok(file_contents) => file_contents,
            Err(_error) =>  {
                continue;
            }
        };
        let mut line_number = 0;
        for line in file_contents.lines() {
            line_number = line_number + 1;
            for (key,value) in &regex_config.regex_pattern {
                let scan_regex = Regex::new(&value).unwrap();
                if scan_regex.is_match(line) {
                    scan_results_vec.push(
                        ScanResults {
                            file_name: file.to_owned(),
                            scan_type: key.to_owned(),
                            line_number: line_number
                        }
                    );
                 }
            }   
        }
    }
    pb.finish_with_message("Done Scanning");
    Ok(
        scan_results_vec
    )
}
