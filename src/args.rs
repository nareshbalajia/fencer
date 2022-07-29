use clap::{Command, Arg};

pub struct Args {
    pub project_dir: String,
    pub excluded_paths: String
}

impl Args {
    pub fn parse() -> Self {
        let arg_match = Command::new("My Program")
            .author("Naresh, nareshbalajia@mail.com")
            .version("1.0.0")
            .name("Fencer")
            .about("A mini CLI tool to scan creds and secrets in source code")
            .arg(
                Arg::with_name("project_dir")
                    .short('p')
                    .long("project_dir")
                    .takes_value(true)
                    .required(true)
                    .help("Input the relative path to the project dirs"),
            )
            .arg(
                Arg::with_name("exclude_paths")
                    .short('e')
                    .long("exclude_paths")
                    .takes_value(true)
                    .required(false)
                    .help("The directories to exclude for the scan"),
            ).get_matches();
        
        let project_dir = arg_match.value_of("project_dir").unwrap().to_string();
        let excluded_paths = arg_match.value_of("exclude_paths").unwrap().to_string();

        Args {
            project_dir: project_dir,
            excluded_paths: excluded_paths
        }
    }
}