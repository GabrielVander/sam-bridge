use std::sync::Arc;

use async_trait::async_trait;
use authentication::{
    application::gateways::{AuthorizationResult, CredentialGateway, CredentialGatewayError},
    domain::entities::Credential,
};
use sam::client::{SamClient, SamClientError, SamCredentials};

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
        let cred = CredentialWrapper::from(credential);
        let result: Result<(), SamClientError> = self.client.login(&cred.into());

        match result {
            Ok(_) => Ok(AuthorizationResult::Authorized),
            Err(e) => match e {
                SamClientError::RequestError { http_error: _ } => {
                    Err(CredentialGatewayError::UnableToPerformOperation)
                }
                SamClientError::UnexpectedResponse { context: _ } => {
                    Err(CredentialGatewayError::UnableToPerformOperation)
                }
                SamClientError::InvalidCredentials => Ok(AuthorizationResult::Unauthorized),
                SamClientError::SessionExpired => Ok(AuthorizationResult::Unauthorized),
            },
        }
    }
}

struct CredentialWrapper<'a>(&'a Credential);

impl<'a> From<&'a Credential> for CredentialWrapper<'a> {
    fn from(cred: &'a Credential) -> Self {
        Self(cred)
    }
}

impl<'a> From<CredentialWrapper<'a>> for SamCredentials {
    fn from(val: CredentialWrapper<'a>) -> Self {
        SamCredentials {
            login: val.0.email.0.clone(),
            password: val.0.password.0.clone(),
        }
    }
}
