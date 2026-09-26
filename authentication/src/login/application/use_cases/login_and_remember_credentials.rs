use std::sync::Arc;

use crate::{
    application::gateways::{
        AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, SaveCredentialGateway,
    },
    domain::entities::{Credential, Email, Password},
};

#[derive(Clone)]
pub struct LoginAndRememberCredentialsUseCase {
    authorizer: Arc<dyn AuthorizeCredentialGateway + Send + Sync>,
    credential_storage: Arc<dyn SaveCredentialGateway + Send + Sync>,
}

impl LoginAndRememberCredentialsUseCase {
    #[must_use]
    pub const fn new(
        authorizer: Arc<dyn AuthorizeCredentialGateway + Send + Sync>,
        credential_storage: Arc<dyn SaveCredentialGateway + Send + Sync>,
    ) -> Self {
        Self {
            authorizer,
            credential_storage,
        }
    }

    pub fn execute(&self, email: String, password: String) -> Result<(), LoginUseCaseError> {
        let credential: Credential = Credential::new(Email(email), Password(password));

        match self.authorizer.authorize(&credential) {
            Ok(AuthorizationResult::Authorized) => {
                let _ = self.credential_storage.save(&credential);
                Ok(())
            }
            Ok(AuthorizationResult::Unauthorized) => Err(LoginUseCaseError::InvalidEmailOrPassword),
            Err(error) => Err(LoginUseCaseError::UnableToPerformAuthorization(error)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginUseCaseError {
    InvalidEmailOrPassword,
    UnableToPerformAuthorization(AuthorizationError),
}
