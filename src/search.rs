use std::fs;
use std::error::Error;

use regex::Regex;

use crate::config::{RegexConfig, ScanResults};

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

pub fn search_for_secrets(project_dir: &str, regex_config: RegexConfig) -> Result<Vec<ScanResults>, Box<dyn Error>> {
    // prepare vec for recursive finding of project dir files
    let mut files_path_vec: Vec<String> = Vec::new();

    // why this must be declared as mut? remove this and error will be thrown at vec push statement
    let mut scan_results_vec: Vec<ScanResults> = Vec::new();

    // get the recursed project file names
    read_dir_recurse(project_dir, &mut files_path_vec).unwrap();
    
    // now read all the files and check for secrets:
    for file in files_path_vec {
        let file_contents = fs::read_to_string(&file)?;
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
    Ok(
        scan_results_vec
    )
}

fn read_dir_recurse(project_dir: &str, files_path_vec: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    //ln!("Incoming dir: {}", project_dir);
    let dir = fs::read_dir(project_dir).unwrap();
    
    for file in dir {
        // get file name object and file path
        let filename = file.unwrap();
        let file_path = filename.path();

        //get metadata to find if obj is file or dir. if file, add to vector, if not, all recursion
        let file_md = fs::metadata(filename.path()).unwrap();

        if file_md.is_dir() {
            // recurse to find all file names inside sub dirs
            if !file_path.display().to_string().contains("git") && !file_path.display().to_string().contains("target") {
                read_dir_recurse(&file_path.display().to_string(), files_path_vec).ok();
            }
        }
        else {
            // push the file name for the string vector
            files_path_vec.push(file_path.display().to_string());
        }
        
    }

    Ok(())
}