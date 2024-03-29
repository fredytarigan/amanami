use super::args::{Args, SubCommands};
use crate::aws::Aws;
use crate::config::loader;
use crate::errors::ApplicationErrors;

pub fn run() -> Result<(), ApplicationErrors> {
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

    match cmd.subcommand {
        SubCommands::Eks {} => {
            let aws = Aws::new(config.aws);

            aws.get_eks_clusters_update()?;
            aws.get_eks_nodegroups_update()?;
            aws.get_eks_addons_update()?;

            Ok(())
        }

        SubCommands::Iam {} => {
            let aws = Aws::new(config.aws);

            aws.get_iam_user_details()?;

            Ok(())
        }
    }
}
