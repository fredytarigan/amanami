use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: AppInfo,
    pub aws: Vec<AWS>,
}

#[derive(Debug, Deserialize)]
pub struct AppInfo {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct AWS {
    pub account_id: String,
    pub role_arn: Option<String>,
    pub eks: Option<Vec<EKS>>,
}

#[derive(Debug, Deserialize)]
pub struct EKS {
    pub cluster_name: String,
    pub region: String,
}

impl AppConfig {
    pub fn new(config_file: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            // add config from files
            .add_source(config::File::with_name(config_file))
            // add config from environment variables
            // environment variables must be prefixed with BLEKI
            .add_source(config::Environment::with_prefix("BLEKI"))
            .build()?;

        config.try_deserialize::<AppConfig>()
    }
}