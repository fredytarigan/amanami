use aws_config::SdkConfig;
use aws_types::region::Region;

#[derive(Debug)]
pub struct Config {
    pub region: String,
    pub role_arn: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            region: String::from("us-east-1"),
            role_arn: None,
        }
    }
}

impl Config {
    // pub fn new(region: String, role_arn: Option<String>) -> Self {
    //     Self { region, role_arn }
    // }

    pub fn generate_config(&self) -> SdkConfig {
        match &self.role_arn {
            Some(_) => {
                let config = self.default();

                self.assume_role(config)
            }
            None => self.default(),
        }
    }

    #[::tokio::main]
    async fn default(&self) -> SdkConfig {
        aws_config::from_env()
            .region(Region::new(self.region.clone()))
            .load()
            .await
    }

    #[::tokio::main]
    async fn assume_role(&self, config: SdkConfig) -> SdkConfig {
        // hardcoded session name
        let session_name = "amanami-debug";

        let provider = aws_config::sts::AssumeRoleProvider::builder(self.role_arn.clone().unwrap())
            .configure(&config)
            .region(Region::new(self.region.clone()))
            .session_name(session_name)
            .build()
            .await;

        aws_config::from_env()
            .credentials_provider(provider)
            .region(Region::new(self.region.clone()))
            .load()
            .await
    }
}
