use authentication::application::gateways::{CredentialGatewayError, CredentialStore, FailureKind};
use authentication::application::use_cases::{
    RememberCredentialsError, RememberCredentialsUseCase, RestoreSessionResult,
    RestoreSessionUseCase,
};
use authentication::domain::entities::{Credential, Email, Password};
use pretty_assertions::assert_eq;
use std::sync::Arc;
use test_support::credentials::{
    FailingCredentialStore, FakeCredentialGateway, InMemoryCredentialStore,
};

fn credential() -> Credential {
    Credential::new(
        Email("Some email".to_string()),
        Password("secretpassword123".to_string()),
    )
}

fn store_holding_a_credential() -> Arc<InMemoryCredentialStore> {
    Arc::new(InMemoryCredentialStore::holding(
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ))
}

fn remembered(store: &InMemoryCredentialStore) -> Option<(String, String)> {
    store
        .load()
        .map(|credential| (credential.email.0, credential.password.0))
}

#[test]
fn remember_credentials_persists_via_the_store() {
    let credential_store: Arc<InMemoryCredentialStore> = Arc::new(InMemoryCredentialStore::new());

    let result = RememberCredentialsUseCase::new(credential_store.clone()).execute(&credential());

    assert_eq!(result, Ok(()));
    assert_eq!(
        remembered(&credential_store),
        Some(("Some email".to_string(), "secretpassword123".to_string()))
    );
}

#[test]
fn remember_credentials_surfaces_store_failures() {
    let use_case = RememberCredentialsUseCase::new(Arc::new(FailingCredentialStore));

    let result = use_case.execute(&credential());

    assert_eq!(result, Err(RememberCredentialsError::UnableToPersist));
}

#[test]
fn restore_session_without_stored_credentials_reports_no_stored_credentials() {
    let use_case = RestoreSessionUseCase::new(
        Arc::new(InMemoryCredentialStore::new()),
        Arc::new(FakeCredentialGateway::authorizing()),
    );

    assert_eq!(
        use_case.execute(),
        RestoreSessionResult::NoStoredCredentials
    );
}

#[test]
fn restore_session_with_valid_stored_credentials_is_restored() {
    let use_case = RestoreSessionUseCase::new(
        store_holding_a_credential(),
        Arc::new(FakeCredentialGateway::authorizing()),
    );

    assert_eq!(use_case.execute(), RestoreSessionResult::Restored);
}

#[test]
fn restore_session_with_rejected_credentials_clears_the_store_and_reports_rejected() {
    let credential_store = store_holding_a_credential();
    let use_case = RestoreSessionUseCase::new(
        credential_store.clone(),
        Arc::new(FakeCredentialGateway::rejecting()),
    );

    let result = use_case.execute();

    assert_eq!(result, RestoreSessionResult::CredentialsRejected);
    assert_eq!(
        remembered(&credential_store),
        None,
        "rejected stored credentials should be forgotten"
    );
}

#[test]
fn restore_session_when_gateway_is_unavailable_leaves_the_store_untouched() {
    let credential_store = store_holding_a_credential();
    let use_case = RestoreSessionUseCase::new(
        credential_store.clone(),
        Arc::new(FakeCredentialGateway::new(Err(
            CredentialGatewayError::UnableToPerformOperation {
                kind: FailureKind::Network,
                details: "connection refused".to_owned(),
            },
        ))),
    );

    let result = use_case.execute();

    assert_eq!(result, RestoreSessionResult::UnableToPerformOperation);
    assert_eq!(
        remembered(&credential_store),
        Some(("Some email".to_string(), "secretpassword123".to_string())),
        "a transient failure must not forget a valid saved login"
    );
}
