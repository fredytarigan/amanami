use std::process::ExitCode;

use super::args::{Args, SubCommands};

pub fn run() -> anyhow::Result<ExitCode> {
    // parse cmd args
    let cmd = <Args as clap::Parser>::parse();

    // load config
    

    match cmd.subcommand {
        SubCommands::Eks {} => {
            println!("EKS Subcommand");
        }
    }

    Ok(ExitCode::SUCCESS)
}
