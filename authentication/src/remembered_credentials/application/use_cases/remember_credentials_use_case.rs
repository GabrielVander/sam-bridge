use std::sync::Arc;

use thiserror::Error;

use crate::{
    remembered_credentials::application::gateways::CredentialStore,
    shared::domain::entities::Credential,
};

#[derive(Clone)]
pub struct RememberCredentialsUseCase {
    credential_store: Arc<dyn CredentialStore + Send + Sync>,
}

impl RememberCredentialsUseCase {
    pub fn new(credential_store: Arc<dyn CredentialStore + Send + Sync>) -> Self {
        Self { credential_store }
    }

    pub async fn execute(&self, credential: Credential) -> Result<(), RememberCredentialsError> {
        self.credential_store
            .save(&credential)
            .await
            .map_err(|_| RememberCredentialsError::UnableToPersist)
    }
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum RememberCredentialsError {
    #[error("Unable to persist the credential")]
    UnableToPersist,
}
