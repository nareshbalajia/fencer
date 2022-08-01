# Fencer

Fencer is a mini-CLI tool that can used to scan various kind of secrets/credentials that are hardcoded into a project source code files

# Features

- Identiying the secrets injected into the source code and outputs it along with the kind of secret (Eg: AWS Creds, Github Personal Access Token)

- Exluding sub-dirs within the project that does not have UTF 8 encoded files like build/, target/

## Table of Contents

- [Installation](#installation)
  - Installation Using Cargo(#cargo_installation)
  - Installing the executable directly(#github_release)
  - Building the docker locally(#docker_build)
- [Usage Examples](#usage)
  - Local Execution
  - Docker Execution
- Supported Secrets


# Installation
<sup>[(Back to top)](#table-of-contents)</sup>

## Installation Using Cargo

If you installed Cargo, the official Rust Package manager, run the following command:

```sh
cargo install fencer
```

## Installing the executable directly

If rust/cargo isn't installed on your machine, you can directly install the executable from the repo's release page
    - <Coming Soon>

## Building the docker locally

This repo comes in with a Dockerfile that can be built and run locally

```sh
docker build -t fencer:local .
```


# Usage Examples
<sup>[(Back to top)](#table-of-contents)</sup>

## Local Execution

```bash
❯ fencer --help
Fencer 1.0.0
Naresh, nareshbalajia@mail.com
A mini CLI tool to scan creds and secrets in source code

USAGE:
    fencer [OPTIONS] --project_dir <project_dir>

OPTIONS:
    -e, --exclude_paths <exclude_paths>
            The directories to exclude for the scan [default: target build .git]

    -h, --help
            Print help information

    -p, --project_dir <project_dir>
            Input the relative path to the project dirs

    -V, --version
            Print version information
```


## Docker Execution
Mounting the project directory using the docker run command would enable the tool to be run via Docker

```bash
docker run -v "$(pwd)"/<project_dir_path>:/app -it nareshbalajia/fencer:latest --project_dir /app
```

# Supported Secrets

| Secret Type | Match Pattern |
| --- | --- |
| aws | REGEX |
| amazon_mws_auth_token | REGEX |
| authorization_basic | REGEX |
| authorization_bearer | REGEX |
| authorization_api | REGEX |
| google_api | REGEX |
| google_oauth | REGEX |
| json_web_token | REGEX |
| firebase | REGEX |
| facebook_access_token | REGEX |
| github_access_token | REGEX |
| ssh_rsa | String Match |
| ssh_ec | String Match |
| passwords | REGEX/String Match |
