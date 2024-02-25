use aws_config::SdkConfig;
use aws_sdk_ssm::Client;
use aws_types::region::Region;

#[derive(Debug)]
pub struct Ssm<'sdk> {
    config: &'sdk SdkConfig,
    region: String,
}

impl<'sdk> Ssm<'sdk> {
    pub fn new(config: &'sdk SdkConfig, region: String) -> Self {
        Self { config, region }
    }

    pub fn client(&self) -> Client {
        let config = aws_sdk_ssm::config::Builder::from(self.config)
            .region(Region::new(self.region.clone()))
            .build();

        Client::from_conf(config)
    }

    #[::tokio::main]
    pub async fn get_latest_eks_ami_id(&self, client: &Client, eks_version: String) -> String {
        let resp = client
            .get_parameter()
            .name(format!(
                "/aws/service/eks/optimized-ami/{}/amazon-linux-2/recommended/image_id",
                eks_version
            ))
            .send()
            .await
            .unwrap();

        let ami_id: Vec<_> = resp
            .parameter()
            .into_iter()
            .map(|x| &x.value)
            .flat_map(|x| x)
            .collect();

        String::from(ami_id[0])
    }
}
