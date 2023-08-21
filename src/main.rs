use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::collections::HashMap;
use std::fs::File;
use std::process::Command;
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    hostname: String,
    remote_regex: String,
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

fn hostname() -> String {
    let output = Command::new("hostname").output().unwrap();
    return str::from_utf8(&output.stdout[..])
        .unwrap()
        .trim_end()
        .to_string();
}

fn config_matches(config: &Config, state: &State) -> bool {
    let re = Regex::new(&config.remote_regex).unwrap();
    if !re.is_match(&state.git_remote_v) {
        return false;
    }

    if state.hostname != config.hostname {
        return false;
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
