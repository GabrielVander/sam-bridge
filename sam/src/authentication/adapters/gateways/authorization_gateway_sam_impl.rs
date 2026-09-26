use authentication::{
    application::gateways::{AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway},
    domain::entities::Credential,
};
use std::sync::Arc;

use crate::client::{SamClient, SamClientError, SamCredentials};
use crate::diagnostics::{error_chain, failure_kind};

pub struct AuthorizationGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl AuthorizationGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}
impl AuthorizeCredentialGateway for AuthorizationGatewaySamImpl {
    fn authorize(
        &self,
        credential: &Credential,
    ) -> Result<AuthorizationResult, AuthorizationError> {
        match self.client.login(&credential.into()) {
            Ok(()) => Ok(AuthorizationResult::Authorized),
            Err(SamClientError::InvalidCredentials | SamClientError::SessionExpired) => {
                Ok(AuthorizationResult::Unauthorized)
            }
            Err(error) => Err(AuthorizationError::UnableToPerformOperation {
                kind: failure_kind(&error),
                details: error_chain(&error),
            }),
        }
    }
}

impl From<&Credential> for SamCredentials {
    fn from(val: &Credential) -> Self {
        Self {
            login: val.email().as_str().to_owned(),
            password: val.password().as_str().to_owned(),
        }
    }
}
