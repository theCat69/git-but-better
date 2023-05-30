use std::{
    env::Args,
    io,
    iter::Skip,
    process::{Command, Output, Stdio},
};

use crate::git_infos::{BRANCH_NAME, REMOTE_NAME};

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
    pub fn new(os_args: &mut Skip<Args>) -> CmdRunnable {
        let cmd_main_param = os_args
            .next()
            .expect("gbb command should have a main parameter");

        let main_arg = handle_git_main_param_alias(cmd_main_param);

        let args = handle_params(&main_arg, os_args);

        match main_arg.as_str() {
            "ui" => CmdRunnable::GitUi(GitUiWrapper { args }),
            _ => CmdRunnable::Git(GitWrapper { main_arg, args }),
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
        run_cmd_stdout_inherited(Command::new("git").arg(&self.main_arg).args(&self.args))
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

fn handle_git_main_param_alias(cmd_main_param: String) -> String {
    match cmd_main_param.as_str() {
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
        _ => cmd_main_param,
    }
}

fn handle_params(git_main_param: &String, cmd_iter: &mut Skip<Args>) -> Vec<String> {
    let args: Vec<String>;

    match git_main_param.as_str() {
        "push" => args = handle_push(cmd_iter),
        "diff" => args = handle_diff(cmd_iter),
        _ => args = cmd_iter.collect(),
    }
    args
}

fn handle_push(cmd_iter: &mut Skip<Args>) -> Vec<String> {
    let mut args = vec![];
    for ele in cmd_iter {
        match ele.as_str() {
            "-u" => {
                args.push("-u".to_string());
                add_branch_and_remote_to_args(&mut args);
            }
            "-d" => {
                args.push("-d".to_string());
                add_branch_and_remote_to_args(&mut args);
            }
            _ => args.push(ele),
        }
    }

    return args;
}

fn handle_diff(cmd_iter: &mut Skip<Args>) -> Vec<String> {
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

fn add_branch_and_remote_to_args(args: &mut Vec<String>) {
    args.push(REMOTE_NAME.to_string());
    args.push(BRANCH_NAME.to_string());
}
