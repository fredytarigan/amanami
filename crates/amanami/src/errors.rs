mod aws;

pub use aws::AWSErrors;

#[derive(Debug)]
pub enum ApplicationErrors {
    AWSErrors(AWSErrors),
}
