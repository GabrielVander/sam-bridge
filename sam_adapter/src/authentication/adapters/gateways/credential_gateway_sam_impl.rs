use std::sync::Arc;

use async_trait::async_trait;
use authentication::{
    application::gateways::{AuthorizationResult, CredentialGateway},
    domain::entities::Credential,
};
use sam::client::{SamClient, SamCredentials};

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
    async fn authorize(&self, credential: &Credential) -> Result<AuthorizationResult, String> {
        let cred = CredentialWrapper::from(credential);
        let result: Result<(), String> = self.client.login(&cred.into());

        match result {
            Ok(_) => Ok(AuthorizationResult::Authorized),
            Err(e) if e.as_str() == "Invalid credentials" => Ok(AuthorizationResult::Unauthorized),
            Err(e) => Err(e),
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
