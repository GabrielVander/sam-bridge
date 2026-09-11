use async_trait::async_trait;
use thiserror::Error;

use crate::login::domain::entities::Credential;

#[async_trait]
pub trait CredentialGateway {
    async fn authorize(
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
