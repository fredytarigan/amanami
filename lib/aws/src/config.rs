use aws_config::SdkConfig;
use aws_types::region::Region;

#[derive(Debug)]
pub struct Config {
    region: String,
    role_arn: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            region: String::from("ap-southeast-1"),
            role_arn: None,
        }
    }
}

impl Config {
    pub fn new(region: String, role_arn: Option<String>) -> Self {
        Self { region, role_arn }
    }

    pub fn generate_config(&self) -> SdkConfig {
        match &self.role_arn {
            Some(role) => {}
            None => {}
        }

        unimplemented!()
    }

    #[::tokio::main]
    async fn default(&self) -> SdkConfig {
        aws_config::from_env()
            .region(Region::new(self.region.clone()))
            .load()
            .await
    }

    #[::tokio::main]
    async fn assume_role(&self) -> SdkConfig {
        unimplemented!()
    }
}
