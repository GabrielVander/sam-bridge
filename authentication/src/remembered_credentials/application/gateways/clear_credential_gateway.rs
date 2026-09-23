use thiserror::Error;

pub trait ClearCredentialGateway {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError>;
}

#[derive(Error, Debug, Clone, Copy)]
pub enum ClearCredentialGatewayError {
    #[error("Unable to perform credential clear operation")]
    UnableToPerformOperation,
}
