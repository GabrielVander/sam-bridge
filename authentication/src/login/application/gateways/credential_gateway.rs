use thiserror::Error;

use crate::shared::application::failure_kind::FailureKind;
use crate::shared::domain::entities::Credential;
pub trait CredentialGateway {
    fn authorize(
        &self,
        credential: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationResult {
    Authorized,
    Unauthorized,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum CredentialGatewayError {
    #[error("Unable to perform credential authorization: {details}")]
    UnableToPerformOperation { kind: FailureKind, details: String },
}
