use lazy_static::lazy_static;
use std::{
    env::{self, Args},
    ffi::OsStr,
    iter::Skip,
    process::{Command, Stdio},
};

lazy_static! {
    static ref BRANCH_NAME: String = get_current_git_branch();
    static ref REMOTE_NAME: String = get_git_remote();
}

fn main() {
    let mut cmd_iter = env::args().skip(1);

    let git_main_param = cmd_iter.next().expect("No git main command");
    let args: Vec<String>;

    match git_main_param.as_str() {
        "push" => args = handle_push(cmd_iter),
        _ => args = cmd_iter.collect(),
    }

    let mut git_cmd = Command::new("git");
    git_cmd.arg(git_main_param);

    git_cmd
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("git command failed");
}

fn handle_push(cmd_iter: Skip<Args>) -> Vec<String> {
    let mut args = vec![];
    for ele in cmd_iter {
        match ele.as_str() {
            "-u" => {
                args.push("-u".to_string());
                args.push(REMOTE_NAME.to_string());
                args.push(BRANCH_NAME.to_string());
            }
            "-d" => {
                args.push("-d".to_string());
                args.push(REMOTE_NAME.to_string());
                args.push(BRANCH_NAME.to_string());
            }
            _ => args.push(ele),
        }
    }

    return args;
}

fn get_git_remote() -> String {
    return run_git_command(vec!["remote"], "should get remote name");
}

fn get_current_git_branch() -> String {
    return run_git_command(
        vec!["branch", "--show-current", "2>nul"],
        "Cannot get current branch",
    );
}

fn run_git_command<T: AsRef<OsStr>>(args: Vec<T>, msg: &str) -> String {
    return String::from_utf8(Command::new("git").args(args).output().expect(msg).stdout)
        .expect("stdout from command should be utf8 stream of bytes")
        .trim()
        .to_string();
}
