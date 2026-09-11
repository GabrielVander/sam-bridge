use std::sync::Arc;

use async_trait::async_trait;
use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError,
};
use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseError};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;

#[test]
fn login_successful() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    smol::block_on(async {
        let result = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, Ok(()));
    });
}

#[test]
fn login_unsuccessful() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Unauthorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    smol::block_on(async {
        let result = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
    });
}

#[test]
fn login_failure() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Err(
        CredentialGatewayError::UnableToPerformOperation,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    smol::block_on(async {
        let result = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, Err(LoginUseCaseError::UnableToPerformAuthorization));
    });
}

struct FakeCredentialGateway {
    result: Result<AuthorizationResult, CredentialGatewayError>,
}

impl FakeCredentialGateway {
    const fn new(result: Result<AuthorizationResult, CredentialGatewayError>) -> Self {
        Self { result }
    }
}

#[async_trait]
impl CredentialGateway for FakeCredentialGateway {
    async fn authorize(
        &self,
        _: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError> {
        self.result.clone()
    }
}
