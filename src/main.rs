use lazy_static::lazy_static;
use std::{
    env::{self, Args},
    ffi::OsStr,
    iter::Skip,
    process::{self, Command, Stdio},
};

lazy_static! {
    static ref BRANCH_NAME: String = get_current_git_branch();
    static ref REMOTE_NAME: String = get_git_remote();
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

fn main() {
    let mut cmd_iter = env::args().skip(1);

    let cmd_main_param = cmd_iter
        .next()
        .expect("gbb command should have a main parameter");

    let git_main_param = handle_git_main_param(cmd_main_param).unwrap();

    let args = handle_params(&git_main_param, cmd_iter);

    let mut git_cmd = Command::new("git");
    git_cmd.arg(git_main_param);

    git_cmd
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("git command should return an output");
}

fn handle_params(git_main_param: &String, cmd_iter: Skip<Args>) -> Vec<String> {
    let args: Vec<String>;

    match git_main_param.as_str() {
        "push" => args = handle_push(cmd_iter),
        "diff" => args = handle_diff(cmd_iter),
        _ => args = cmd_iter.collect(),
    }
    args
}

fn handle_git_main_param(cmd_main_param: String) -> Result<String, String> {
    match cmd_main_param.as_str() {
        "ui" => {
            run_git_ui();
            Ok("Should never happen and be refactored".to_string())
        }
        "p" => Ok("push".to_string()),
        "c" => Ok("commit".to_string()),
        "ch" => Ok("checkout".to_string()),
        "m" => Ok("merge".to_string()),
        "pl" => Ok("pull".to_string()),
        "r" => Ok("rebase".to_string()),
        "b" => Ok("branch".to_string()),
        "d" => Ok("diff".to_string()),
        "s" => Ok("stash".to_string()),
        "i" => Ok("init".to_string()),
        _ => Ok(cmd_main_param),
    }
}

fn run_git_ui() {
    Command::new("gitui")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("git command should return an output");
    process::exit(0);
}

fn handle_diff(cmd_iter: Skip<Args>) -> Vec<String> {
    let mut args = vec![];
    for ele in cmd_iter {
        match ele.as_str() {
            "-st" => {
                args.push("--staged".to_string());
            }
            _ => args.push(ele),
        }
    }

    return args;
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
