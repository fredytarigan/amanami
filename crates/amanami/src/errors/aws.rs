use thiserror::Error;

#[derive(Debug, Error)]
pub enum AWSErrors {
    #[error("")]
    ConstructionFailure,
    #[error("")]
    TimeoutError,
    #[error("")]
    DispatchError,
    #[error("")]
    ResponseError,
    #[error("")]
    ServiceError,
    #[error("")]
    ConnectorError,
}

// impl AWSErrors {
//     fn message(&self) -> &str {
//         match self {
//             Self::ConstructionFailure => "",
//             Self::TimeoutError => "",
//             Self::DispatchError => "",
//             Self::ResponseError => "",
//             Self::ServiceError => "",
//             Self::ConnectorError => "",
//         }
//     }
// }
