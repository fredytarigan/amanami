use aws_config::SdkConfig;
use aws_sdk_autoscaling::Client;

#[derive(Debug)]
pub struct Autoscaling<'sdk> {
    config: &'sdk SdkConfig,
    id: String,
}

#[derive(Debug)]
pub struct LaunchTemplate {
    pub id: String,
    pub version: String,
}

impl<'sdk> Autoscaling<'sdk> {
    pub fn new(config: &'sdk SdkConfig, id: String) -> Self {
        Self { config, id }
    }

    #[::tokio::main]
    pub async fn get_autoscaling_launch_template(&self) -> LaunchTemplate {
        let client = Client::new(self.config);

        let resp = client
            .describe_auto_scaling_groups()
            .auto_scaling_group_names(self.id.clone())
            .send()
            .await
            .unwrap();

        let launch_template: Vec<LaunchTemplate> = resp
            .auto_scaling_groups()
            .into_iter()
            .map(|x| &x.mixed_instances_policy)
            .flat_map(|x| x)
            .map(|x| &x.launch_template)
            .flat_map(|x| x)
            .map(|x| &x.launch_template_specification)
            .flat_map(|x| x)
            .map(|x| {
                let launch_template_id = match &x.launch_template_id {
                    Some(d) => d,
                    None => "",
                };

                let launch_template_version = match &x.version {
                    Some(v) => v,
                    None => "",
                };

                LaunchTemplate {
                    id: String::from(launch_template_id),
                    version: String::from(launch_template_version),
                }
            })
            .collect();

        LaunchTemplate {
            id: launch_template[0].id.clone(),
            version: launch_template[0].version.clone(),
        }
    }
}
