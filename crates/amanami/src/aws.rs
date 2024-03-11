mod autoscaling;
mod config;
mod ec2;
mod eks;
mod ssm;
mod iam;

use crate::errors::ApplicationErrors;
use crate::output::OutputTable;
use crate::config::AwsConfig;
use config::Config;
use eks::{Eks, EksCluster, EksNodeGroup};
use iam::{Iam, IamAccount};
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
    iam: Option<Vec<IamConfig>>,
}

#[derive(Debug, Clone)]
pub struct EksConfig {
    cluster_name: String,
    region: String,
}

#[derive(Debug, Clone)]
pub struct IamConfig {
    region: String,
}

impl Aws {
    pub fn new(config: Vec<AwsConfig>) -> Self {
        let mut aws: Vec<AwsAccount> = Vec::new();

        for account in config {
            let mut eks: Vec<EksConfig> = Vec::new();
            let mut iam: Vec<IamConfig> = Vec::new();

            if let Some(d) = account.eks {
                for item in d {
                    let eks_config = EksConfig {
                        cluster_name: item.cluster_name,
                        region: item.region,
                    };

                    eks.push(eks_config);
                }
            }

            if let Some(d) = account.iam {
                for item in d {
                    let iam_config = IamConfig {
                        region: item.region,
                    };

                    iam.push(iam_config);
                }
            }

            let eks_data = match eks.len() {
                0 => None,
                _ => Some(eks),
            };

            let iam_data = match iam.len() {
                0 => None,
                _ => Some(iam),
            };

            let aws_account = AwsAccount {
                account_id: account.account_id,
                role_arn: account.role_arn,
                eks: eks_data,
                iam: iam_data,
            };

            aws.push(aws_account);
        }

        Self { aws_account: aws }
    }

    pub fn get_eks_clusters_update(&self) -> Result<(), ApplicationErrors> {
        // construct a vec of eks cluster
        let mut eks_clusters: Vec<EksCluster> = Vec::new();
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

        thread::scope(|scope| {
            // loop over all aws account
            for cluster in eks_clusters {
                let tx = tx.clone();
                scope.spawn(move || -> Result<(), ApplicationErrors> {
                    let config = Config {
                        role_arn: cluster.role_arn,
                        region: cluster.region.clone(),
                    };

                    let config = config.generate_config();

                    // generate eks client
                    let eks = Eks::new(
                        &config,
                        cluster.cluster_name.clone(),
                        cluster.region.clone(),
                    );

                    let client = eks.client();

                    let cluster_version = match eks.get_cluster_version(&client) {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("{} {}", "Error occured while getting cluster version on EKS cluster".bold().red(), cluster.cluster_name.bold().red());
                            eprintln!();
                            return Err(e);
                        }
                    };

                    let latest_cluster_version = match eks.get_latest_cluster_version(&client) {
                        Ok(d) => d,
                        Err(e) => {
                            eprintln!("{} {}", "Error occured while getting latest cluster version on EKS cluster".bold().red(), cluster.cluster_name.bold().red());
                            eprintln!();
                            return Err(e);
                        }
                    };

                    let mut upgrade_available = String::from("Not Available");

                    if cluster_version != latest_cluster_version {
                        upgrade_available = String::from("Available");
                    }

                    let _ = tx.send((
                        cluster.account_id,
                        cluster.region,
                        cluster.cluster_name.clone(),
                        cluster_version,
                        latest_cluster_version,
                        upgrade_available,
                    ));

                    Ok(())
                });
            }
        });

        drop(tx);

        // let's prepare the output table
        let mut rows = vec![];

        while let Ok((
                account_id, 
                region, 
                cluster_name, 
                cluster_version,
                latest_cluster_version,
                upgrade_available
            )) = rx.recv() {
            let upgrade_available: Cell = if upgrade_available == "Not Available" {
                Cell::new(&upgrade_available)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
            } else {
                Cell::new(&upgrade_available)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .fg(Color::Green)
            };

            let cluster_data = vec![
                Cell::new(account_id),
                Cell::new(cluster_name),
                Cell::new(region).set_alignment(CellAlignment::Center),
                Cell::new(cluster_version).set_alignment(CellAlignment::Center),
                Cell::new(latest_cluster_version).set_alignment(CellAlignment::Center),
                upgrade_available,
            ];

            rows.push(cluster_data);
        }

        if ! rows.is_empty() {

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

            println!("{}", "EKS Cluster Details: ".bold().yellow());
            table.display_output();
            println!();
        }

        Ok(())
    }

    pub fn get_eks_nodegroups_update(&self) -> Result<(),ApplicationErrors> {
        // consutruct a vec of eks cluster
        let mut eks_clusters: Vec<EksNodeGroup> = Vec::new();
        let (tx, rx) = channel();

        for account in self.aws_account.clone() {
            if let Some(eks) = account.eks {
                for item in eks {
                    let config = Config {
                        role_arn: account.role_arn.clone(),
                        region: item.region.clone(),
                    };

                    let config = config.generate_config();

                    // generate eks client
                    let eks = Eks::new(&config, item.cluster_name.clone(), item.region.clone());

                    let client = eks.client();

                    let result_nodegroup = eks.list_nodegroups(&client);

                    let nodegroup = match result_nodegroup {
                        Ok(d) => d,
                        Err(e) => {
                            println!(
                                "{} {}", 
                                "Error occured while getting nodegroup list on EKS cluster".bold().red(), 
                                item.cluster_name
                            );
                            println!();
                            return Err(e);
                        }
                    };

                    let result: Vec<_> = nodegroup.iter().flatten().collect();

                    for node in result {
                        eks_clusters.push(EksNodeGroup {
                            account_id: account.account_id.clone(),
                            cluster_name: item.cluster_name.clone(),
                            region: item.region.clone(),
                            role_arn: account.role_arn.clone(),
                            node_name: String::from(node),
                        })
                    }
                }
            }
        }

        // loop over all aws account
        for cluster in eks_clusters {
            let tx = tx.clone();
            thread::spawn(move || -> Result<(), ApplicationErrors> {
                let config = Config {
                    role_arn: cluster.role_arn,
                    region: cluster.region.clone(),
                };

                let config = config.generate_config();

                // generate eks client
                let eks = Eks::new(
                    &config,
                    cluster.cluster_name.clone(),
                    cluster.region.clone(),
                );

                let client = eks.client();

                let result_nodegroup = eks.get_nodegroup_update(&client, cluster.node_name.clone());

                let result = match result_nodegroup {
                    Ok(d) => d,
                    Err(e) => {
                        println!(
                            "{} {} {} {}", 
                            "Error occured while getting node group update version on EKS cluster ".bold().red(),
                            cluster.cluster_name.bold().red(),
                            "with nodegroup name ".bold().red(),
                            cluster.node_name.clone().bold().red()
                        );
                        println!();
                        return Err(e);
                    }
                };

                let _ = tx.send((
                    cluster.account_id,
                    cluster.region,
                    cluster.cluster_name.clone(),
                    result,
                ));

                Ok(())
            });
        }

        drop(tx);

        // let's prepare the output table
        let mut rows = vec![];

        while let Ok((account_id, region, cluster_name, data)) = rx.recv() {
            let upgrade_available: Cell = if data.upgrade_available == "Not Available" {
                Cell::new(&data.upgrade_available)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
            } else {
                Cell::new(&data.upgrade_available)
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .fg(Color::Green)
            };

            let nodegrop_data = vec![
                Cell::new(account_id.clone()),
                Cell::new(cluster_name.clone()),
                Cell::new(region.clone()).set_alignment(CellAlignment::Center),
                Cell::new(data.node_name),
                Cell::new(data.ami_name).set_alignment(CellAlignment::Center),
                Cell::new(data.latest_ami_name).set_alignment(CellAlignment::Center),
                upgrade_available,
            ];

            rows.push(nodegrop_data);
        }

        if ! rows.is_empty() {

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
                    Cell::new(String::from("Nodegroup Name"))
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::DarkMagenta),
                    Cell::new(String::from("AMI Version"))
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::DarkMagenta),
                    Cell::new(String::from("Latest AMI Version"))
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

            println!("{}", "Nodegroup Details: ".bold().yellow());
            table.display_output();
            println!();
        }

        Ok(())
    }

    pub fn get_eks_addons_update(&self) -> Result<(), ApplicationErrors> {
        // construct a vec of eks cluster
        let mut eks_clusters: Vec<EksCluster> = Vec::new();
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

        thread::scope(|scope| {
            // loop over all aws account
            for cluster in eks_clusters {
                let tx = tx.clone();

                scope.spawn(move || -> Result<(), ApplicationErrors> {
                    let config = Config {
                        role_arn: cluster.role_arn,
                        region: cluster.region.clone(),
                    };

                    let config = config.generate_config();

                    // generate eks client
                    let eks = Eks::new(
                        &config,
                        cluster.cluster_name.clone(),
                        cluster.region.clone(),
                    );

                    let client = eks.client();

                    let result_cluster_version = eks.get_cluster_version(&client);

                    let cluster_version = match result_cluster_version {
                        Ok(d) => d,
                        Err(e) => {
                            println!(
                                "{} {}",
                                "Error occured while getting EKS cluster version for"
                                    .bold()
                                    .red(),
                                cluster.cluster_name.bold().red()
                            );
                            println!();
                            return Err(e);
                        }
                    };

                    let result_addons = eks.list_addons(&client);

                    let addons = match result_addons {
                        Ok(d) => d,
                        Err(e) => {
                            println!(
                                "{} {}",
                                "Error occured while list addons on EKS cluster"
                                    .bold()
                                    .red(),
                                cluster.cluster_name.bold().red()
                            );
                            println!();
                            return Err(e);
                        }
                    };

                    for addon in addons {
                        let result_current_version = eks.get_addons_version(
                            &client,
                            addon.clone(),
                            cluster.cluster_name.clone(),
                        );

                        let current_version = match result_current_version {
                            Ok(d) => d,
                            Err(e) => {
                                println!(
                                    "{} {} {} {}",
                                    "Error occured while getting addons version on EKS cluster"
                                        .red()
                                        .bold(),
                                    cluster.cluster_name.bold().red(),
                                    "and addon name".bold().red(),
                                    addon.bold().red()
                                );
                                println!();
                                return Err(e);
                            }
                        };

                        let result_latest_version = eks.get_addons_latest_version(
                            &client,
                            addon.clone(),
                            cluster_version.clone(),
                        );

                        let latest_version = match result_latest_version {
                            Ok(d) => d,
                            Err(e) => {
                                println!(
                                    "{} {} {} {}", 
                                    "Error occured while getting addons latest version on EKS cluster".bold().red(), 
                                    cluster.cluster_name.bold().red(), 
                                    "and addon name".bold().red(), 
                                    addon.bold().red()
                                );
                                println!();
                                return Err(e);
                            }
                        };

                        let _ = tx.send((
                            cluster.account_id.clone(),
                            cluster.cluster_name.clone(),
                            cluster.region.clone(),
                            addon,
                            current_version,
                            latest_version,
                        ));
                    }

                    Ok(())
                });
            }
        });

        drop(tx);

        // let's prepare the output table
        let mut rows = vec![];

        while let Ok((
            account_id,
            cluster_name,
            region,
            addon_name,
            current_version,
            latest_version,
        )) = rx.recv()
        {
            let upgrade_available: Cell = if current_version == latest_version {
                Cell::new("Not Available")
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
            } else {
                Cell::new("Available")
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black)
                    .fg(Color::Green)
            };

            let addon_data = vec![
                Cell::new(account_id),
                Cell::new(cluster_name),
                Cell::new(region).set_alignment(CellAlignment::Center),
                Cell::new(addon_name),
                Cell::new(current_version),
                Cell::new(latest_version),
                upgrade_available,
            ];

            rows.push(addon_data);
        }

        if ! rows.is_empty() {
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
                    Cell::new(String::from("Addon Name"))
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::DarkMagenta),
                    Cell::new(String::from("Installed Version"))
                        .set_alignment(CellAlignment::Center)
                        .add_attribute(Attribute::Bold)
                        .fg(Color::DarkMagenta),
                    Cell::new(String::from("Latest Version"))
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

            println!("{}", "Addons Details: ".bold().yellow());
            table.display_output();
            println!();
        }

        Ok(())
    }

    pub fn get_iam_user_details(&self) -> Result<(), ApplicationErrors> {
        // construct a vec of iam account
        let mut iam_accounts: Vec<IamAccount> = Vec::new();
        let (tx, rx) = channel();
        
        for account in self.aws_account.clone() {
            if let Some(iam) = account.iam {
                for item in iam {
                    let iam_account = IamAccount {
                        account_id: account.account_id.clone(),
                        region: item.region,
                        role_arn: account.role_arn.clone(),
                    };

                    iam_accounts.push(iam_account);
                }
            }
        }

        thread::scope(|scope| {
            for item in iam_accounts {
                let tx = tx.clone();

                scope.spawn(move || -> Result<(), ApplicationErrors> {
                    let config = Config {
                        role_arn: item.role_arn,
                        region: item.region.clone(),
                    };

                    let config = config.generate_config();

                    // generate iam client
                    let iam = Iam::new(
                        &config,
                        item.region,
                    );

                    let client = iam.client();

                    let users: Result<Vec<String>, ApplicationErrors> = iam.list_users(client);

                    let _ = tx.send(users);


                    Ok(())
                });
            }
        });

        drop(tx);

        Ok(())
    }
}
