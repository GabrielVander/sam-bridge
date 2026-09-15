use std::sync::Arc;
use std::sync::Mutex;

use async_trait::async_trait;
use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError, CredentialStore,
    CredentialStoreError,
};
use authentication::application::use_cases::{
    RememberCredentialsError, RememberCredentialsUseCase, RestoreSessionResult,
    RestoreSessionUseCase,
};
use authentication::domain::entities::{Credential, Email, Password};
use pretty_assertions::assert_eq;

#[test]
fn remember_credentials_persists_via_the_store() {
    let credential_store: Arc<SpyCredentialStore> = Arc::new(SpyCredentialStore::new());

    let use_case: RememberCredentialsUseCase =
        RememberCredentialsUseCase::new(credential_store.clone());

    let result = smol::block_on(async {
        use_case
            .execute(Credential::new(
                Email("Some email".to_string()),
                Password("secretpassword123".to_string()),
            ))
            .await
    });

    assert_eq!(result, Ok(()));

    let saved_credential: (String, String) = credential_store
        .saved_credential
        .lock()
        .expect("mutex is not poisoned")
        .clone()
        .expect("save should have been called");

    assert_eq!(
        saved_credential,
        ("Some email".to_string(), "secretpassword123".to_string())
    );
}

#[test]
fn remember_credentials_surfaces_store_failures() {
    let credential_store: Arc<FailingCredentialStore> = Arc::new(FailingCredentialStore);

    let use_case: RememberCredentialsUseCase = RememberCredentialsUseCase::new(credential_store);

    let result = smol::block_on(async {
        use_case
            .execute(Credential::new(
                Email("Some email".to_string()),
                Password("secretpassword123".to_string()),
            ))
            .await
    });

    assert_eq!(result, Err(RememberCredentialsError::UnableToPersist));
}

#[test]
fn restore_session_without_stored_credentials_reports_no_stored_credentials() {
    let credential_store: Arc<StubCredentialStore> = Arc::new(StubCredentialStore::new(None));
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));

    let use_case: RestoreSessionUseCase =
        RestoreSessionUseCase::new(credential_store, credential_gateway);

    let result = smol::block_on(async { use_case.execute().await });

    assert_eq!(result, RestoreSessionResult::NoStoredCredentials);
}

#[test]
fn restore_session_with_valid_stored_credentials_is_restored() {
    let credential_store: Arc<StubCredentialStore> = Arc::new(StubCredentialStore::new(Some((
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ))));
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));

    let use_case: RestoreSessionUseCase =
        RestoreSessionUseCase::new(credential_store, credential_gateway);

    let result = smol::block_on(async { use_case.execute().await });

    assert_eq!(result, RestoreSessionResult::Restored);
}

#[test]
fn restore_session_with_rejected_credentials_clears_the_store_and_reports_rejected() {
    let credential_store: Arc<StubCredentialStore> = Arc::new(StubCredentialStore::new(Some((
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ))));
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Unauthorized,
    )));

    let use_case: RestoreSessionUseCase =
        RestoreSessionUseCase::new(credential_store.clone(), credential_gateway);

    let result = smol::block_on(async { use_case.execute().await });

    assert_eq!(result, RestoreSessionResult::CredentialsRejected);
    assert!(
        *credential_store
            .clear_called
            .lock()
            .expect("mutex is not poisoned"),
        "rejected stored credentials should be cleared"
    );
}

#[test]
fn restore_session_when_gateway_is_unavailable_leaves_the_store_untouched() {
    let credential_store: Arc<StubCredentialStore> = Arc::new(StubCredentialStore::new(Some((
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ))));
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Err(
        CredentialGatewayError::UnableToPerformOperation,
    )));

    let use_case: RestoreSessionUseCase =
        RestoreSessionUseCase::new(credential_store.clone(), credential_gateway);

    let result = smol::block_on(async { use_case.execute().await });

    assert_eq!(result, RestoreSessionResult::UnableToPerformOperation);
    assert!(
        !*credential_store
            .clear_called
            .lock()
            .expect("mutex is not poisoned"),
        "a transient failure must not forget a valid saved login"
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

#[async_trait]
impl CredentialGateway for FakeCredentialGateway {
    async fn authorize(
        &self,
        _: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError> {
        self.result.clone()
    }
}

struct StubCredentialStore {
    load_result: Option<(String, String)>,
    clear_called: Mutex<bool>,
}

impl StubCredentialStore {
    const fn new(load_result: Option<(String, String)>) -> Self {
        Self {
            load_result,
            clear_called: Mutex::new(false),
        }
    }
}

#[async_trait]
impl CredentialStore for StubCredentialStore {
    async fn save(&self, _: &Credential) -> Result<(), CredentialStoreError> {
        Ok(())
    }

    async fn load(&self) -> Option<Credential> {
        self.load_result
            .clone()
            .map(|(email, password)| Credential::new(Email(email), Password(password)))
    }

    async fn clear(&self) -> Result<(), CredentialStoreError> {
        if let Ok(mut called) = self.clear_called.lock() {
            *called = true;
        }
        Ok(())
    }
}

struct SpyCredentialStore {
    saved_credential: Mutex<Option<(String, String)>>,
}

impl SpyCredentialStore {
    const fn new() -> Self {
        Self {
            saved_credential: Mutex::new(None),
        }
    }
}

#[async_trait]
impl CredentialStore for SpyCredentialStore {
    async fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError> {
        if let Ok(mut saved) = self.saved_credential.lock() {
            *saved = Some((credential.email.0.clone(), credential.password.0.clone()));
        }
        Ok(())
    }

    async fn load(&self) -> Option<Credential> {
        None
    }

    async fn clear(&self) -> Result<(), CredentialStoreError> {
        Ok(())
    }
}

struct FailingCredentialStore;

#[async_trait]
impl CredentialStore for FailingCredentialStore {
    async fn save(&self, _: &Credential) -> Result<(), CredentialStoreError> {
        Err(CredentialStoreError::UnableToPerformOperation)
    }

    async fn load(&self) -> Option<Credential> {
        None
    }

    async fn clear(&self) -> Result<(), CredentialStoreError> {
        Ok(())
    }
}
