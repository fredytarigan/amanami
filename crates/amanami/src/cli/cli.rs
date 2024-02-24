use std::process::ExitCode;

use super::cmd::CmdLine;

pub fn run() -> anyhow::Result<ExitCode> {
    let cmd = <CmdLine as clap::Parser>::parse();

    println!("Hello World !!!");

    Ok(ExitCode::SUCCESS)
}
