use std::fs;
use std::error::Error;

use crate::config::RegexConfig;

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

pub fn search_for_secrets(project_dir: &str, regex_config: RegexConfig) -> Result<(SearchResult), Box<dyn Error>> {
    let mut files_path_vec: Vec<String> = Vec::new();
    read_dir_recurse(project_dir, &mut files_path_vec).unwrap();
    println!("All file names are {:?}", files_path_vec);
    
    Ok(
        SearchResult::new("test", 2 , "test").unwrap()
    )
}

fn read_dir_recurse(project_dir: &str, files_path_vec: &mut Vec<String>) -> Result<(), Box<dyn Error>> {
    println!("Incoming dir: {}", project_dir);
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
                read_dir_recurse(&file_path.display().to_string(), files_path_vec);
            }
        }
        else {
            // push the file name for the string vector
            files_path_vec.push(file_path.display().to_string());
        }
        
    }

    Ok(())

}