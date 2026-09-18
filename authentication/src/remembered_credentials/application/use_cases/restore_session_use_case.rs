use std::sync::Arc;

use crate::{
    application::gateways::{AuthorizationResult, CredentialGateway, CredentialGatewayError},
    remembered_credentials::application::gateways::CredentialStore,
};

#[derive(Clone)]
pub struct RestoreSessionUseCase {
    credential_store: Arc<dyn CredentialStore + Send + Sync>,
    credential_gateway: Arc<dyn CredentialGateway + Send + Sync>,
}

impl RestoreSessionUseCase {
    pub fn new(
        credential_store: Arc<dyn CredentialStore + Send + Sync>,
        credential_gateway: Arc<dyn CredentialGateway + Send + Sync>,
    ) -> Self {
        Self {
            credential_store,
            credential_gateway,
        }
    }

    #[must_use]
    pub fn execute(&self) -> RestoreSessionResult {
        let Some(credential) = self.credential_store.load() else {
            return RestoreSessionResult::NoStoredCredentials;
        };

        match self.credential_gateway.authorize(&credential) {
            Ok(AuthorizationResult::Authorized) => RestoreSessionResult::Restored,
            Ok(AuthorizationResult::Unauthorized) => {
                let _ = self.credential_store.clear();
                RestoreSessionResult::CredentialsRejected
            }
            Err(CredentialGatewayError::UnableToPerformOperation) => {
                RestoreSessionResult::UnableToPerformOperation
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionResult {
    Restored,
    NoStoredCredentials,
    CredentialsRejected,
    UnableToPerformOperation,
}
