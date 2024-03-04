mod aws;

pub use aws::AwsErrors;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationErrors {
    #[error("{0}")]
    ConfigNotFound(String),

    #[error("Error: {0}")]
    AwsErrors(AwsErrors),
}
