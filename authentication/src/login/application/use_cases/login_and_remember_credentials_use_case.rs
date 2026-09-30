use std::sync::Arc;

use crate::{
    application::gateways::{
        AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, SaveCredentialGateway,
        SaveCredentialGatewayError,
    },
    domain::entities::{Credential, Email, Password},
};

#[derive(Clone)]
pub struct LoginAndRememberCredentialsUseCase {
    authorizer: Arc<dyn AuthorizeCredentialGateway>,
    credential_storage: Arc<dyn SaveCredentialGateway>,
}

impl LoginAndRememberCredentialsUseCase {
    #[must_use]
    pub const fn new(
        authorizer: Arc<dyn AuthorizeCredentialGateway>,
        credential_storage: Arc<dyn SaveCredentialGateway>,
    ) -> Self {
        Self {
            authorizer,
            credential_storage,
        }
    }

    pub fn execute(&self, email: String, password: String) -> Result<LoginOutcome, LoginError> {
        let credential: Credential = Credential::new(Email::new(email), Password::new(password));

        match self.authorizer.authorize(&credential) {
            Ok(AuthorizationResult::Authorized) => Ok(self.remember(&credential)),
            Ok(AuthorizationResult::Unauthorized) => Err(LoginError::InvalidEmailOrPassword),
            Err(error) => Err(LoginError::UnableToPerformAuthorization(error)),
        }
    }

    fn remember(&self, credential: &Credential) -> LoginOutcome {
        match self.credential_storage.save(credential) {
            Ok(()) => LoginOutcome::LoggedIn,
            Err(error) => LoginOutcome::LoggedInWithoutRemembering(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginOutcome {
    LoggedIn,
    LoggedInWithoutRemembering(SaveCredentialGatewayError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginError {
    InvalidEmailOrPassword,
    UnableToPerformAuthorization(AuthorizationError),
}
