use authentication::application::gateways::{CredentialGatewayError, FailureKind};
use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseError};
use pretty_assertions::assert_eq;
use std::sync::Arc;
use test_support::credentials::{FakeCredentialGateway, RecordingCredentialGateway};

fn login_with(gateway: FakeCredentialGateway) -> Result<(), LoginUseCaseError> {
    LoginUseCase::new(Arc::new(gateway)).execute(LoginCommand::new(
        "Some email".to_string(),
        "secretpassword123".to_string(),
    ))
}

#[test]
fn login_successful() {
    let result = login_with(FakeCredentialGateway::authorizing());

    assert_eq!(result, Ok(()));
}

#[test]
fn login_unsuccessful() {
    let result = login_with(FakeCredentialGateway::rejecting());

    assert_eq!(result, Err(LoginUseCaseError::InvalidEmailOrPassword));
}

#[test]
fn login_failure_carries_the_gateways_kind_and_details() {
    let result = login_with(FakeCredentialGateway::new(Err(
        CredentialGatewayError::UnableToPerformOperation {
            kind: FailureKind::Network,
            details: "connection refused".to_owned(),
        },
    )));

    assert_eq!(
        result,
        Err(LoginUseCaseError::UnableToPerformAuthorization {
            kind: FailureKind::Network,
            details: "connection refused".to_owned(),
        })
    );
}

#[test]
fn login_forwards_email_and_password_to_the_credential_gateway() {
    let credential_gateway: Arc<RecordingCredentialGateway> =
        Arc::new(RecordingCredentialGateway::new());

    LoginUseCase::new(credential_gateway.clone())
        .execute(LoginCommand::new(
            "Some email".to_string(),
            "secretpassword123".to_string(),
        ))
        .expect("gateway is stubbed to authorize");

    assert_eq!(
        credential_gateway.last_received(),
        Some(("Some email".to_string(), "secretpassword123".to_string()))
    );
}
