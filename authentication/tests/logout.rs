use authentication::application::gateways::{
    ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
    SaveCredentialGateway,
};
use authentication::application::use_cases::{LogoutError, LogoutUseCase};
use authentication::domain::entities::Credential;
use pretty_assertions::assert_eq;
use std::sync::Arc;

mod support;
use support::{InMemoryCredentialStore, credential};

#[test]
fn clears_the_store() {
    let in_memory_credential_store: Arc<InMemoryCredentialStore> =
        Arc::new(InMemoryCredentialStore::new());

    let use_case: LogoutUseCase = LogoutUseCase::new(in_memory_credential_store.clone());

    let credential: Credential = credential();
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
fn surfaces_gateway_failures_with_their_details() {
    let fake_clear_credential: Arc<FakeClearCredentialGateway> =
        Arc::new(FakeClearCredentialGateway::unable_to_perform_operation());

    let result: Result<(), LogoutError> = LogoutUseCase::new(fake_clear_credential).execute();

    assert_eq!(
        result,
        Err(LogoutError::UnableToClearCredentials {
            details: "Unable to remove the credential file: Permission denied".to_owned()
        })
    );
}

struct FakeClearCredentialGateway {
    clear: Result<(), ClearCredentialGatewayError>,
}

impl FakeClearCredentialGateway {
    fn unable_to_perform_operation() -> Self {
        Self {
            clear: Err(ClearCredentialGatewayError::UnableToPerformOperation {
                details: "Unable to remove the credential file: Permission denied".to_owned(),
            }),
        }
    }
}

impl ClearCredentialGateway for FakeClearCredentialGateway {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        self.clear.clone()
    }
}
