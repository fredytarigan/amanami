use std::error::Error;
use std::fmt::Display;
use std::fmt::{Formatter, Result};

#[derive(Debug)]
pub enum AWSErrors {
    ConstructionFailure,
    TimeoutError,
    DispatchError,
    ResponseError,
    ServiceError,
}

impl Display for AWSErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "{}", self.message())
    }
}

impl Error for AWSErrors {}

impl AWSErrors {
    fn message(&self) -> &str {
        match self {
            Self::ConstructionFailure => "",
            Self::TimeoutError => "",
            Self::DispatchError => "",
            Self::ResponseError => "",
            Self::ServiceError => "",
        }
    }
}
