use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub app: App,
    pub aws: Vec<Aws>,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Aws {
    pub account_id: String,
    pub role_arn: Option<String>,
    pub eks: Option<Vec<Eks>>,
}

#[derive(Debug, Deserialize)]
pub struct Eks {
    pub cluster_name: String,
    pub region: String,
}

impl AppConfig {
    /// load config either with files or environment variables
    /// environment variables will take precendes
    pub fn new(config_file: &str) -> Result<AppConfig, ConfigError> {
        let config = Config::builder()
            // load config from files
            .add_source(config::File::with_name(config_file))
            // load config from environment variables
            // environment variables must be prefixed with "AMANAMI"
            .add_source(config::Environment::with_prefix("AMANAMI"))
            .build()?;

        config.try_deserialize::<AppConfig>()
    }
}
