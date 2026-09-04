use std::sync::Arc;

use crate::{
    application::gateways::{AuthorizationResult, CredentialGateway},
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

    pub async fn execute(&self, command: LoginCommand) -> LoginResult {
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
    #[must_use]
    pub const fn new(email: String, password: String) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization { context: String },
}

impl From<LoginCommand> for Credential {
    fn from(command: LoginCommand) -> Self {
        Self::new(Email(command.email), Password(command.password))
    }
}

impl From<Result<AuthorizationResult, String>> for LoginResult {
    fn from(result: Result<AuthorizationResult, String>) -> Self {
        match result {
            Ok(auth_result) => auth_result.into(),
            Err(err) => Self::UnableToPerformAuthorization { context: err },
        }
    }
}

impl From<AuthorizationResult> for LoginResult {
    fn from(result: AuthorizationResult) -> Self {
        match result {
            AuthorizationResult::Authorized => Self::Successful,
            AuthorizationResult::Unauthorized => Self::InvalidEmailOrPassword,
        }
    }
}
