use crate::gbb_git_wrapper::CmdWrapper;
use std::env;

mod gbb_git_wrapper;
mod git_infos;

fn main() {
    let mut cmd_iter = env::args().skip(1);
    CmdWrapper::new(&mut cmd_iter)
        .run_command()
        .expect("Git wrapper should return an output");
}
