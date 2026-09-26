use authentication::{
    application::gateways::{
        AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, FailureKind,
    },
    domain::entities::Credential,
};
use std::sync::Arc;

use crate::client::{SamClient, SamClientError, SamCredentials};
use crate::diagnostics::error_chain;

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
        let result: Result<(), SamClientError> = self.client.login(&credential.into());

        match result {
            Ok(()) => Ok(AuthorizationResult::Authorized),
            Err(e) => match e {
                SamClientError::RequestError { .. } => {
                    Err(AuthorizationError::UnableToPerformOperation {
                        kind: FailureKind::Transient,
                        details: error_chain(&e),
                    })
                }
                SamClientError::UnexpectedResponse { .. } => {
                    Err(AuthorizationError::UnableToPerformOperation {
                        kind: FailureKind::Unexpected,
                        details: error_chain(&e),
                    })
                }
                SamClientError::InvalidCredentials | SamClientError::SessionExpired => {
                    Ok(AuthorizationResult::Unauthorized)
                }
            },
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
