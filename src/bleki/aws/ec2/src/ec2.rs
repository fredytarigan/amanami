use aws_config::SdkConfig;
use aws_sdk_ec2::Client;

#[derive(Debug)]
pub struct EC2<'sdk> {
    config: &'sdk SdkConfig,
    launch_template: LaunchTemplate,
}

#[derive(Debug)]
struct LaunchTemplate {
    id: String,
    version: String,
}

impl<'sdk> EC2<'sdk> {
    pub fn new(config: &'sdk SdkConfig, id: String, version: String) -> Self {
        let launch_template = LaunchTemplate { id, version };

        Self {
            config,
            launch_template,
        }
    }

    #[::tokio::main]
    pub async fn get_launch_template_ami_id(&self) -> String {
        let client = Client::new(self.config);

        let resp = client
            .describe_launch_template_versions()
            .launch_template_id(self.launch_template.id.clone())
            .versions(self.launch_template.version.clone())
            .send()
            .await
            .unwrap();

        let ami_id: Vec<_> = resp
            .launch_template_versions()
            .into_iter()
            .map(|x| &x.launch_template_data)
            .flat_map(|x| x)
            .map(|x| &x.image_id)
            .map(|x| match &x {
                Some(image) => image,
                None => "",
            })
            .collect();

        String::from(ami_id[0])
    }

    #[::tokio::main]
    pub async fn get_ami_name(&self, ami_id: String) -> String {
        let client = Client::new(self.config);

        let resp = client
            .describe_images()
            .image_ids(ami_id)
            .send()
            .await
            .unwrap();

        let ami_name: Vec<_> = resp
            .images()
            .into_iter()
            .map(|x| &x.name)
            .flat_map(|x| x)
            .collect();

        String::from(ami_name[0])
    }
}
