use std::sync::Arc;

use authentication::application::use_cases::{LoginCommand, LoginUseCase};
use flutter_rust_bridge::frb;
use sam::client::SamClientImpl;
use sam_adapter::adapters::gateways::CredentialGatewaySamImpl;

use crate::infra::Config;

pub use authentication::application::use_cases::LoginResult;

pub struct Application {
    login_use_case: LoginUseCase,
}

impl Application {
    pub(crate) fn new(config: Config) -> Self {
        let sam_client: Arc<SamClientImpl> = Arc::new(
            SamClientImpl::new(config.sam_client_base_url).expect("Sam client initilization"),
        );

        let sam_credential_gateway: Arc<CredentialGatewaySamImpl> =
            Arc::new(CredentialGatewaySamImpl::new(sam_client.clone()));

        let login_use_case: LoginUseCase = LoginUseCase::new(sam_credential_gateway.clone());

        Self { login_use_case }
    }

    pub async fn login(&self, email: String, password: String) -> LoginResult {
        self.login_use_case
            .execute(LoginCommand::new(email.into(), password.into()))
            .await
    }
}

#[frb(mirror(LoginResult))]
pub enum LoginResultMirror {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization { context: String },
}
