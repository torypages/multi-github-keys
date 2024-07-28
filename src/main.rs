use clap::Parser;
use regex::Regex;
use std::env;
use std::path::Path;
use std::process::{self, Command};
#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    ssh_key_path: String,
    ssh_clone_string: Option<String>,
}

fn set_ssh_command(ssh_command: &String) {
    if !Path::new(".git").exists() {
        println!("This is not a git repo!!!!!");
        process::exit(1);
    }
    let mut o = Command::new("git")
        .arg("config")
        .arg("core.sshCommand")
        .arg(format!("{ssh_command}"))
        .spawn()
        .expect("Could not set sshCommand");
    let _ = o.wait();
}

fn ssh_add(ssh_key: &String) {
    let mut o = Command::new("ssh-add")
        .arg(ssh_key)
        .spawn()
        .expect("Could not add ssh key");
    let _ = o.wait();
}

fn git_clone(ssh_command: &String, ssh_clone_string: &String) {
    let mut o = Command::new("git")
        .env("GIT_SSH_COMMAND", ssh_command)
        .arg("clone")
        .arg(ssh_clone_string)
        .spawn()
        .expect("Could not set sshCommand");
    let _ = o.wait();
}

fn folder_from_ssh_clone_string(ssh_clone_string: &String) -> String {
    let re = Regex::new(r"^.*/(.*).git$").unwrap();
    return re
        .captures(ssh_clone_string)
        .unwrap()
        .get(1)
        .unwrap()
        .as_str()
        .to_string();
}

fn main() {
    let args: Args = Args::parse();
    let ssh_command = format!("ssh -i {}", args.ssh_key_path);
    ssh_add(&args.ssh_key_path);
    if args.ssh_clone_string.is_some() {
        git_clone(&ssh_command, &args.ssh_clone_string.clone().unwrap());
        let cloned_folder = folder_from_ssh_clone_string(&args.ssh_clone_string.unwrap());
        let new_path_str = "".to_owned() + &cloned_folder;
        let _ = env::set_current_dir(Path::new(&new_path_str));
    }
    set_ssh_command(&ssh_command);
}
