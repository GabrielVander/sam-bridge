use authentication::application::use_cases::{
    LoginError, LoginOutcome, LogoutError, RestoreSessionError, RestoreSessionOutcome,
};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginOutcomeDto {
    Successful,
    SuccessfulWithoutRemembering { report: ErrorReportDto },
    InvalidEmailOrPassword,
    Failure { report: ErrorReportDto },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionOutcomeDto {
    Restored,
    NotAvailable,
    Failure { report: ErrorReportDto },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogoutOutcomeDto {
    Successful,
    Failure { report: ErrorReportDto },
}

impl From<Result<LoginOutcome, LoginError>> for LoginOutcomeDto {
    fn from(result: Result<LoginOutcome, LoginError>) -> Self {
        match result {
            Ok(LoginOutcome::LoggedIn) => Self::Successful,
            Ok(LoginOutcome::LoggedInWithoutRemembering(error)) => {
                Self::SuccessfulWithoutRemembering {
                    report: error.into(),
                }
            }
            Err(LoginError::InvalidEmailOrPassword) => Self::InvalidEmailOrPassword,
            Err(LoginError::UnableToPerformAuthorization(error)) => Self::Failure {
                report: error.into(),
            },
        }
    }
}

impl From<Result<RestoreSessionOutcome, RestoreSessionError>> for RestoreSessionOutcomeDto {
    fn from(result: Result<RestoreSessionOutcome, RestoreSessionError>) -> Self {
        match result {
            Ok(RestoreSessionOutcome::Restored) => Self::Restored,
            Ok(
                RestoreSessionOutcome::NoStoredCredentials
                | RestoreSessionOutcome::CredentialsRejected,
            )
            | Err(RestoreSessionError::UnableToClearRejectedCredentials(_)) => Self::NotAvailable,
            Err(RestoreSessionError::UnableToPerformOperation(error)) => Self::Failure {
                report: error.into(),
            },
        }
    }
}

impl From<Result<(), LogoutError>> for LogoutOutcomeDto {
    fn from(result: Result<(), LogoutError>) -> Self {
        match result {
            Ok(()) => Self::Successful,
            Err(error) => Self::Failure {
                report: error.into(),
            },
        }
    }
}
