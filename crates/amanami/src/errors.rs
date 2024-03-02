mod aws;

pub use aws::AWSErrors;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationErrors {
    #[error("Error: {0}")]
    AWSErrors(AWSErrors),
}
