use async_trait::async_trait;

use crate::login::domain::entities::Credential;

#[async_trait]
pub trait CredentialGateway {
    async fn authorize(&self, credential: &Credential) -> Result<AuthorizationResult, String>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthorizationResult {
    Authorized,
    Unauthorized,
}
