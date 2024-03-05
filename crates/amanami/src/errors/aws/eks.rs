use aws_sdk_eks as eks;
use aws_smithy_runtime_api::client::result::SdkError as sdk_error;
use aws_smithy_runtime_api::http::Response;
use aws_smithy_types::error::display::DisplayErrorContext;
use eks::operation::list_nodegroups::ListNodegroupsError;
use eks::operation::{
    describe_addon::DescribeAddonError, describe_addon_versions::DescribeAddonVersionsError,
    describe_cluster::DescribeClusterError, list_addons::ListAddonsError,
};
use thiserror::Error;

use crate::errors::ApplicationErrors;
use crate::errors::AwsErrors;

fn uplifted_errors<T>(aws_err: sdk_error<T, Response>) -> ApplicationErrors
where
    T: Into<eks::Error> + std::error::Error + 'static,
{
    match aws_err {
        e @ sdk_error::ConstructionFailure(_) => {
            let message = DisplayErrorContext(&e).to_string();
            ApplicationErrors::AwsErrors(AwsErrors::ConstructionFailure(message))
        }

        e @ sdk_error::TimeoutError(_) => {
            let message = DisplayErrorContext(&e).to_string();
            ApplicationErrors::AwsErrors(AwsErrors::TimeoutError(message))
        }

        e @ sdk_error::DispatchFailure(_) => {
            let message = DisplayErrorContext(&e).to_string();
            ApplicationErrors::AwsErrors(AwsErrors::DispatchError(message))
        }

        e @ sdk_error::ResponseError(_) => {
            let message = DisplayErrorContext(&e).to_string();
            ApplicationErrors::AwsErrors(AwsErrors::ResponseError(message))
        }

        sdk_error::ServiceError(service_err) => match service_err.into_err().into() {
            // request errors
            e @ eks::Error::AccessDeniedException(_)
            | e @ eks::Error::BadRequestException(_)
            | e @ eks::Error::ClientException(_)
            | e @ eks::Error::InvalidParameterException(_)
            | e @ eks::Error::InvalidRequestException(_)
            | e @ eks::Error::NotFoundException(_) => {
                let message = DisplayErrorContext(&e).to_string();
                ApplicationErrors::AwsErrors(AwsErrors::AwsEksErrors(AwsEksErrors::RequestError(
                    message,
                )))
            }

            // server errors
            e @ eks::Error::ResourceInUseException(_)
            | e @ eks::Error::ResourceLimitExceededException(_)
            | e @ eks::Error::ResourcePropagationDelayException(_)
            | e @ eks::Error::ServerException(_)
            | e @ eks::Error::ServiceUnavailableException(_) => {
                let message = DisplayErrorContext(&e).to_string();
                ApplicationErrors::AwsErrors(AwsErrors::AwsEksErrors(AwsEksErrors::ServerError(
                    message,
                )))
            }

            // client errors
            e @ eks::Error::ResourceNotFoundException(_)
            | e @ eks::Error::UnsupportedAvailabilityZoneException(_) => {
                let message = DisplayErrorContext(&e).to_string();
                ApplicationErrors::AwsErrors(AwsErrors::AwsEksErrors(AwsEksErrors::ClientError(
                    message,
                )))
            }

            _ => {
                let message =
                    String::from("Encountered an unknown error from AWS EKS in service level");
                ApplicationErrors::AwsErrors(AwsErrors::UnknownErrors(message))
            }
        },

        _ => {
            let message =
                String::from("Encountered an unknown error from AWS EKS in constructor level");
            ApplicationErrors::AwsErrors(AwsErrors::UnknownErrors(message))
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

impl From<eks::error::SdkError<DescribeClusterError>> for ApplicationErrors {
    fn from(err: eks::error::SdkError<DescribeClusterError>) -> Self {
        uplifted_errors(err)
    }
}

impl From<eks::error::SdkError<ListAddonsError>> for ApplicationErrors {
    fn from(err: eks::error::SdkError<ListAddonsError>) -> Self {
        uplifted_errors(err)
    }
}

impl From<eks::error::SdkError<DescribeAddonError>> for ApplicationErrors {
    fn from(err: eks::error::SdkError<DescribeAddonError>) -> Self {
        uplifted_errors(err)
    }
}

impl From<eks::error::SdkError<DescribeAddonVersionsError>> for ApplicationErrors {
    fn from(err: eks::error::SdkError<DescribeAddonVersionsError>) -> Self {
        uplifted_errors(err)
    }
}

impl From<eks::error::SdkError<ListNodegroupsError>> for ApplicationErrors {
    fn from(err: eks::error::SdkError<ListNodegroupsError>) -> Self {
        uplifted_errors(err)
    }
}
