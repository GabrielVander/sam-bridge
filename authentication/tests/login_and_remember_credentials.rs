use authentication::application::gateways::{CredentialGatewayError, CredentialStore, FailureKind};
use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LoginUseCaseError,
};
use pretty_assertions::assert_eq;
use std::sync::Arc;
use test_support::credentials::{
    FailingCredentialStore, FakeCredentialGateway, InMemoryCredentialStore,
};

fn remembered(store: &InMemoryCredentialStore) -> Option<(String, String)> {
    store
        .load()
        .map(|credential| (credential.email.0, credential.password.0))
}

fn login(
    gateway: FakeCredentialGateway,
    store: Arc<dyn CredentialStore + Send + Sync>,
) -> Result<(), LoginUseCaseError> {
    LoginAndRememberCredentialsUseCase::new(Arc::new(gateway), store)
        .execute("Some email".to_string(), "secretpassword123".to_string())
}

#[test]
fn successful_login_remembers_the_credential() {
    let credential_store: Arc<InMemoryCredentialStore> = Arc::new(InMemoryCredentialStore::new());

    let result = login(
        FakeCredentialGateway::authorizing(),
        credential_store.clone(),
    );

    assert_eq!(result, Ok(()));
    assert_eq!(
        remembered(&credential_store),
        Some(("Some email".to_string(), "secretpassword123".to_string()))
    );
}

#[test]
fn failed_login_never_attempts_to_remember_the_credential() {
    let credential_store: Arc<InMemoryCredentialStore> = Arc::new(InMemoryCredentialStore::new());

    let result = login(FakeCredentialGateway::rejecting(), credential_store.clone());

    assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
    assert_eq!(
        remembered(&credential_store),
        None,
        "a rejected login must never be remembered"
    );
}

#[test]
fn a_failure_to_remember_the_credential_does_not_fail_the_login() {
    let result = login(
        FakeCredentialGateway::authorizing(),
        Arc::new(FailingCredentialStore),
    );

    assert_eq!(result, Ok(()));
}

#[test]
fn gateway_failure_is_reported_with_its_kind_and_details() {
    let credential_store: Arc<InMemoryCredentialStore> = Arc::new(InMemoryCredentialStore::new());

    let result = login(
        FakeCredentialGateway::new(Err(CredentialGatewayError::UnableToPerformOperation {
            kind: FailureKind::Network,
            details: "connection refused".to_owned(),
        })),
        credential_store.clone(),
    );

    assert_eq!(
        result,
        Err(LoginUseCaseError::UnableToPerformAuthorization {
            kind: FailureKind::Network,
            details: "connection refused".to_owned(),
        })
    );
    assert_eq!(
        remembered(&credential_store),
        None,
        "a failed authorization attempt must never be remembered"
    );
}
