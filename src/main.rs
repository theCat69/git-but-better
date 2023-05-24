use std::{
    env,
    io::{self, Write},
    process::Command,
};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut git_cmd = Command::new("git");

    let output = git_cmd.args(args).output().expect("git command failed");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
