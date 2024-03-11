mod eks;

pub use eks::AwsEksErrors;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AwsErrors {
    #[error("{0}")]
    ConstructionFailure(String),

    #[error("{0}")]
    TimeoutError(String),

    #[error("{0}")]
    DispatchError(String),

    #[error("{0}")]
    ResponseError(String),

    #[error("{0}")]
    AwsEksErrors(#[source] AwsEksErrors),

    #[error("{0}")]
    UnknownErrors(String),
}
