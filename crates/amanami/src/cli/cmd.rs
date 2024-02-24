use std::process::ExitCode;

use super::args::{Args, SubCommands};
use crate::config::loader;

pub fn run() -> anyhow::Result<ExitCode> {
    // parse cmd args
    let cmd = <Args as clap::Parser>::parse();

    // load config
    let config = match cmd.config_file {
        Some(file) => {
            // load config file from cli args
            loader(&file)
        }
        None => {
            // if no config file provided from cli args
            // load default config file location
            let file = String::from("./config/config.yaml");
            loader(&file)
        }
    };

    println!("{:?}", config);

    match cmd.subcommand {
        SubCommands::Eks {} => {
            println!("EKS Subcommand");
        }
    }

    Ok(ExitCode::SUCCESS)
}
