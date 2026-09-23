use std::sync::Arc;

use thiserror::Error;

use crate::{
    application::gateways::{
        AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway,
        ClearCredentialGateway, FailureKind, LoadCredentialGateway,
    },
    domain::entities::Credential,
};

#[derive(Clone)]
pub struct RestoreSessionUseCase {
    credential_loader: Arc<dyn LoadCredentialGateway + Send + Sync>,
    credential_eraser: Arc<dyn ClearCredentialGateway + Send + Sync>,
    authorizer: Arc<dyn AuthorizeCredentialGateway + Send + Sync>,
}

impl RestoreSessionUseCase {
    pub fn new(
        credential_loader: Arc<dyn LoadCredentialGateway + Send + Sync>,
        credential_eraser: Arc<dyn ClearCredentialGateway + Send + Sync>,
        authorizer: Arc<dyn AuthorizeCredentialGateway + Send + Sync>,
    ) -> Self {
        Self {
            credential_loader,
            credential_eraser,
            authorizer,
        }
    }

    pub fn execute(&self) -> Result<RestoreSessionOutcome, RestoreSessionError> {
        let Some(credential): Option<Credential> = self.credential_loader.load() else {
            return Ok(RestoreSessionOutcome::NoStoredCredentials);
        };

        match self.authorizer.authorize(&credential) {
            Ok(AuthorizationResult::Authorized) => Ok(RestoreSessionOutcome::Restored),
            Ok(AuthorizationResult::Unauthorized) => {
                self.credential_eraser
                    .clear()
                    .map_err(|_| RestoreSessionError::UnableToClearRejectedCredentials)?;
                Ok(RestoreSessionOutcome::CredentialsRejected)
            }
            Err(AuthorizationError::UnableToPerformOperation { kind, details }) => {
                Err(RestoreSessionError::UnableToPerformOperation { kind, details })
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionOutcome {
    Restored,
    NoStoredCredentials,
    CredentialsRejected,
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionError {
    #[error("Unable to perform credential authorization: {details}")]
    UnableToPerformOperation { kind: FailureKind, details: String },
    #[error("Unable to clear rejected credentials")]
    UnableToClearRejectedCredentials,
}
