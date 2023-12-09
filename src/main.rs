use std::env;
use std::env::Args;
use std::iter::Skip;

use gbb_git_wrapper::CmdRunner;
use gbb_git_wrapper::CmdRunnerTrait;

mod gbb_git_wrapper;
mod git_infos;

pub struct Cli {
    main_param: String,
    args: Vec<String>,
}

fn main() {
    let mut cmd_iter = env::args().skip(1);

    let cli = parse_args(&mut cmd_iter);

    CmdRunner::new(cli)
        .run_command()
        .expect("Git wrapper should return an output");
}

fn parse_args(cmd_iter: &mut Skip<Args>) -> Cli {
    Cli {
        main_param: cmd_iter
            .next()
            .expect("gbb command should have a main parameter"),
        args: cmd_iter.collect(),
    }
}
