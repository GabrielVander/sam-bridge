use thiserror::Error;

use crate::shared::domain::entities::Credential;
pub trait CredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError>;

    fn load(&self) -> Option<Credential>;

    fn clear(&self) -> Result<(), CredentialStoreError>;
}

#[derive(Error, Debug, Clone, Copy)]
pub enum CredentialStoreError {
    #[error("Unable to perform credential storage operation")]
    UnableToPerformOperation,
}
