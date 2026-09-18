use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError,
};
use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseError};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;
use std::sync::{Arc, Mutex};

#[test]
fn login_successful() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    let result = use_case.execute(LoginCommand::new(
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ));

    assert_eq!(result, Ok(()));
}

#[test]
fn login_unsuccessful() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Unauthorized,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    let result = use_case.execute(LoginCommand::new(
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ));

    assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
}

#[test]
fn login_failure() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Err(
        CredentialGatewayError::UnableToPerformOperation,
    )));

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway);

    let result = use_case.execute(LoginCommand::new(
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ));

    assert_eq!(result, Err(LoginUseCaseError::UnableToPerformAuthorization));
}

#[test]
fn login_forwards_email_and_password_to_the_credential_gateway() {
    let credential_gateway: Arc<SpyCredentialGateway> = Arc::new(SpyCredentialGateway::new());

    let use_case: LoginUseCase = LoginUseCase::new(credential_gateway.clone());

    use_case
        .execute(LoginCommand::new(
            "Some email".to_string(),
            "secretpassword123".to_string(),
        ))
        .expect("gateway is stubbed to authorize");

    let received_credential: (String, String) = credential_gateway
        .received_credential
        .lock()
        .expect("mutex is not poisoned")
        .clone()
        .expect("authorize should have been called");

    assert_eq!(
        received_credential,
        ("Some email".to_string(), "secretpassword123".to_string())
    );
}

struct FakeCredentialGateway {
    result: Result<AuthorizationResult, CredentialGatewayError>,
}

impl FakeCredentialGateway {
    const fn new(result: Result<AuthorizationResult, CredentialGatewayError>) -> Self {
        Self { result }
    }
}
impl CredentialGateway for FakeCredentialGateway {
    fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, CredentialGatewayError> {
        self.result.clone()
    }
}

struct SpyCredentialGateway {
    received_credential: Mutex<Option<(String, String)>>,
}

impl SpyCredentialGateway {
    const fn new() -> Self {
        Self {
            received_credential: Mutex::new(None),
        }
    }
}
impl CredentialGateway for SpyCredentialGateway {
    fn authorize(
        &self,
        credential: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError> {
        if let Ok(mut received_credential) = self.received_credential.lock() {
            *received_credential =
                Some((credential.email.0.clone(), credential.password.0.clone()));
        }

        Ok(AuthorizationResult::Authorized)
    }
}
