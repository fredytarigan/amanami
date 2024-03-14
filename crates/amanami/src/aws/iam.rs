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

#[derive(Debug, Clone)]
pub struct UserAccessKeys {
    pub account_id: String,
    pub user_name: String,
    pub access_keys: Option<Vec<AccessKey>>,
}

#[derive(Debug, Clone)]
pub struct AccessKey {
    pub key_id: String,
    pub create_date: String,
    pub status: String,
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

        let mut users = Vec::new();

        let _: Vec<_> = resp
            .users()
            .iter()
            .map(|x| {
                users.push(x.user_name.to_owned());
            })
            .collect();

        Ok(users)
    }

    #[::tokio::main]
    pub async fn list_access_keys(
        &self,
        client: Client,
        account_id: String,
        user_name: String,
    ) -> Result<Option<UserAccessKeys>, ApplicationErrors> {
        let resp = client
            .list_access_keys()
            .user_name(user_name)
            .send()
            .await
            .unwrap();

        let access_keys = resp.access_key_metadata();

        if !access_keys.is_empty() {
            let mut aks: Vec<AccessKey> = vec![];

            for access_key in access_keys {
                let key_id = match access_key.access_key_id.clone() {
                    Some(d) => d,
                    None => String::from(""),
                };

                let create_date = match access_key.create_date {
                    Some(d) => d.to_string(),
                    None => String::from(""),
                };

                let status = match access_key.status.clone() {
                    Some(d) => d.to_string(),
                    None => String::from(""),
                };

                aks.push(AccessKey {
                    key_id,
                    create_date,
                    status,
                })
            }

            let user_name = match access_keys[0].user_name.clone() {
                Some(d) => d,
                None => String::from(""),
            };

            let mut user_access_keys = None;

            if !aks.is_empty() {
                user_access_keys = Some(aks);
            }

            let uak = UserAccessKeys {
                account_id,
                user_name,
                access_keys: user_access_keys,
            };

            return Ok(Some(uak));
        }

        Ok(None)
    }

    #[::tokio::main]
    pub async fn get_access_key_last_used(
        &self,
        client: Client,
        key_id: String,
    ) -> Result<String, ApplicationErrors> {
        let resp = client
            .get_access_key_last_used()
            .access_key_id(&key_id)
            .send()
            .await
            .unwrap();

        let access_key: Vec<_> = resp
            .access_key_last_used
            .into_iter()
            .map(|x| x.last_used_date)
            .collect();

        Ok(access_key[0].to_string())
    }
}
