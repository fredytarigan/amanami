use aws_config::SdkConfig;
use aws_sdk_iam::Client;
use aws_types::region::Region;

use crate::errors::ApplicationErrors;

#[derive(Debug)]
pub struct Iam<'sdk> {
    config: &'sdk SdkConfig,
    region: String,
}

#[derive(Debug, Clone)]
pub struct IamAccount {
    pub account_id: String,
    pub region: String,
    pub role_arn: Option<String>,
}

impl<'sdk> Iam<'sdk> {
    pub fn new(config: &'sdk SdkConfig, region: String) -> Self {
        Self { config, region }
    }

    pub fn client(&self) -> Client {
        let config = aws_sdk_iam::config::Builder::from(self.config)
            .region(Region::new(self.region.clone()))
            .build();

        Client::from_conf(config)
    }

    #[::tokio::main]
    pub async fn list_users(&self, client: Client) -> Result<Vec<String>, ApplicationErrors> {
        let resp = client.list_users().send().await.unwrap();

        let users = resp.users();

        println!("{:?}", users);

        unimplemented!()
    }

    #[::tokio::main]
    pub async fn list_access_keys(&self, client: Client) -> Result<Vec<String>, ApplicationErrors> {
        let resp = client.list_access_keys().send().await.unwrap();

        let access_keys = resp.access_key_metadata();

        println!("{:?}", access_keys);

        unimplemented!()
    }
}
