use authentication::adapters::InMemoryCredentialStore;
use authentication::application::gateways::{
    AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, ClearCredentialGateway,
    ClearCredentialGatewayError, FailureKind, LoadCredentialGateway, SaveCredentialGateway,
};
use authentication::application::use_cases::{
    RestoreSessionError, RestoreSessionOutcome, RestoreSessionUseCase,
};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;
use std::sync::Arc;

#[test]
fn use_case_without_stored_credentials_reports_no_stored_credentials() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());
    let fake_credential_authorizer: Arc<FakeCredentialAuthorizer> =
        Arc::new(FakeCredentialAuthorizer::authorizing());

    let use_case: RestoreSessionUseCase = RestoreSessionUseCase::new(
        in_memory_credential_store.clone(),
        in_memory_credential_store,
        fake_credential_authorizer,
    );

    let result: Result<RestoreSessionOutcome, RestoreSessionError> = use_case.execute();

    assert_eq!(result, Ok(RestoreSessionOutcome::NoStoredCredentials));
}

#[test]
fn use_case_with_valid_stored_credentials_is_restored() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let fake_authorization_gateway: Arc<FakeCredentialAuthorizer> =
        Arc::new(FakeCredentialAuthorizer::authorizing());

    let use_case: RestoreSessionUseCase = RestoreSessionUseCase::new(
        in_memory_credential_store.clone(),
        in_memory_credential_store.clone(),
        fake_authorization_gateway,
    );

    in_memory_credential_store
        .save(&Credential::default())
        .unwrap();

    let result: Result<RestoreSessionOutcome, RestoreSessionError> = use_case.execute();

    assert_eq!(result, Ok(RestoreSessionOutcome::Restored));
}

#[test]
fn use_case_with_rejected_credentials_clears_the_store_and_reports_rejected() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let fake_authorization_gateway: Arc<FakeCredentialAuthorizer> =
        Arc::new(FakeCredentialAuthorizer::not_authorizing());

    let use_case: RestoreSessionUseCase = RestoreSessionUseCase::new(
        in_memory_credential_store.clone(),
        in_memory_credential_store.clone(),
        fake_authorization_gateway,
    );

    in_memory_credential_store
        .save(&Credential::default())
        .unwrap();

    let result: Result<RestoreSessionOutcome, RestoreSessionError> = use_case.execute();

    assert_eq!(result, Ok(RestoreSessionOutcome::CredentialsRejected));
    assert_eq!(
        in_memory_credential_store.load(),
        None,
        "rejected stored credentials should be forgotten"
    );
}

#[test]
fn use_case_when_rejected_credentials_cannot_be_cleared_surfaces_the_failure() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let fake_authorization_gateway: Arc<FakeCredentialAuthorizer> =
        Arc::new(FakeCredentialAuthorizer::not_authorizing());
    let fake_credential_eraser: Arc<FakeClearCredentialGateway> =
        Arc::new(FakeClearCredentialGateway::unable_to_perform_operation());

    let use_case: RestoreSessionUseCase = RestoreSessionUseCase::new(
        in_memory_credential_store.clone(),
        fake_credential_eraser,
        fake_authorization_gateway,
    );

    in_memory_credential_store
        .save(&Credential::default())
        .unwrap();

    let result: Result<RestoreSessionOutcome, RestoreSessionError> = use_case.execute();

    assert_eq!(
        result,
        Err(RestoreSessionError::UnableToClearRejectedCredentials)
    );
}

#[test]
fn use_case_when_authorization_gateway_fails_leaves_the_store_untouched() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let authorizer: Arc<FakeCredentialAuthorizer> = Arc::new(
        FakeCredentialAuthorizer::failing_with(AuthorizationError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        }),
    );

    let use_case: RestoreSessionUseCase = RestoreSessionUseCase::new(
        in_memory_credential_store.clone(),
        in_memory_credential_store.clone(),
        authorizer,
    );

    let credential: Credential = Credential::default();
    in_memory_credential_store.save(&credential).unwrap();

    let result: Result<RestoreSessionOutcome, RestoreSessionError> = use_case.execute();

    assert_eq!(
        result,
        Err(RestoreSessionError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        })
    );
    assert_eq!(
        in_memory_credential_store.load(),
        Some(credential),
        "a transient failure must not forget a valid saved login"
    );
}

struct FakeCredentialAuthorizer {
    result: Result<AuthorizationResult, AuthorizationError>,
}

impl FakeCredentialAuthorizer {
    const fn authorizing() -> Self {
        Self {
            result: Ok(AuthorizationResult::Authorized),
        }
    }

    const fn not_authorizing() -> Self {
        Self {
            result: Ok(AuthorizationResult::Unauthorized),
        }
    }

    const fn failing_with(error: AuthorizationError) -> Self {
        Self { result: Err(error) }
    }
}

impl AuthorizeCredentialGateway for FakeCredentialAuthorizer {
    fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, AuthorizationError> {
        self.result.clone()
    }
}

struct FakeClearCredentialGateway {
    clear: Result<(), ClearCredentialGatewayError>,
}

impl FakeClearCredentialGateway {
    const fn unable_to_perform_operation() -> Self {
        Self {
            clear: Err(ClearCredentialGatewayError::UnableToPerformOperation),
        }
    }
}

impl ClearCredentialGateway for FakeClearCredentialGateway {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        self.clear
    }
}
