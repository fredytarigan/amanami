use aws_config::SdkConfig;
use aws_sdk_ssm::Client;

#[derive(Debug)]
pub struct SSM<'sdk> {
    config: &'sdk SdkConfig,
}

impl<'sdk> SSM<'sdk> {
    pub fn new(config: &'sdk SdkConfig) -> Self {
        Self { config }
    }

    #[::tokio::main]
    pub async fn get_latest_eks_ami_id(&self, eks_version: String) -> String {
        let client = Client::new(self.config);

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
