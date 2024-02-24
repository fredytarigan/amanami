mod autoscaling;
mod config;
mod ec2;
mod eks;
mod ssm;

use crate::config::AwsConfig;
use crate::output::OutputTable;
use config::Config;
use eks::{Eks, EksCluster};
use std::sync::mpsc::channel;
use std::thread;

use colored::*;

use comfy_table::{Attribute, Cell, CellAlignment, Color};

#[derive(Debug, Clone)]
pub struct Aws {
    aws_account: Vec<AwsAccount>,
}

#[derive(Debug, Clone)]
pub struct AwsAccount {
    account_id: String,
    role_arn: Option<String>,
    eks: Option<Vec<EksConfig>>,
}

#[derive(Debug, Clone)]
pub struct EksConfig {
    cluster_name: String,
    region: String,
}

impl Aws {
    pub fn new(config: Vec<AwsConfig>) -> Self {
        let mut aws: Vec<AwsAccount> = Vec::new();

        for account in config {
            let mut eks: Vec<EksConfig> = Vec::new();

            if let Some(d) = account.eks {
                for item in d {
                    let eks_config = EksConfig {
                        cluster_name: item.cluster_name,
                        region: item.region,
                    };

                    eks.push(eks_config);
                }
            }

            let eks_data = match eks.len() {
                0 => None,
                _ => Some(eks),
            };

            let aws_account = AwsAccount {
                account_id: account.account_id,
                role_arn: account.role_arn,
                eks: eks_data,
            };

            aws.push(aws_account);
        }

        Self { aws_account: aws }
    }

    pub fn get_eks_updates(&self) -> Result<(), std::io::Error> {
        // construct a vec of eks cluster
        let mut eks_clusters: Vec<EksCluster> = Vec::new();
        // let mut result = HashMap::new();
        let (tx, rx) = channel();

        for account in self.aws_account.clone() {
            if let Some(eks) = account.eks {
                for item in eks {
                    let cluster = EksCluster {
                        account_id: account.account_id.clone(),
                        cluster_name: item.cluster_name,
                        region: item.region,
                        role_arn: account.role_arn.clone(),
                    };

                    eks_clusters.push(cluster);
                }
            }
        }

        // loop over all aws account
        for cluster in eks_clusters {
            let tx = tx.clone();
            thread::spawn(move || {
                let config = Config {
                    role_arn: cluster.role_arn,
                    region: cluster.region.clone(),
                    ..Default::default()
                };

                let config = config.generate_config();

                // generate eks client
                let eks = Eks::new(
                    &config,
                    cluster.cluster_name.clone(),
                    cluster.region.clone(),
                );

                let client = eks.client();

                let result = eks.get_cluster_update(&client);

                let _ = tx.send((
                    cluster.account_id,
                    cluster.region,
                    cluster.cluster_name.clone(),
                    result,
                ));
            });
        }

        drop(tx);

        // let's prepare the output table
        let mut rows = vec![];

        while let Ok((account_id, region, cluster_name, result)) = rx.recv() {
            let upgrade_available: Cell;
            if result.cluster_version == "Not Available" {
                upgrade_available = Cell::new(&result.cluster_version)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black);
            } else {
                upgrade_available = Cell::new(&result.cluster_version)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .fg(Color::Green);
            }

            let cluster_data = vec![
                Cell::new(account_id),
                Cell::new(cluster_name),
                Cell::new(region).set_alignment(CellAlignment::Center),
                Cell::new(result.cluster_version).set_alignment(CellAlignment::Center),
                Cell::new(result.latest_cluster_version).set_alignment(CellAlignment::Center),
                upgrade_available,
            ];

            rows.push(cluster_data);
        }

        // define output table
        let table = OutputTable::new(
            vec![
                Cell::new(String::from("AWS Account ID"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
                Cell::new(String::from("EKS Cluster Name"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
                Cell::new(String::from("Region"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
                Cell::new(String::from("EKS Cluster Version"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
                Cell::new(String::from("Latest Version Available"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
                Cell::new(String::from("Upgrade Available"))
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::DarkMagenta),
            ],
            rows,
        );

        println!("");
        println!("{}", "EKS Cluster Details: ".bold().yellow());
        table.display_output();

        println!("");

        Ok(())
    }
}
