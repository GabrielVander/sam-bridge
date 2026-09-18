use thiserror::Error;

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

#[derive(Error, Debug, Clone, Copy)]
pub enum CredentialGatewayError {
    #[error("Unable to perform credential authorization")]
    UnableToPerformOperation,
}
