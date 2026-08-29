use std::sync::Arc;

use async_trait::async_trait;
use authentication::application::gateways::{AuthorizationResult, CredentialGateway};
use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseResult};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;

#[test]
fn login_successful() {
    let credential_gateway: Arc<dyn CredentialGateway> = Arc::new(FakeCredentialGatewayImpl::new(
        Ok(AuthorizationResult::Authorized),
    ));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginUseCaseResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, LoginUseCaseResult::Successful);
    });
}

#[test]
fn login_unsuccessful() {
    let credential_gateway: Arc<dyn CredentialGateway> = Arc::new(FakeCredentialGatewayImpl::new(
        Ok(AuthorizationResult::Unauthorized),
    ));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginUseCaseResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, LoginUseCaseResult::InvalidEmailOrPassword);
    });
}

#[test]
fn login_failure() {
    let error_message: &str = "Some error";
    let credential_gateway: Arc<dyn CredentialGateway> = Arc::new(FakeCredentialGatewayImpl::new(
        Err(error_message.to_string()),
    ));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginUseCaseResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(
            result,
            LoginUseCaseResult::UnableToPerformAuthorization {
                context: error_message.to_string()
            }
        );
    });
}

struct FakeCredentialGatewayImpl {
    result: Result<AuthorizationResult, String>,
}

impl FakeCredentialGatewayImpl {
    fn new(result: Result<AuthorizationResult, String>) -> Self {
        Self { result }
    }
}

#[async_trait]
impl CredentialGateway for FakeCredentialGatewayImpl {
    async fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, String> {
        self.result.clone()
    }
}
