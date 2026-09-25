use std::sync::Arc;

use thiserror::Error;

use crate::application::gateways::{ClearCredentialGateway, ClearCredentialGatewayError};

#[derive(Clone)]
pub struct LogoutUseCase {
    credential_clearer: Arc<dyn ClearCredentialGateway + Send + Sync>,
}

impl LogoutUseCase {
    pub fn new(credential_clearer: Arc<dyn ClearCredentialGateway + Send + Sync>) -> Self {
        Self { credential_clearer }
    }

    pub fn execute(&self) -> Result<(), LogoutError> {
        self.credential_clearer.clear().map_err(
            |ClearCredentialGatewayError::UnableToPerformOperation { details }| {
                LogoutError::UnableToClearCredentials { details }
            },
        )
    }
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum LogoutError {
    #[error("Unable to clear credentials: {details}")]
    UnableToClearCredentials { details: String },
}
