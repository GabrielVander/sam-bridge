use thiserror::Error;

use crate::shared::domain::entities::Credential;
use shared_kernel::failure_kind::FailureKind;

pub trait AuthorizeCredentialGateway {
    fn authorize(&self, credential: &Credential)
    -> Result<AuthorizationResult, AuthorizationError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationResult {
    Authorized,
    Unauthorized,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationError {
    #[error("Unable to perform credential authorization: {details}")]
    UnableToPerformOperation { kind: FailureKind, details: String },
}
