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
    Failure { report: ErrorReportDto },
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
            Err(error) => Self::Failure {
                report: error.into(),
            },
        }
    }
}
