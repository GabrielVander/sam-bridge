use authentication::application::gateways::{
    AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, FailureKind,
    LoadCredentialGateway, SaveCredentialGateway, SaveCredentialGatewayError,
};
use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LoginUseCaseError,
};
use authentication::domain::entities::{Credential, Email, Password};
use pretty_assertions::assert_eq;
use std::sync::Arc;

#[path = "support/helpers.rs"]
mod support;
use support::InMemoryCredentialStore;

#[test]
fn successful_login_saves_the_credential() {
    let authorization_gateway: Arc<FakeAuthorizationGateway> =
        Arc::new(FakeAuthorizationGateway::always_authorized());
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let use_case: LoginAndRememberCredentialsUseCase = LoginAndRememberCredentialsUseCase::new(
        authorization_gateway,
        in_memory_credential_store.clone(),
    );

    let result: Result<(), LoginUseCaseError> =
        use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Ok(()));
    assert_eq!(
        in_memory_credential_store.load(),
        Some(Credential::new(
            Email::new("Some email".to_string()),
            Password::new("secretpassword123".to_string())
        ))
    );
}

#[test]
fn failed_login_never_attempts_to_save_the_credential() {
    let authorization_gateway: Arc<FakeAuthorizationGateway> =
        Arc::new(FakeAuthorizationGateway::always_unauthorized());
    let credential_gateway: Arc<InMemoryCredentialStore> = Arc::new(InMemoryCredentialStore::new());

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(authorization_gateway, credential_gateway.clone());

    let result: Result<(), LoginUseCaseError> =
        use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
    assert_eq!(
        credential_gateway.load(),
        None,
        "a rejected login must never be remembered"
    );
}

#[test]
fn a_failure_to_save_the_credential_does_not_fail_the_login() {
    let authorization_gateway: Arc<FakeAuthorizationGateway> =
        Arc::new(FakeAuthorizationGateway::always_authorized());
    let credential_gateway: Arc<FakeSaveCredentialGateway> =
        Arc::new(FakeSaveCredentialGateway::unable_to_perform_operation());

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(authorization_gateway, credential_gateway);

    let result: Result<(), LoginUseCaseError> =
        use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Ok(()));
}

#[test]
fn authorization_gateway_failure_is_reported_with_its_kind_and_details() {
    let fake_authorization: Arc<FakeAuthorizationGateway> = Arc::new(
        FakeAuthorizationGateway::failing_with(AuthorizationError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        }),
    );
    let fake_save_credential: Arc<FakeSaveCredentialGateway> =
        Arc::new(FakeSaveCredentialGateway::unable_to_perform_operation());

    let use_case =
        LoginAndRememberCredentialsUseCase::new(fake_authorization, fake_save_credential);

    let result: Result<(), LoginUseCaseError> =
        use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(
        result,
        Err(LoginUseCaseError::UnableToPerformAuthorization(
            AuthorizationError::UnableToPerformOperation {
                kind: FailureKind::Transient,
                details: "connection refused".to_owned(),
            }
        ))
    );
}

struct FakeAuthorizationGateway {
    result: Result<AuthorizationResult, AuthorizationError>,
}

impl FakeAuthorizationGateway {
    const fn always_authorized() -> Self {
        Self {
            result: Ok(AuthorizationResult::Authorized),
        }
    }

    const fn always_unauthorized() -> Self {
        Self {
            result: Ok(AuthorizationResult::Unauthorized),
        }
    }

    const fn failing_with(error: AuthorizationError) -> Self {
        Self { result: Err(error) }
    }
}

impl AuthorizeCredentialGateway for FakeAuthorizationGateway {
    fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, AuthorizationError> {
        self.result.clone()
    }
}

struct FakeSaveCredentialGateway {
    result: Result<(), SaveCredentialGatewayError>,
}

impl FakeSaveCredentialGateway {
    const fn unable_to_perform_operation() -> Self {
        Self {
            result: Err(SaveCredentialGatewayError::UnableToPerformOperation),
        }
    }
}

impl SaveCredentialGateway for FakeSaveCredentialGateway {
    fn save(&self, _credential: &Credential) -> Result<(), SaveCredentialGatewayError> {
        self.result
    }
}
