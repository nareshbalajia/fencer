use std::fs;
use std::error::Error;

use crate::config::{ScanResults};

pub fn read_dir_recurse(project_dir: &str, excluded_paths: &Vec<String>, files_path_vec: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(project_dir).unwrap();
    
    for file in dir {
        // get file name object and file path
        let file_path = file.unwrap().path();

        //get metadata to find if obj is file or dir. if file, add to vector, if not, all recursion
        let file_md = fs::metadata(&file_path).unwrap();
        if file_md.is_dir() {
            if !is_excluded_dir(&file_path.display().to_string(), excluded_paths) {
                read_dir_recurse(&file_path.display().to_string(), excluded_paths, files_path_vec).ok();
            }            
        }
        else {
            // push the file name for the string vector
            files_path_vec.push(file_path.display().to_string());
        }
        
    }
    Ok(())
}


fn is_excluded_dir(dir: &str, excluded_paths: &Vec<String>) -> bool {
    for paths in excluded_paths {
        if dir.contains(paths) {
            return true
        }
    }
    false
} 

pub fn print_results(scan_results: Vec<ScanResults>) {
    if scan_results.len() > 0 {
        println!("This sucks, but fencer found some secrets injected into the source code:");
        for result in scan_results {
            println!(
                "File name: {:?}, Line Number: {:?}, Secret Type: {:?}",
                result.file_name, result.line_number, result.scan_type
            );
        }
    }
    else {
        println!("Yay! No secrets");
    }
}