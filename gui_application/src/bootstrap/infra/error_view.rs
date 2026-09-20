use authentication::application::gateways::FailureKind as AuthenticationFailureKind;
use student::application::gateways::{
    FailureKind, MusicianProfileGatewayError, StudentGatewayError, StudentLessonsGatewayError,
};
use student::application::use_cases::AssessStudentProgressError;

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
    Unknown,
}

impl From<FailureKind> for ErrorKindDto {
    fn from(kind: FailureKind) -> Self {
        match kind {
            FailureKind::Network => Self::Network,
            FailureKind::UnexpectedResponse => Self::UnexpectedResponse,
            FailureKind::SessionExpired => Self::SessionExpired,
            FailureKind::Unknown => Self::Unknown,
        }
    }
}

impl From<AuthenticationFailureKind> for ErrorKindDto {
    fn from(kind: AuthenticationFailureKind) -> Self {
        match kind {
            AuthenticationFailureKind::Network => Self::Network,
            AuthenticationFailureKind::UnexpectedResponse => Self::UnexpectedResponse,
            AuthenticationFailureKind::SessionExpired => Self::SessionExpired,
            AuthenticationFailureKind::Unknown => Self::Unknown,
        }
    }
}

impl From<StudentGatewayError> for ErrorReportDto {
    fn from(error: StudentGatewayError) -> Self {
        match error {
            StudentGatewayError::UnableToPerformOperation { kind, details } => Self {
                kind: kind.into(),
                details,
            },
        }
    }
}

impl From<StudentLessonsGatewayError> for ErrorReportDto {
    fn from(error: StudentLessonsGatewayError) -> Self {
        match error {
            StudentLessonsGatewayError::UnableToPerformOperation { kind, details } => Self {
                kind: kind.into(),
                details,
            },
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
            ) => Self {
                kind: kind.into(),
                details,
            },
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_authentication_failure_kind_has_a_matching_error_kind_dto() {
        let cases = [
            (AuthenticationFailureKind::Network, ErrorKindDto::Network),
            (
                AuthenticationFailureKind::UnexpectedResponse,
                ErrorKindDto::UnexpectedResponse,
            ),
            (
                AuthenticationFailureKind::SessionExpired,
                ErrorKindDto::SessionExpired,
            ),
            (AuthenticationFailureKind::Unknown, ErrorKindDto::Unknown),
        ];

        for (kind, expected) in cases {
            assert_eq!(ErrorKindDto::from(kind), expected);
        }
    }

    #[test]
    fn a_student_without_an_instrument_is_reported_as_unknown_with_the_reason() {
        let report = ErrorReportDto::from(AssessStudentProgressError::NoInstrumentAssigned);

        assert_eq!(
            report,
            ErrorReportDto {
                kind: ErrorKindDto::Unknown,
                details: "student has no instrument assigned yet".to_owned(),
            }
        );
    }
}
