use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;
use std::fs::File;
use std::process::Command;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    hostname: Option<String>,
    remote_regex: Option<String>,
    local_path_regex: Option<String>,
    key: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConfigFile {
    keys: HashMap<String, String>,
    configs: Vec<Config>,
}

#[derive(Debug)]
struct State {
    git_remote_v: String,
    hostname: String,
}

fn return_first_openable_file(paths: Vec<String>) -> File {
    for path in paths {
        match std::fs::File::open(path) {
            Ok(file) => return file,
            Err(_) => continue,
        }
    }
    panic!("Could not open config file");
}

fn read_config() -> ConfigFile {
    let filename = "multi-github-keys.yaml".to_string();
    let possible_paths = vec![
        filename,
        std::env::var("HOME").unwrap() + "/.config/multi-github-keys.yaml",
    ];
    let f = return_first_openable_file(possible_paths);

    let scrape_config: ConfigFile = serde_yaml::from_reader(f).expect("-");
    return scrape_config;
}

fn github_remote() -> String {
    let output = Command::new("git")
        .arg("remote")
        .arg("-v")
        .output()
        .unwrap();
    return str::from_utf8(&output.stdout[..]).unwrap().to_string();
}

fn run_command(cmd: &str) -> String {
    let output = Command::new(cmd).output().unwrap();
    return str::from_utf8(&output.stdout[..])
        .unwrap()
        .trim_end()
        .to_string();
}

fn hostname() -> String {
    return run_command("hostname");
}

fn local_path() -> String {
    return run_command("pwd");
}

fn config_matches(config: &Config, state: &State) -> bool {
    if config.remote_regex.is_some() {
        let remote_regex = config.remote_regex.as_ref().unwrap();
        let re = Regex::new(remote_regex).unwrap();
        if !re.is_match(&state.git_remote_v) {
            return false;
        }
    }

    if config.hostname.is_some() && &state.hostname != config.hostname.as_ref().unwrap() {
        return false;
    }

    if config.local_path_regex.is_some() {
        let local_path_regex = config.local_path_regex.as_ref().unwrap();
        let re = Regex::new(local_path_regex).unwrap();
        if !re.is_match(&local_path()) {
            return false;
        }
    }

    return true;
}

fn print_ssh_command(key: &String) {
    println!("{}", key);
}

fn main() {
    let config = read_config();
    let keys = config.keys;
    let state = State {
        git_remote_v: github_remote(),
        hostname: hostname(),
    };
    // println!("{:?}", state);
    for conf in config.configs.iter() {
        if config_matches(&conf, &state) {
            print_ssh_command(&keys.get(&conf.key).unwrap());
            return;
        }
    }
    print_ssh_command(&keys.get("default").unwrap());
}
