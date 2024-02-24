use std::process::ExitCode;

use super::args::{Args, SubCommands};
use crate::config::load_config;

pub fn run() -> anyhow::Result<ExitCode> {
    // parse cmd args
    let cmd = <Args as clap::Parser>::parse();

    // load config
    let config = match cmd.config_file {
        Some(file) => {
            // load config file from cli args
            load_config(&file)
        }
        None => {
            // if no config file provided from cli args
            // load default config file location
            let file = String::from("./config/config.yaml");
            load_config(&file)
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
