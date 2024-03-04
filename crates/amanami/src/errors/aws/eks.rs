use aws_sdk_eks as eks;
use aws_smithy_runtime_api::client::result::SdkError as sdk_error;
use aws_smithy_runtime_api::http::Response;
use aws_smithy_types::error::display::DisplayErrorContext;
use eks::operation::list_clusters::ListClustersError;
use thiserror::Error;

use super::AwsErrors;

fn uplifted_errors<T>(aws_err: sdk_error<T, Response>) -> AwsErrors
where
    T: Into<eks::Error> + std::error::Error + 'static,
{
    match aws_err {
        e @ sdk_error::ConstructionFailure(_) => {
            let message = format!("{}", DisplayErrorContext(&e).to_string());
            AwsErrors::ConstructionFailure(message)
        }

        e @ sdk_error::TimeoutError(_) => {
            let message = format!("{}", DisplayErrorContext(&e).to_string());
            AwsErrors::TimeoutError(message)
        }

        e @ sdk_error::DispatchFailure(_) => {
            let message = format!("{}", DisplayErrorContext(&e).to_string());
            AwsErrors::DispatchError(message)
        }

        e @ sdk_error::ResponseError(_) => {
            let message = format!("{}", DisplayErrorContext(&e).to_string());
            AwsErrors::ResponseError(message)
        }

        sdk_error::ServiceError(service_err) => match service_err.into_err().into() {
            // request errors
            e @ eks::Error::AccessDeniedException(_)
            | e @ eks::Error::BadRequestException(_)
            | e @ eks::Error::ClientException(_)
            | e @ eks::Error::InvalidParameterException(_)
            | e @ eks::Error::InvalidRequestException(_)
            | e @ eks::Error::NotFoundException(_) => {
                let message = format!("{}", DisplayErrorContext(&e).to_string());
                AwsErrors::AwsEksErrors(AwsEksErrors::RequestError(message))
            }

            // server errors
            e @ eks::Error::ResourceInUseException(_)
            | e @ eks::Error::ResourceLimitExceededException(_)
            | e @ eks::Error::ResourcePropagationDelayException(_)
            | e @ eks::Error::ServerException(_)
            | e @ eks::Error::ServiceUnavailableException(_) => {
                let message = format!("{}", DisplayErrorContext(&e).to_string());
                AwsErrors::AwsEksErrors(AwsEksErrors::ServerError(message))
            }

            // client errors
            e @ eks::Error::ResourceNotFoundException(_)
            | e @ eks::Error::UnsupportedAvailabilityZoneException(_) => {
                let message = format!("{}", DisplayErrorContext(&e).to_string());
                AwsErrors::AwsEksErrors(AwsEksErrors::ClientError(message))
            }

            _ => {
                let message =
                    String::from("Encountered an unknown error from AWS EKS in service level");
                AwsErrors::UnknownErrors(message)
            }
        },

        _ => {
            let message =
                String::from("Encountered an unknown error from AWS EKS in constructor level");
            AwsErrors::UnknownErrors(message)
        }
    }
}

#[derive(Debug, Error)]
pub enum AwsEksErrors {
    #[error("{0}")]
    RequestError(String),

    #[error("{0}")]
    ServerError(String),

    #[error("{0}")]
    ClientError(String),
}

impl From<eks::error::SdkError<ListClustersError>> for AwsErrors {
    fn from(err: eks::error::SdkError<ListClustersError>) -> Self {
        uplifted_errors(err)
    }
}
