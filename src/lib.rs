pub mod args;
pub mod config;
pub mod search;
pub mod utilities;

const REGEX_JSON_STRING: &str = r#"{
    "aws": "A[SK]IA[0-9A-Z]{16}",
    "ssh_rsa": "-----BEGIN RSA PRIVATE KEY-----",
    "ssh_ec": "-----BEGIN EC PRIVATE KEY-----"
}"#;