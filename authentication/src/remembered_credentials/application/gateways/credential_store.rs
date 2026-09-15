use async_trait::async_trait;
use thiserror::Error;

use crate::shared::domain::entities::Credential;

#[async_trait]
pub trait CredentialStore {
    async fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError>;

    async fn load(&self) -> Option<Credential>;

    async fn clear(&self) -> Result<(), CredentialStoreError>;
}

#[derive(Error, Debug, Clone, Copy)]
pub enum CredentialStoreError {
    #[error("Unable to perform credential storage operation")]
    UnableToPerformOperation,
}
