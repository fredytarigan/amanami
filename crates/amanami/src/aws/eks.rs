use super::autoscaling::Autoscaling;
use super::ec2::Ec2;
use super::ssm::Ssm;
use aws_config::SdkConfig;
use aws_sdk_eks::Client;
use aws_types::region::Region;

#[derive(Debug)]
pub struct Eks<'sdk> {
    config: &'sdk SdkConfig,
    cluster_name: String,
    region: String,
}

#[derive(Debug, Clone)]
pub struct EksCluster {
    pub account_id: String,
    pub cluster_name: String,
    pub region: String,
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EksNodeGroup {
    pub account_id: String,
    pub cluster_name: String,
    pub region: String,
    pub role_arn: Option<String>,
    pub node_name: String,
}

#[derive(Debug)]
pub struct Response {
    pub cluster_response: ClusterResponse,
    pub nodegroup_response: Vec<NodegroupResponse>,
}

#[derive(Debug)]
pub struct ClusterResponse {
    pub cluster_version: String,
    pub latest_cluster_version: String,
    pub upgrade_available: String,
}

#[derive(Debug)]
pub struct NodegroupResponse {
    pub node_name: String,
    pub ami_name: String,
    pub latest_ami_name: String,
    pub upgrade_available: String,
}

impl<'sdk> Eks<'sdk> {
    pub fn new(config: &'sdk SdkConfig, cluster_name: String, region: String) -> Self {
        Self {
            config,
            cluster_name,
            region,
        }
    }

    pub fn client(&self) -> Client {
        let config = aws_sdk_eks::config::Builder::from(self.config)
            .region(Region::new(self.region.clone()))
            .build();

        Client::from_conf(config)
    }

    pub fn get_cluster_update(&self, client: &Client) -> ClusterResponse {
        // get cluster update availability
        let cluster_version = self.get_cluster_version(client);
        let latest_cluster_version = self.get_latest_cluster_version(client);

        let mut upgrade_available: String = String::from("Not Available");

        if cluster_version != latest_cluster_version {
            upgrade_available = String::from("Available");
        }

        ClusterResponse {
            cluster_version: cluster_version.clone(),
            latest_cluster_version,
            upgrade_available,
        }
    }

    pub fn get_nodegroup_update(&self, client: &Client, name: String) -> NodegroupResponse {
        let cluster_version = self.get_cluster_version(client);
        let asg_name = self.get_nodegroup_asg(client, name.clone());

        let asg = Autoscaling::new(self.config, self.region.clone());
        let asg_client = asg.client();
        let launch_template = asg.get_asg_launch_template(asg_client, asg_name);

        let ssm = Ssm::new(self.config, self.region.clone());
        let ssm_client = ssm.client();
        let latest_ami_id = ssm.get_latest_eks_ami_id(&ssm_client, cluster_version.clone());

        let ec2 = Ec2::new(
            self.config,
            self.region.clone(),
            launch_template.id.clone(),
            launch_template.version.clone(),
        );
        let ec2_client = ec2.client();
        let ami_id = ec2.get_launch_template_ami_id(&ec2_client);
        let ami_name = ec2.get_ami_name(&ec2_client, ami_id);
        let latest_ami_name = ec2.get_ami_name(&ec2_client, latest_ami_id);

        let mut upgrade_available: String = String::from("Not Available");

        if ami_name != latest_ami_name {
            upgrade_available = String::from("Available");
        }

        NodegroupResponse {
            node_name: name,
            ami_name,
            latest_ami_name,
            upgrade_available,
        }
    }

    #[::tokio::main]
    async fn get_cluster_version(&self, client: &Client) -> String {
        let resp = client
            .describe_cluster()
            .name(self.cluster_name.clone())
            .send()
            .await;

        match resp {
            Ok(_) => {}
            Err(e) => {
                println!("{}", e);
            }
        }

        // let cluster_version: Vec<_> = resp
        //     .cluster()
        //     .into_iter()
        //     .flat_map(|x| &x.version)
        //     .collect();

        // String::from(cluster_version[0])

        unimplemented!()
    }

    #[::tokio::main]
    async fn get_latest_cluster_version(&self, client: &Client) -> String {
        let resp = client
            .describe_addon_versions()
            // hardcoded to check kube-proxy latest version
            // as kube-proxy addons version should be linear
            // with cluster latest version
            .addon_name("kube-proxy")
            .send()
            .await
            .unwrap();

        let mut latest_cluster_version = Vec::new();

        let _ = resp
            .addons()
            .iter()
            .flat_map(|x| &x.addon_versions)
            .map(|x| {
                for item in x {
                    let cluster_version = match &item.compatibilities {
                        Some(x) => x
                            .iter()
                            .filter_map(|x| x.cluster_version())
                            .collect::<Vec<&str>>(),
                        None => continue,
                    };

                    for item in cluster_version {
                        latest_cluster_version.push(item);
                    }
                }
            })
            .collect::<Vec<_>>();

        // get latest version supported from addons
        latest_cluster_version.sort();
        latest_cluster_version.dedup();
        latest_cluster_version.reverse();

        String::from(latest_cluster_version[0])
    }

    #[::tokio::main]
    pub async fn list_nodegroups(&self, client: &Client) -> Option<Vec<String>> {
        let resp = client
            .list_nodegroups()
            .cluster_name(self.cluster_name.clone())
            .send()
            .await
            .unwrap();

        resp.nodegroups
    }

    #[::tokio::main]
    async fn get_nodegroup_asg(&self, client: &Client, name: String) -> String {
        let resp = client
            .describe_nodegroup()
            .cluster_name(self.cluster_name.clone())
            .nodegroup_name(name)
            .send()
            .await
            .unwrap();

        let autoscaling_group: Vec<_> = resp
            .nodegroup()
            .into_iter()
            .flat_map(|x| &x.resources)
            .flat_map(|x| &x.auto_scaling_groups)
            .flatten()
            .map(|x| &x.name)
            .map(|x| match x {
                Some(d) => d,
                None => "",
            })
            .collect();

        String::from(autoscaling_group[0])
    }
}
