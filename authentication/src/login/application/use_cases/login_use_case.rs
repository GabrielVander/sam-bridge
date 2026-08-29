use std::sync::Arc;

use crate::{
    application::gateways::{AuthorizationResult, CredentialGateway},
    domain::entities::{Credential, Email, Password},
};

pub struct LoginUseCase {
    credential_gateway: Arc<dyn CredentialGateway>,
}

impl LoginUseCase {
    pub fn new(credential_gateway: Arc<dyn CredentialGateway>) -> Self {
        Self { credential_gateway }
    }

    pub async fn execute(&self, command: LoginCommand) -> LoginUseCaseResult {
        self.credential_gateway
            .authorize(&command.into())
            .await
            .into()
    }
}

pub struct LoginCommand {
    email: String,
    password: String,
}

impl LoginCommand {
    pub fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoginUseCaseResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization { context: String },
}

impl From<LoginCommand> for Credential {
    fn from(command: LoginCommand) -> Self {
        Credential::new(Email(command.email), Password(command.password))
    }
}

impl From<Result<AuthorizationResult, String>> for LoginUseCaseResult {
    fn from(result: Result<AuthorizationResult, String>) -> Self {
        match result {
            Ok(auth_result) => auth_result.into(),
            Err(err) => LoginUseCaseResult::UnableToPerformAuthorization { context: err },
        }
    }
}

impl From<AuthorizationResult> for LoginUseCaseResult {
    fn from(result: AuthorizationResult) -> Self {
        match result {
            AuthorizationResult::Authorized => LoginUseCaseResult::Successful,
            AuthorizationResult::Unauthorized => LoginUseCaseResult::InvalidEmailOrPassword,
        }
    }
}
