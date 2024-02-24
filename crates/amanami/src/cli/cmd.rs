use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "amanami")]
#[command(about = "A simple CLI application to check available updates for a certain things")]
pub struct CmdLine {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {}
