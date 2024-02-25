use aws_config::SdkConfig;
use aws_sdk_autoscaling::Client;
use aws_types::region::Region;

#[derive(Debug)]
pub struct Autoscaling<'sdk> {
    config: &'sdk SdkConfig,
    region: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: String,
    pub version: String,
}

impl<'sdk> Autoscaling<'sdk> {
    pub fn new(config: &'sdk SdkConfig, region: String) -> Self {
        Self { config, region }
    }

    pub fn client(&self) -> Client {
        let config = aws_sdk_autoscaling::config::Builder::from(self.config)
            .region(Region::new(self.region.clone()))
            .build();

        Client::from_conf(config)
    }

    #[::tokio::main]
    pub async fn get_asg_launch_template(&self, client: Client, name: String) -> Response {
        let resp = client
            .describe_auto_scaling_groups()
            .auto_scaling_group_names(name)
            .send()
            .await
            .unwrap();

        let launch_template = resp
            .auto_scaling_groups()
            .iter()
            .flat_map(|x| &x.mixed_instances_policy)
            .flat_map(|x| &x.launch_template)
            .flat_map(|x| &x.launch_template_specification)
            .map(|x| {
                let launch_template_id = match &x.launch_template_id {
                    Some(d) => d,
                    None => "",
                };

                let launch_template_version = match &x.version {
                    Some(v) => v,
                    None => "",
                };

                Response {
                    id: String::from(launch_template_id),
                    version: String::from(launch_template_version),
                }
            })
            .collect::<Vec<Response>>();

        Response {
            id: launch_template[0].id.clone(),
            version: launch_template[0].version.clone(),
        }
    }
}
