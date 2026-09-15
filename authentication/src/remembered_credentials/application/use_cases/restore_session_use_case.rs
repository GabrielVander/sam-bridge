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

    pub async fn execute(&self) -> RestoreSessionResult {
        let Some(credential) = self.credential_store.load().await else {
            return RestoreSessionResult::NoStoredCredentials;
        };

        match self.credential_gateway.authorize(&credential).await {
            Ok(AuthorizationResult::Authorized) => RestoreSessionResult::Restored,
            Ok(AuthorizationResult::Unauthorized) => {
                // The stored credentials are genuinely no longer valid — self-heal
                // rather than keep retrying them on every future launch. A plain
                // transport/network failure (below) is deliberately NOT treated
                // this way: it would otherwise forget a valid saved login on a
                // transient blip.
                let _ = self.credential_store.clear().await;
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
