use thiserror::Error;

use crate::shared::domain::entities::Credential;

pub trait SaveCredentialGateway: Send + Sync {
    fn save(&self, credential: &Credential) -> Result<(), SaveCredentialGatewayError>;
}

#[derive(Error, Debug, Clone, Copy)]
pub enum SaveCredentialGatewayError {
    #[error("Unable to perform credential storage operation")]
    UnableToPerformOperation,
}
