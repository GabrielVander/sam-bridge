use authentication::application::use_cases::{
    LoginUseCaseError, LogoutError, RestoreSessionError, RestoreSessionOutcome,
};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginOutcomeDto {
    Successful,
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

impl From<Result<(), LoginUseCaseError>> for LoginOutcomeDto {
    fn from(result: Result<(), LoginUseCaseError>) -> Self {
        match result {
            Ok(()) => Self::Successful,
            Err(LoginUseCaseError::InvalidEmailOrPassword) => Self::InvalidEmailOrPassword,
            Err(LoginUseCaseError::UnableToPerformAuthorization(error)) => Self::Failure {
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
            | Err(RestoreSessionError::UnableToClearRejectedCredentials) => Self::NotAvailable,
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
