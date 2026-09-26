use authentication::application::gateways::AuthorizationError;
use authentication::application::use_cases::LogoutError;
use sam::diagnostics::error_chain;
use shared_kernel::failure_kind::FailureKind;
use student::application::gateways::{
    MusicianProfileGatewayError, StudentGatewayError, StudentLessonsGatewayError,
};
use student::application::use_cases::AssessStudentProgressError;

use crate::composition::StartupError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrorReportDto {
    pub kind: ErrorKindDto,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKindDto {
    Network,
    UnexpectedResponse,
    SessionExpired,
    LocalStorage,
    Unknown,
}

impl ErrorReportDto {
    fn from_failure(kind: FailureKind, details: String) -> Self {
        Self {
            kind: kind.into(),
            details,
        }
    }
}

impl From<FailureKind> for ErrorKindDto {
    fn from(kind: FailureKind) -> Self {
        match kind {
            FailureKind::Transient => Self::Network,
            FailureKind::Unexpected => Self::UnexpectedResponse,
            FailureKind::SessionExpired => Self::SessionExpired,
            FailureKind::Unclassified => Self::Unknown,
        }
    }
}

impl From<StartupError> for ErrorReportDto {
    fn from(error: StartupError) -> Self {
        let kind: ErrorKindDto = match error {
            StartupError::CredentialStorage(_) => ErrorKindDto::LocalStorage,
            StartupError::HttpClient(_) => ErrorKindDto::Unknown,
        };

        Self {
            kind,
            details: error_chain(&error),
        }
    }
}

impl From<AuthorizationError> for ErrorReportDto {
    fn from(error: AuthorizationError) -> Self {
        match error {
            AuthorizationError::UnableToPerformOperation { kind, details } => {
                Self::from_failure(kind, details)
            }
        }
    }
}

impl From<LogoutError> for ErrorReportDto {
    fn from(error: LogoutError) -> Self {
        Self {
            kind: ErrorKindDto::LocalStorage,
            details: error.to_string(),
        }
    }
}

impl From<StudentGatewayError> for ErrorReportDto {
    fn from(error: StudentGatewayError) -> Self {
        match error {
            StudentGatewayError::UnableToPerformOperation { kind, details } => {
                Self::from_failure(kind, details)
            }
        }
    }
}

impl From<StudentLessonsGatewayError> for ErrorReportDto {
    fn from(error: StudentLessonsGatewayError) -> Self {
        match error {
            StudentLessonsGatewayError::UnableToPerformOperation { kind, details } => {
                Self::from_failure(kind, details)
            }
        }
    }
}

impl From<AssessStudentProgressError> for ErrorReportDto {
    fn from(error: AssessStudentProgressError) -> Self {
        match error {
            AssessStudentProgressError::Profile(
                MusicianProfileGatewayError::UnableToPerformOperation { kind, details },
            )
            | AssessStudentProgressError::Lessons(
                StudentLessonsGatewayError::UnableToPerformOperation { kind, details },
            ) => Self::from_failure(kind, details),
            AssessStudentProgressError::Profile(MusicianProfileGatewayError::NotFound) => Self {
                kind: ErrorKindDto::UnexpectedResponse,
                details: error.to_string(),
            },
            other => Self {
                kind: ErrorKindDto::Unknown,
                details: other.to_string(),
            },
        }
    }
}
