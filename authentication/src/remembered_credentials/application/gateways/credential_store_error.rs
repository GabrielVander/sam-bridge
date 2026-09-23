use thiserror::Error;

#[derive(Error, Debug, Clone, Copy)]
pub enum CredentialStoreError {
    #[error("Unable to perform credential storage operation")]
    UnableToPerformOperation,
}
