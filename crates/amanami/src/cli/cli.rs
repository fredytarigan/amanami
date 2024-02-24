use std::process::ExitCode;

use super::cmd::{CmdLine, SubCommands};

pub fn run() -> anyhow::Result<ExitCode> {
    // parse cmd args
    let cmd = <CmdLine as clap::Parser>::parse();

    // load config
    

    match cmd.subcommand {
        SubCommands::EKS {} => {
            println!("EKS Subcommand");
        }
    }

    Ok(ExitCode::SUCCESS)
}
