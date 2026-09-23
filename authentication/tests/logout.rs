use authentication::adapters::InMemoryCredentialStore;
use authentication::application::gateways::{
    ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
    SaveCredentialGateway,
};
use authentication::application::use_cases::{LogoutError, LogoutUseCase};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;
use std::sync::Arc;

#[test]
fn clears_the_store() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let use_case: LogoutUseCase = LogoutUseCase::new(in_memory_credential_store.clone());

    let credential: Credential = Credential::default();
    in_memory_credential_store.save(&credential).unwrap();
    assert_eq!(in_memory_credential_store.load(), Some(credential));

    let result: Result<(), LogoutError> = use_case.execute();

    assert_eq!(result, Ok(()));
    assert_eq!(
        in_memory_credential_store.load(),
        None,
        "logging out should forget the stored credential"
    );
}

#[test]
fn surfaces_gateway_failures() {
    let fake_clear_credential: Arc<FakeClearCredentialGateway> =
        Arc::new(FakeClearCredentialGateway::unable_to_perform_operation());

    let result: Result<(), LogoutError> = LogoutUseCase::new(fake_clear_credential).execute();

    assert_eq!(result, Err(LogoutError::UnableToClearCredentials));
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
