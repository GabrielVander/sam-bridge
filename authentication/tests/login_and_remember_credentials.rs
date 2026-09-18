use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError, CredentialStore,
    CredentialStoreError,
};
use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LoginUseCaseError,
};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;
use std::sync::{Arc, Mutex};

#[test]
fn successful_login_remembers_the_credential() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));
    let credential_store: Arc<SpyCredentialStore> = Arc::new(SpyCredentialStore::new());

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(credential_gateway, credential_store.clone());

    let result = use_case.execute("Some email".to_string(), "secretpassword123".to_string());

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
fn failed_login_never_attempts_to_remember_the_credential() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Unauthorized,
    )));
    let credential_store: Arc<SpyCredentialStore> = Arc::new(SpyCredentialStore::new());

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(credential_gateway, credential_store.clone());

    let result = use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
    assert!(
        credential_store
            .saved_credential
            .lock()
            .expect("mutex is not poisoned")
            .is_none(),
        "a rejected login must never be remembered"
    );
}

#[test]
fn a_failure_to_remember_the_credential_does_not_fail_the_login() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Ok(
        AuthorizationResult::Authorized,
    )));
    let credential_store: Arc<FailingCredentialStore> = Arc::new(FailingCredentialStore);

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(credential_gateway, credential_store);

    let result = use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Ok(()));
}

#[test]
fn gateway_failure_is_reported_as_unable_to_perform_authorization() {
    let credential_gateway: Arc<FakeCredentialGateway> = Arc::new(FakeCredentialGateway::new(Err(
        CredentialGatewayError::UnableToPerformOperation,
    )));
    let credential_store: Arc<SpyCredentialStore> = Arc::new(SpyCredentialStore::new());

    let use_case: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(credential_gateway, credential_store.clone());

    let result = use_case.execute("Some email".to_string(), "secretpassword123".to_string());

    assert_eq!(result, Err(LoginUseCaseError::UnableToPerformAuthorization));
    assert!(
        credential_store
            .saved_credential
            .lock()
            .expect("mutex is not poisoned")
            .is_none(),
        "a failed authorization attempt must never be remembered"
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
impl CredentialStore for SpyCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError> {
        if let Ok(mut saved) = self.saved_credential.lock() {
            *saved = Some((credential.email.0.clone(), credential.password.0.clone()));
        }
        Ok(())
    }

    fn load(&self) -> Option<Credential> {
        None
    }

    fn clear(&self) -> Result<(), CredentialStoreError> {
        Ok(())
    }
}

struct FailingCredentialStore;
impl CredentialStore for FailingCredentialStore {
    fn save(&self, _: &Credential) -> Result<(), CredentialStoreError> {
        Err(CredentialStoreError::UnableToPerformOperation)
    }

    fn load(&self) -> Option<Credential> {
        None
    }

    fn clear(&self) -> Result<(), CredentialStoreError> {
        Ok(())
    }
}
