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

    pub fn execute(
        &self,
        email: String,
        password: String,
    ) -> Result<LoginOutcome, LoginUseCaseError> {
        let credential: Credential = Credential::new(Email::new(email), Password::new(password));

        match self.authorizer.authorize(&credential) {
            Ok(AuthorizationResult::Authorized) => Ok(self.remember(&credential)),
            Ok(AuthorizationResult::Unauthorized) => Err(LoginUseCaseError::InvalidEmailOrPassword),
            Err(error) => Err(LoginUseCaseError::UnableToPerformAuthorization(error)),
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
pub enum LoginUseCaseError {
    InvalidEmailOrPassword,
    UnableToPerformAuthorization(AuthorizationError),
}
