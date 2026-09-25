use thiserror::Error;

pub trait ClearCredentialGateway {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError>;
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ClearCredentialGatewayError {
    #[error("Unable to perform credential clear operation: {details}")]
    UnableToPerformOperation { details: String },
}
