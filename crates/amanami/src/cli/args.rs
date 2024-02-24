use clap::{Parser, Subcommand};
use std::path::PathBuf;

use super::styles::get_styles;

#[derive(Parser, Debug)]
#[command(name = "amanami")]
#[command(about = "A simple CLI application to check available updates for a certain things")]
#[command(styles=get_styles())]
pub struct Args {
    /// Config file
    /// Application will look for "config/config.yaml" if this option isn't specified
    #[clap(verbatim_doc_comment)]
    #[arg(short = 'f', long, value_name = "CONFIG_FILE")]
    pub config_file: Option<PathBuf>,

    #[command(subcommand)]
    pub subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
pub enum SubCommands {
    /// AWS EKS update check
    #[clap(verbatim_doc_comment)]
    Eks,
}
