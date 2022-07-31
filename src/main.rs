use fencer::{args, config, search, utilities};

fn main() {
    
    let args =args::Args::parse();

    let regex_json: config::RegexConfig = config::RegexConfig::new().unwrap();
    let results= search::search_for_secrets(&args.project_dir, &args.excluded_paths, regex_json).unwrap();

    // print the results
    utilities::print_results(results);
}
