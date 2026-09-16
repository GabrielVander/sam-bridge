use std::sync::Arc;

use crate::{
    application::gateways::{
        AuthorizationResult, CredentialGateway, CredentialGatewayError, CredentialStore,
    },
    application::use_cases::LoginUseCaseError,
    domain::entities::{Credential, Email, Password},
};

#[derive(Clone)]
pub struct LoginAndRememberCredentialsUseCase {
    credential_gateway: Arc<dyn CredentialGateway + Send + Sync>,
    credential_store: Arc<dyn CredentialStore + Send + Sync>,
}

impl LoginAndRememberCredentialsUseCase {
    #[must_use]
    pub const fn new(
        credential_gateway: Arc<dyn CredentialGateway + Send + Sync>,
        credential_store: Arc<dyn CredentialStore + Send + Sync>,
    ) -> Self {
        Self {
            credential_gateway,
            credential_store,
        }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<(), LoginUseCaseError> {
        let credential: Credential = Credential::new(Email(email), Password(password));

        match self.credential_gateway.authorize(&credential).await {
            Ok(AuthorizationResult::Authorized) => {
                // Remembering credentials is a best-effort side effect: a
                // failed save must never turn an otherwise-successful login
                // into a failure for the user.
                let _ = self.credential_store.save(&credential).await;
                Ok(())
            }
            Ok(AuthorizationResult::Unauthorized) => Err(LoginUseCaseError::InvalidEmailOrPassword),
            Err(CredentialGatewayError::UnableToPerformOperation) => {
                Err(LoginUseCaseError::UnableToPerformAuthorization)
            }
        }
    }
}
