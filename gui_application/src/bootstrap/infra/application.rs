use std::sync::Arc;

use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseError};
use sam::{
    authentication::adapters::gateways::CredentialGatewaySamImpl, client::SamClientImpl,
    http::SamOperations,
};

use crate::infra::Config;

pub struct Application {
    login_use_case: LoginUseCase,
}

impl Application {
    pub(crate) fn new(config: &Config) -> Result<Self, String> {
        let reqwest_client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .map_err(|e| e.to_string())?;

        let sam_operations = SamOperations::new(
            reqwest_client,
            &config.sam_client_base_url,
            &config.sam_auth_endpoint,
            "",
        );

        let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

        let sam_credential_gateway: Arc<CredentialGatewaySamImpl> =
            Arc::new(CredentialGatewaySamImpl::new(sam_client));

        let login_use_case: LoginUseCase = LoginUseCase::new(sam_credential_gateway);

        Ok(Self { login_use_case })
    }

    pub async fn login(&self, email: String, password: String) -> LoginResult {
        self.login_use_case
            .execute(LoginCommand::new(email, password))
            .await
            .into()
    }
}

pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization,
}

impl From<Result<(), LoginUseCaseError>> for LoginResult {
    fn from(value: Result<(), LoginUseCaseError>) -> Self {
        match value {
            Ok(()) => Self::Successful,
            Err(err) => err.into(),
        }
    }
}

impl From<LoginUseCaseError> for LoginResult {
    fn from(value: LoginUseCaseError) -> Self {
        match value {
            LoginUseCaseError::InvalidEmailOrPassword => Self::InvalidEmailOrPassword,
            LoginUseCaseError::UnableToPerformAuthorization => Self::UnableToPerformAuthorization,
        }
    }
}
