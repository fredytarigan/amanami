use aws_types::region::Region;
use aws_config::SdkConfig;

#[derive(Debug)]
pub struct Config {
    region: String,
    role_arn: Option<String>,
}

impl Config {
    pub fn new(region: String, role_arn: Option<String>) -> Self {
        Self {
            region,
            role_arn,
        }
    }

    #[::tokio::main]
    pub async fn get_config(&self) -> SdkConfig {
       aws_config::from_env()
            .region(Region::new(self.region.to_owned())).load().await
    }

    #[::tokio::main]
    pub async fn assume_role(&self, config: SdkConfig, session_name: String) -> SdkConfig {
        let role_arn = self.role_arn.clone().unwrap();

        let provider = aws_config::sts::AssumeRoleProvider::builder(role_arn)
            .configure(&config)
            .region(Region::new(self.region.to_owned()))
            .session_name(session_name)
            .build()
            .await;

        aws_config::from_env()
            .credentials_provider(provider)
            .region(Region::new(self.region.to_owned()))
            .load()
            .await   
    }
}