use aws_config::SdkConfig;
use aws_sdk_eks::Client;

#[derive(Debug)]
pub struct EKS<'sdk> {
    config: &'sdk SdkConfig,
    cluster_name: String,
}

impl<'sdk> EKS<'sdk> {
    pub fn new(config: &'sdk SdkConfig, cluster_name: String) -> Self {
        Self {
            config,
            cluster_name,
        }
    }

    #[::tokio::main]
    pub async fn get_cluster_version(&self) -> String {
        let client = Client::new(self.config);

        let resp = client
            .describe_cluster()
            .name(self.cluster_name.clone())
            .send()
            .await
            .unwrap();

        let mut cluster_version = vec![];

        let _ = resp
            .cluster()
            .into_iter()
            .map(|x| match &x.version {
                Some(x) => {
                    cluster_version.push(x);
                }
                None => println!("No Data"),
            })
            .collect::<Vec<_>>();

        String::from(cluster_version[0])
    }

    #[::tokio::main]
    pub async fn get_latest_cluster_version(&self) -> String {
        let client = Client::new(self.config);

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
            .into_iter()
            .map(|x| &x.addon_versions)
            .map(|x| match x {
                Some(x) => {
                    for item in x {
                        let cluster_version = match &item.compatibilities {
                            Some(x) => x
                                .into_iter()
                                .map(|x| x.cluster_version())
                                .map(|x| match x {
                                    Some(d) => d,
                                    None => "",
                                })
                                .collect::<Vec<&str>>(),
                            None => continue,
                        };

                        for item in cluster_version {
                            latest_cluster_version.push(item);
                        }
                    }
                }
                None => println!("No Data"),
            })
            .collect::<Vec<_>>();

        // get latest version supported from addons
        latest_cluster_version.sort();
        latest_cluster_version.dedup();
        latest_cluster_version.reverse();

        String::from(latest_cluster_version[0])
    }

    #[::tokio::main]
    pub async fn list_node_groups(&self) -> Option<Vec<String>> {
        let client = Client::new(self.config);

        let resp = client
            .list_nodegroups()
            .cluster_name(self.cluster_name.clone())
            .send()
            .await
            .unwrap();

        if let Some(nodegroups) = resp.nodegroups {
            Some(nodegroups)
        } else {
            None
        }
    }

    #[::tokio::main]
    pub async fn get_nodegroup_auto_scaling_group(&self, name: String) -> String {
        let client = Client::new(self.config);

        let resp = client
            .describe_nodegroup()
            .cluster_name(self.cluster_name.clone())
            .nodegroup_name(name)
            .send()
            .await
            .unwrap();

        let nodegroup: Vec<_> = resp
            .nodegroup()
            .into_iter()
            .map(|x| &x.resources)
            .flat_map(|x| x)
            .map(|x| &x.auto_scaling_groups)
            .flat_map(|x| x)
            .flat_map(|x| x)
            .map(|x| &x.name)
            .map(|x| match x {
                Some(d) => d,
                None => "",
            })
            .collect();

        String::from(nodegroup[0])
    }
}
