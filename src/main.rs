use fencer::{args, config, search};

fn main() {
    
    let args =args::Args::parse();

    let regex_json: config::RegexConfig = config::RegexConfig::new().unwrap();
    let results= search::search_for_secrets(&args.project_dir, regex_json).unwrap();

    for result in results {
        println!("file name: {:?} line number: {:?} scan type: {:?}", result.file_name, result.line_number, result.scan_type);
    }    
}
