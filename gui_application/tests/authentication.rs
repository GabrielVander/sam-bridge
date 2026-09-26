use authentication::application::gateways::{AuthorizationError, AuthorizationResult, FailureKind};
use gui_application::api::authentication::{
    LoginOutcomeDto, LogoutOutcomeDto, RestoreSessionOutcomeDto,
};
use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use pretty_assertions::assert_eq;

#[path = "support/helpers.rs"]
mod support;
use support::FakeSam;

fn network_failure() -> AuthorizationError {
    AuthorizationError::UnableToPerformOperation {
        kind: FailureKind::Transient,
        details: "Request failed for operation 'authentication'".to_owned(),
    }
}

fn network_report() -> ErrorReportDto {
    ErrorReportDto {
        kind: ErrorKindDto::Network,
        details: "Request failed for operation 'authentication'".to_owned(),
    }
}

#[test]
fn an_authorized_login_is_successful() {
    let facade = FakeSam::default()
        .authorizing(Ok(AuthorizationResult::Authorized))
        .build();

    let result = facade.login("e".to_owned(), "p".to_owned());

    assert_eq!(result, LoginOutcomeDto::Successful);
}

#[test]
fn a_login_sam_rejects_is_an_invalid_email_or_password() {
    let facade = FakeSam::default()
        .authorizing(Ok(AuthorizationResult::Unauthorized))
        .build();

    let result = facade.login("e".to_owned(), "p".to_owned());

    assert_eq!(result, LoginOutcomeDto::InvalidEmailOrPassword);
}

#[test]
fn a_login_that_cannot_reach_sam_is_reported_with_its_kind_and_details() {
    let facade = FakeSam::default()
        .authorizing(Err(network_failure()))
        .build();

    let result = facade.login("e".to_owned(), "p".to_owned());

    assert_eq!(
        result,
        LoginOutcomeDto::Failure {
            report: network_report()
        }
    );
}

#[test]
fn a_session_with_valid_stored_credentials_is_restored() {
    let facade = FakeSam::default().with_a_stored_credential().build();

    let result = facade.restore_session();

    assert_eq!(result, RestoreSessionOutcomeDto::Restored);
}

#[test]
fn a_session_without_stored_credentials_is_not_available() {
    let facade = FakeSam::default().build();

    let result = facade.restore_session();

    assert_eq!(result, RestoreSessionOutcomeDto::NotAvailable);
}

#[test]
fn a_session_that_cannot_reach_sam_is_reported_with_its_kind_and_details() {
    let facade = FakeSam::default()
        .with_a_stored_credential()
        .authorizing(Err(network_failure()))
        .build();

    let result = facade.restore_session();

    assert_eq!(
        result,
        RestoreSessionOutcomeDto::Failure {
            report: network_report()
        }
    );
}

#[test]
fn a_rejected_credential_that_cannot_be_cleared_still_routes_to_the_login_form() {
    let facade = FakeSam::default()
        .with_a_stored_credential()
        .authorizing(Ok(AuthorizationResult::Unauthorized))
        .failing_to_clear("Permission denied")
        .build();

    let result = facade.restore_session();

    assert_eq!(result, RestoreSessionOutcomeDto::NotAvailable);
}

#[test]
fn logging_out_clears_the_stored_credential_so_no_session_can_be_restored() {
    let facade = FakeSam::default().with_a_stored_credential().build();

    let result = facade.logout();

    assert_eq!(result, LogoutOutcomeDto::Successful);
    assert_eq!(
        facade.restore_session(),
        RestoreSessionOutcomeDto::NotAvailable
    );
}

#[test]
fn a_logout_that_cannot_clear_the_credential_is_a_local_storage_failure_with_its_details() {
    let facade = FakeSam::default()
        .with_a_stored_credential()
        .failing_to_clear("Unable to remove the credential file: Permission denied")
        .build();

    let result = facade.logout();

    assert_eq!(
        result,
        LogoutOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::LocalStorage,
                details: "Unable to clear credentials: \
                          Unable to remove the credential file: Permission denied"
                    .to_owned(),
            }
        }
    );
}
