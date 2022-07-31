pub mod args;
pub mod config;
pub mod search;
pub mod utilities;

const REGEX_JSON_STRING: &str = r#"{
    "aws": "A[SK]IA[0-9A-Z]{16}",
    "google_api": "AIza[0-9A-Za-z-_]{35}",
    "google_oauth": "ya29\\.[0-9A-Za-z\\-_]+",
    "json_web_token": "ey[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*$",
    "firebase": "AAAA[A-Za-z0-9_-]{7}:[A-Za-z0-9_-]{140}",
    "facebook_access_token": "EAACEdEose0cBA[0-9A-Za-z]+",
    "github_access_token": "[a-zA-Z0-9_-]*:[a-zA-Z0-9_\\-]+@github\\.com*",
    "ssh_rsa": "-----BEGIN RSA PRIVATE KEY-----",
    "ssh_ec": "-----BEGIN EC PRIVATE KEY-----"
}"#;