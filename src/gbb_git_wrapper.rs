use std::{
    io,
    process::{Command, Output, Stdio},
};

use crate::{
    git_infos::{BRANCH_NAME, REMOTE_NAME},
    Cli,
};

pub struct CmdRunner;

pub struct GitUiWrapper {
    args: Vec<String>,
}

pub struct GitWrapper {
    main_arg: String,
    args: Vec<String>,
}

pub enum CmdRunnable {
    GitUi(GitUiWrapper),
    Git(GitWrapper),
}

impl CmdRunner {
    pub fn new(cli: Cli) -> CmdRunnable {
        match cli.main_param.as_str() {
            "ui" => CmdRunnable::GitUi(GitUiWrapper { args: cli.args }),
            _ => CmdRunnable::Git(GitWrapper {
                main_arg: cli.main_param,
                args: cli.args,
            }),
        }
    }
}

pub trait CmdRunnerTrait {
    fn run_command(&self) -> io::Result<Output>;
}

impl CmdRunnerTrait for CmdRunnable {
    fn run_command(&self) -> io::Result<Output> {
        match self {
            Self::GitUi(wrap) => wrap.run_command(),
            Self::Git(wrap) => wrap.run_command(),
        }
    }
}

impl CmdRunnerTrait for GitWrapper {
    fn run_command(&self) -> io::Result<Output> {
        run_cmd_stdout_inherited(
            Command::new("git")
                .arg(handle_git_main_param_alias(&self))
                .args(handle_params(&self)),
        )
    }
}

impl CmdRunnerTrait for GitUiWrapper {
    fn run_command(&self) -> io::Result<Output> {
        run_cmd_stdout_inherited(Command::new("gitui").args(&self.args))
    }
}

fn run_cmd_stdout_inherited(command: &mut Command) -> Result<Output, io::Error> {
    command
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
}

pub fn handle_git_main_param_alias(git_wrapper: &GitWrapper) -> String {
    match git_wrapper.main_arg.as_str() {
        "p" => "push".to_string(),
        "c" => "commit".to_string(),
        "ch" => "checkout".to_string(),
        "m" => "merge".to_string(),
        "pl" => "pull".to_string(),
        "r" => "rebase".to_string(),
        "b" => "branch".to_string(),
        "d" => "diff".to_string(),
        "s" => "stash".to_string(),
        "i" => "init".to_string(),
        _ => git_wrapper.main_arg.to_owned(),
    }
}
fn handle_params(git_wrapper: &GitWrapper) -> Vec<String> {
    match git_wrapper.main_arg.as_str() {
        "push" => handle_push(git_wrapper),
        "diff" => handle_diff(git_wrapper),
        _ => git_wrapper.args.to_owned(),
    }
}
fn handle_push(git_wrapper: &GitWrapper) -> Vec<String> {
    let mut args = vec![];
    for ele in git_wrapper.args.as_slice() {
        match ele.as_str() {
            "-u" => {
                args.push("-u".to_string());
                add_branch_and_remote_to_args(&mut args);
            }
            "-d" => {
                args.push("-d".to_string());
                add_branch_and_remote_to_args(&mut args);
            }
            _ => args.push(ele.to_string()),
        }
    }
    args
}

fn handle_diff(git_wrapper: &GitWrapper) -> Vec<String> {
    let mut args = vec![];
    for ele in git_wrapper.args.as_slice() {
        match ele.as_str() {
            "-st" => {
                args.push("--staged".to_string());
            }
            _ => args.push(ele.to_string()),
        }
    }
    args
}

fn add_branch_and_remote_to_args(args: &mut Vec<String>) {
    args.push(REMOTE_NAME.to_string());
    args.push(BRANCH_NAME.to_string());
}
