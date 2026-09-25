use authentication::application::use_cases::{
    LoginUseCaseError, LogoutError, RestoreSessionError,
    RestoreSessionOutcome as RestoreSessionUseCaseOutcome,
};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginOutcome {
    Successful,
    InvalidEmailOrPassword,
    Failure { report: ErrorReportDto },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionOutcome {
    Restored,
    NotAvailable,
    Failure { report: ErrorReportDto },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogoutOutcome {
    Successful,
    Failed,
}

impl From<Result<(), LoginUseCaseError>> for LoginOutcome {
    fn from(result: Result<(), LoginUseCaseError>) -> Self {
        match result {
            Ok(()) => Self::Successful,
            Err(LoginUseCaseError::InvalidEmailOrPassword) => Self::InvalidEmailOrPassword,
            Err(LoginUseCaseError::UnableToPerformAuthorization { kind, details }) => {
                Self::Failure {
                    report: ErrorReportDto::from_failure(kind, details),
                }
            }
        }
    }
}

impl From<Result<RestoreSessionUseCaseOutcome, RestoreSessionError>> for RestoreSessionOutcome {
    fn from(result: Result<RestoreSessionUseCaseOutcome, RestoreSessionError>) -> Self {
        match result {
            Ok(RestoreSessionUseCaseOutcome::Restored) => Self::Restored,
            // Clearing a rejected local credential is best-effort cleanup; there's
            // nothing actionable for the caller, so it still routes to the login form.
            Ok(
                RestoreSessionUseCaseOutcome::NoStoredCredentials
                | RestoreSessionUseCaseOutcome::CredentialsRejected,
            )
            | Err(RestoreSessionError::UnableToClearRejectedCredentials) => Self::NotAvailable,
            Err(RestoreSessionError::UnableToPerformOperation { kind, details }) => Self::Failure {
                report: ErrorReportDto::from_failure(kind, details),
            },
        }
    }
}

impl From<Result<(), LogoutError>> for LogoutOutcome {
    fn from(result: Result<(), LogoutError>) -> Self {
        match result {
            Ok(()) => Self::Successful,
            Err(LogoutError::UnableToClearCredentials) => Self::Failed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_rejected_credential_that_cannot_be_cleared_still_routes_to_the_login_form() {
        let outcome =
            RestoreSessionOutcome::from(Err(RestoreSessionError::UnableToClearRejectedCredentials));

        assert_eq!(outcome, RestoreSessionOutcome::NotAvailable);
    }

    #[test]
    fn a_logout_that_cannot_clear_the_credential_is_reported_as_failed() {
        let outcome = LogoutOutcome::from(Err(LogoutError::UnableToClearCredentials));

        assert_eq!(outcome, LogoutOutcome::Failed);
    }
}
