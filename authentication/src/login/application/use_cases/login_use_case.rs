use std::sync::Arc;

use crate::{
    application::gateways::{AuthorizationResult, CredentialGateway, CredentialGatewayError},
    domain::entities::{Credential, Email, Password},
};

#[derive(Clone)]
pub struct LoginUseCase {
    credential_gateway: Arc<dyn CredentialGateway + Send + Sync>,
}

impl LoginUseCase {
    pub fn new(credential_gateway: Arc<dyn CredentialGateway + Send + Sync>) -> Self {
        Self { credential_gateway }
    }

    pub async fn execute(&self, command: LoginCommand) -> Result<(), LoginUseCaseError> {
        let result: Result<AuthorizationResult, CredentialGatewayError> =
            self.credential_gateway.authorize(&command.into()).await;

        match result {
            Ok(auth_result) => match auth_result {
                AuthorizationResult::Authorized => Ok(()),
                AuthorizationResult::Unauthorized => Err(LoginUseCaseError::InvalidEmailOrPassword),
            },
            Err(err) => match err {
                CredentialGatewayError::UnableToPerformOperation => {
                    Err(LoginUseCaseError::UnableToPerformAuthorization)
                }
            },
        }
    }
}

pub struct LoginCommand {
    email: String,
    password: String,
}

impl LoginCommand {
    #[must_use]
    pub const fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginUseCaseError {
    InvalidEmailOrPassword,
    UnableToPerformAuthorization,
}

impl From<LoginCommand> for Credential {
    fn from(command: LoginCommand) -> Self {
        Self::new(Email(command.email), Password(command.password))
    }
}
