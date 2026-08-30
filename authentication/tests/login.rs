use std::sync::Arc;

use async_trait::async_trait;
use authentication::application::gateways::{AuthorizationResult, CredentialGateway};
use authentication::application::use_cases::{LoginCommand, LoginResult, LoginUseCase};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;

#[test]
fn login_successful() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, LoginResult::Successful);
    });
}

#[test]
fn login_unsuccessful() {
    let credential_gateway: Arc<dyn CredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Unauthorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(result, LoginResult::InvalidEmailOrPassword);
    });
}

#[test]
fn login_failure() {
    let error_message: &str = "Some error";
    let credential_gateway: Arc<dyn CredentialGateway> =
        Arc::new(FakeCredentialGateway::new(Err(error_message.to_string())));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    smol::block_on(async {
        let result: LoginResult = use_case
            .execute(LoginCommand::new(
                "Some email".to_string(),
                "secretpassword123".to_string(),
            ))
            .await;

        assert_eq!(
            result,
            LoginResult::UnableToPerformAuthorization {
                context: error_message.to_string()
            }
        );
    });
}

struct FakeCredentialGateway {
    result: Result<AuthorizationResult, String>,
}

impl FakeCredentialGateway {
    fn new(result: Result<AuthorizationResult, String>) -> Self {
        Self { result }
    }
}

#[async_trait]
impl CredentialGateway for FakeCredentialGateway {
    async fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, String> {
        self.result.clone()
    }
}
