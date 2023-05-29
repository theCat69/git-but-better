use std::{ffi::OsStr, process::Command};

use lazy_static::lazy_static;

lazy_static! {
    pub static ref BRANCH_NAME: String = get_current_git_branch();
    pub static ref REMOTE_NAME: String = get_git_remote();
}

fn run_git_command<T: AsRef<OsStr>>(args: Vec<T>, msg: &str) -> String {
    return String::from_utf8(Command::new("git").args(args).output().expect(msg).stdout)
        .expect("stdout from command should be utf8 stream of bytes")
        .trim()
        .to_string();
}

fn get_git_remote() -> String {
    return run_git_command(vec!["remote"], "should get remote name");
}

fn get_current_git_branch() -> String {
    return run_git_command(
        vec!["branch", "--show-current", "2>nul"],
        "Should return current branch name",
    );
}
