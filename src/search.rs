//use std::fs;

pub struct SearchResult {
    pub file_name: String,
    pub line_number: i32,
    pub line_content: String
}

impl SearchResult {
    pub fn new(file_name: &str, line_number: i32, line_content: &str) -> Option<SearchResult> {
        if file_name.is_empty() || line_content.is_empty() || line_number > 0 {
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

/* pub fn search_for_secrets(project_dir: &str, regex_config: RegexConfig) -> Result<SearchResult, &str> {
    let dir = fs::read_dir(project_dir).unwrap_or_else(|error| {
        return Err("Failed to open the dir {}", error);
    });

    for file in dir {
        println!("Name: {}", file.unwrap().path().display())
    }
}
 */
