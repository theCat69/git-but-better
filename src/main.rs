use std::{
    env::{self, Args},
    iter::Skip,
    process::{Command, Stdio},
};

fn main() {
    let mut cmd_iter = env::args().skip(1);

    let git_main_param = cmd_iter.next().expect("No git main command");
    let args: Vec<String>;

    if git_main_param == "push" {
        args = handle_push(cmd_iter);
    } else {
        args = cmd_iter.collect();
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
        if ele == "-u" {
            args.push("-u".to_string());
            args.push("origin".to_string());
            args.push(get_current_git_branch());
        } else {
            args.push(ele);
        }
    }

    return args;
}

fn get_current_git_branch() -> String {
    return String::from_utf8(
        Command::new("git")
            .arg("branch")
            .arg("--show-current")
            .arg("2>nul")
            .output()
            .expect("Cannot get current branch")
            .stdout,
    )
    .expect("stdout from command should be utf8 stream of bytes")
    .trim()
    .to_string();
}
