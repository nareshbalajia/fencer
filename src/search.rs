use std::fs;

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
        SearchResult {
            file_name: file_name,
            line_number: line_number,
            line_content: line_content
        }
    }
}


