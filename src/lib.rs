pub mod args;
pub mod config;
pub mod search;
pub mod utilities;

const REGEX_JSON_STRING: &str = r#"{
    "aws": "A[SK]IA[0-9A-Z]{16}",
    "amazon_mws_auth_token" : "amzn\\.mws\\.[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}",
    "authorization_basic" : "basic [a-zA-Z0-9=:_\\+/-]{5,100}",
    "authorization_bearer" : "bearer [a-zA-Z0-9_\\-\\.=:_\\+/]{5,100}",
    "authorization_api" : "api[key|_key|\\s+]+[a-zA-Z0-9_\\-]{5,100}",
    "google_api": "AIza[0-9A-Za-z-_]{35}",
    "google_oauth": "ya29\\.[0-9A-Za-z\\-_]+",
    "json_web_token": "ey[A-Za-z0-9-_=]+\\.[A-Za-z0-9-_=]+\\.?[A-Za-z0-9-_.+/=]*$",
    "firebase": "AAAA[A-Za-z0-9_-]{7}:[A-Za-z0-9_-]{140}",
    "facebook_access_token": "EAACEdEose0cBA[0-9A-Za-z]+",
    "github_access_token": "(?i)ghp_[0-9a-zA-Z]{36}",
    "ssh_rsa": "-----BEGIN RSA PRIVATE KEY-----",
    "ssh_ec": "-----BEGIN EC PRIVATE KEY-----",
    "password_c1": "(?i)password\\s*",
    "password_c2": "(?i)password is\\s*",
    "password_c3": "(?i)pwd\\s*"
}"#;