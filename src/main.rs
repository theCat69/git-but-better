use std::env;

use gbb_git_wrapper::CmdRunner;
use gbb_git_wrapper::CmdRunnerTrait;

mod gbb_git_wrapper;
mod git_infos;

fn main() {
    let mut cmd_iter = env::args().skip(1);
    CmdRunner::new(&mut cmd_iter)
        .run_command()
        .expect("Git wrapper should return an output");
}
