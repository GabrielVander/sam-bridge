use std::sync::Arc;

use async_trait::async_trait;
use authentication::{
    application::gateways::{AuthorizationResult, CredentialGateway, CredentialGatewayError},
    domain::entities::Credential,
};

use crate::client::{SamClient, SamClientError, SamCredentials};

pub struct CredentialGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl CredentialGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl CredentialGateway for CredentialGatewaySamImpl {
    async fn authorize(
        &self,
        credential: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError> {
        let result: Result<(), SamClientError> = self.client.login(&credential.into());

        match result {
            Ok(()) => Ok(AuthorizationResult::Authorized),
            Err(e) => match e {
                SamClientError::RequestError { http_error: _ }
                | SamClientError::UnexpectedResponse { context: _ } => {
                    Err(CredentialGatewayError::UnableToPerformOperation)
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
            login: val.email.0.clone(),
            password: val.password.0.clone(),
        }
    }
}
