use std::sync::Arc;

use thiserror::Error;

use crate::remembered_credentials::application::gateways::CredentialStore;

#[derive(Clone)]
pub struct LogoutUseCase {
    credential_store: Arc<dyn CredentialStore + Send + Sync>,
}

impl LogoutUseCase {
    pub fn new(credential_store: Arc<dyn CredentialStore + Send + Sync>) -> Self {
        Self { credential_store }
    }

    pub fn execute(&self) -> Result<(), LogoutError> {
        self.credential_store
            .clear()
            .map_err(|_| LogoutError::UnableToClearCredentials)
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogoutError {
    #[error("Unable to clear the stored credential")]
    UnableToClearCredentials,
}
