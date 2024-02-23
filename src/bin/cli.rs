use colored::*;
use comfy_table::{Attribute, Cell, CellAlignment, Color};
use std::env;

use app_aws_autoscaling::Autoscaling;
use app_aws_config::Config as aws_config;
use app_aws_ec2::EC2;
use app_aws_eks::EKS;
use app_aws_ssm::SSM;
use app_config::AppConfig;
use app_output_table::OutputTable;

fn load_config(path: &str) -> AppConfig {
    match AppConfig::new(path) {
        Ok(config) => config,
        Err(e) => {
            eprintln!(
                "{} Unable to prepare configuration with error: {}",
                "Error".red().bold(),
                e
            );
            std::process::exit(1);
        }
    }
}

fn main() {
    // set current directory
    let config_file = env::var("CONFIG_FILE").unwrap_or(String::from("./config/config.yaml"));

    let cfg = load_config(&config_file);

    println!(
        "{} {}",
        "App Name:".bold().bright_green(),
        cfg.app.name.bold().bright_green()
    );
    println!(
        "{} {}",
        "App Version:".bold().bright_green(),
        cfg.app.version.bold().bright_green()
    );
    println!("");

    let mut eks_rows: Vec<Vec<Cell>> = vec![];
    let mut nodegroup_rows: Vec<Vec<Cell>> = vec![];

    for item in cfg.aws {
        let eks_data = match item.eks {
            Some(d) => d,
            None => continue,
        };

        for cluster in eks_data {
            let aws = aws_config::new(cluster.region.clone(), item.role_arn.clone());
            let mut aws_config = aws.get_config();

            aws_config = match item.role_arn.clone() {
                Some(_) => aws.assume_role(aws_config, String::from("bleki-app-debug")),

                None => aws_config,
            };

            let eks = EKS::new(&aws_config, String::from(&cluster.cluster_name));

            let eks_cluster_version = eks.get_cluster_version();
            let latest_eks_cluster_version = eks.get_latest_cluster_version();

            let mut upgrade_available = Cell::new("Not Available")
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::Black);

            if eks_cluster_version != latest_eks_cluster_version {
                upgrade_available = Cell::new("Available")
                    .set_alignment(CellAlignment::Center)
                    .add_attributes(vec![Attribute::Bold, Attribute::SlowBlink])
                    .fg(Color::Green);
            }

            let nodegroups = match eks.list_node_groups() {
                Some(data) => data,
                None => vec![],
            };

            for node in nodegroups {
                let auto_scaling_group_id = eks.get_nodegroup_auto_scaling_group(node.clone());

                let autoscaling = Autoscaling::new(&aws_config, auto_scaling_group_id);
                let launch_template = autoscaling.get_autoscaling_launch_template();

                let ssm = SSM::new(&aws_config);
                let latest_ami_id = ssm.get_latest_eks_ami_id(eks_cluster_version.clone());

                let ec2 = EC2::new(&aws_config, launch_template.id, launch_template.version);
                let ami_id = ec2.get_launch_template_ami_id();
                let ami_name = ec2.get_ami_name(ami_id);
                let latest_ami_name = ec2.get_ami_name(latest_ami_id);

                let mut upgrade_available = Cell::new("Not Available")
                    .set_alignment(CellAlignment::Center)
                    .add_attribute(Attribute::Bold)
                    .fg(Color::Black);

                if ami_name != latest_ami_name {
                    upgrade_available = Cell::new("Available")
                        .set_alignment(CellAlignment::Center)
                        .add_attributes(vec![Attribute::Bold, Attribute::SlowBlink])
                        .fg(Color::Green);
                };

                let nodegroup_data = vec![
                    Cell::new(item.account_id.clone()),
                    Cell::new(cluster.cluster_name.clone()),
                    Cell::new(cluster.region.clone()).set_alignment(CellAlignment::Center),
                    Cell::new(node.clone()),
                    Cell::new(ami_name.clone()).set_alignment(CellAlignment::Center),
                    Cell::new(latest_ami_name.clone()).set_alignment(CellAlignment::Center),
                    upgrade_available,
                ];

                nodegroup_rows.push(nodegroup_data);
            }

            let eks_data = vec![
                Cell::new(item.account_id.clone()),
                Cell::new(cluster.cluster_name.clone()),
                Cell::new(cluster.region.clone()).set_alignment(CellAlignment::Center),
                Cell::new(eks_cluster_version.clone()).set_alignment(CellAlignment::Center),
                Cell::new(latest_eks_cluster_version.clone()).set_alignment(CellAlignment::Center),
                upgrade_available,
            ];

            eks_rows.push(eks_data);
        }
    }

    let eks_table = OutputTable::new(
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
        eks_rows,
    );

    let eks_nodegroup_table = OutputTable::new(
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
            Cell::new(String::from("Node Group Name"))
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkMagenta),
            Cell::new(String::from("AMI Release Version"))
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkMagenta),
            Cell::new(String::from("AMI Release Latest Version"))
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkMagenta),
            Cell::new(String::from("Upgrade Available"))
                .set_alignment(CellAlignment::Center)
                .add_attribute(Attribute::Bold)
                .fg(Color::DarkMagenta),
        ],
        nodegroup_rows,
    );

    println!("{}", "EKS Cluster Details: ".bold().yellow());
    eks_table.display_output();

    println!("");

    println!("{}", "EKS Node Group Details: ".bold().yellow());
    eks_nodegroup_table.display_output();

    println!("");
}
