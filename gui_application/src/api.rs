pub mod authentication;
pub mod error_report;
pub mod lessons;
pub mod progress;
pub mod roster;

use ::authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LogoutUseCase, RestoreSessionUseCase,
};
use student::application::use_cases::{
    AssessStudentProgressUseCase, RetrieveAllAvailableStudentsUseCase,
    RetrieveStudentLessonsUseCase,
};

use crate::api::authentication::{LoginOutcome, LogoutOutcome, RestoreSessionOutcome};
use crate::api::lessons::RetrieveStudentLessonsOutcome;
use crate::api::progress::AssessStudentProgressOutcome;
use crate::api::roster::RetrieveAllAvailableStudentsOutcome;
use crate::composition::{self, Config};

pub fn build_main_application() -> Result<ApplicationFacade, String> {
    composition::build_application(&Config::production())
}

/// The single entry point Flutter talks to: each method runs one use case and
/// hands back its outcome in the shape the bridge carries.
pub struct ApplicationFacade {
    pub(crate) login_and_remember_credentials: LoginAndRememberCredentialsUseCase,
    pub(crate) restore_session: RestoreSessionUseCase,
    pub(crate) logout: LogoutUseCase,
    pub(crate) retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase,
    pub(crate) retrieve_student_lessons: RetrieveStudentLessonsUseCase,
    pub(crate) assess_student_progress: AssessStudentProgressUseCase,
}

impl ApplicationFacade {
    #[must_use]
    pub fn login(&self, email: String, password: String) -> LoginOutcome {
        self.login_and_remember_credentials
            .execute(email, password)
            .into()
    }

    #[must_use]
    pub fn restore_session(&self) -> RestoreSessionOutcome {
        self.restore_session.execute().into()
    }

    #[must_use]
    pub fn logout(&self) -> LogoutOutcome {
        self.logout.execute().into()
    }

    #[must_use]
    pub fn retrieve_all_available_students(&self) -> RetrieveAllAvailableStudentsOutcome {
        self.retrieve_all_available_students.execute().into()
    }

    #[must_use]
    // FRB bridges owned values, so the id cannot be taken as `&str`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn retrieve_student_lessons(&self, student_id: String) -> RetrieveStudentLessonsOutcome {
        self.retrieve_student_lessons.execute(&student_id).into()
    }

    #[must_use]
    // FRB bridges owned values, so the id cannot be taken as `&str`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn assess_student_progress(&self, student_id: String) -> AssessStudentProgressOutcome {
        self.assess_student_progress.execute(&student_id).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::error_report::{ErrorKindDto, ErrorReportDto};
    use crate::api::lessons::StudentLessonsDto;
    use crate::api::roster::{StudentPositionDto, StudentSummaryDto};
    use ::authentication::adapters::InMemoryCredentialStore;
    use ::authentication::application::gateways::{
        AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, SaveCredentialGateway,
    };
    use ::authentication::domain::entities::{Credential, Email, Password};
    use std::sync::Arc;
    use student::application::gateways::{
        FailureKind, MusicianProfileGateway, MusicianProfileGatewayError, StudentGateway,
        StudentGatewayError, StudentLessonsGateway, StudentLessonsGatewayError,
    };
    use student::domain::entities::{
        Instrument, MusicianLevel, MusicianProfile, Region, Student, StudentLessons,
        StudentPosition,
    };

    #[test]
    fn login_success_is_reported() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.login("e".to_owned(), "p".to_owned());

        assert_eq!(result, LoginOutcome::Successful);
    }

    #[test]
    fn login_failure_is_reported() {
        let facade = facade(
            Ok(AuthorizationResult::Unauthorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.login("e".to_owned(), "p".to_owned());

        assert_eq!(result, LoginOutcome::InvalidEmailOrPassword);
    }

    #[test]
    fn login_gateway_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Err(AuthorizationError::UnableToPerformOperation {
                kind: FailureKind::Transient,
                details: "Request failed for operation 'authentication'".to_owned(),
            }),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.login("e".to_owned(), "p".to_owned());

        assert_eq!(
            result,
            LoginOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::Network,
                    details: "Request failed for operation 'authentication'".to_owned(),
                }
            }
        );
    }

    #[test]
    fn restore_session_with_valid_stored_credentials_is_restored() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            Some(("someone@example.com".to_owned(), "hunter2".to_owned())),
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.restore_session();

        assert_eq!(result, RestoreSessionOutcome::Restored);
    }

    #[test]
    fn logout_clears_the_stored_credential_so_restore_session_finds_none() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            Some(("someone@example.com".to_owned(), "hunter2".to_owned())),
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.logout();

        assert_eq!(result, LogoutOutcome::Successful);
        assert_eq!(
            facade.restore_session(),
            RestoreSessionOutcome::NotAvailable
        );
    }

    #[test]
    fn restore_session_without_stored_credentials_is_not_available() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.restore_session();

        assert_eq!(result, RestoreSessionOutcome::NotAvailable);
    }

    #[test]
    fn restore_session_gateway_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Err(AuthorizationError::UnableToPerformOperation {
                kind: FailureKind::Transient,
                details: "Request failed for operation 'authentication'".to_owned(),
            }),
            Some(("someone@example.com".to_owned(), "hunter2".to_owned())),
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.restore_session();

        assert_eq!(
            result,
            RestoreSessionOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::Network,
                    details: "Request failed for operation 'authentication'".to_owned(),
                }
            }
        );
    }

    #[test]
    fn available_students_are_mapped_to_dtos() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(vec![student()]),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.retrieve_all_available_students();

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcome::Success {
                students: vec![StudentSummaryDto {
                    id: "1".to_owned(),
                    name: "Someone".to_owned(),
                    position: StudentPositionDto::YouthService,
                    location: "Somewhere".to_owned(),
                    instrument_name: Some("VIOLINO".to_owned()),
                }]
            }
        );
    }

    #[test]
    fn available_students_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Err(StudentGatewayError::UnableToPerformOperation {
                kind: FailureKind::Unexpected,
                details: "Unable to decode student listing JSON response: expected value"
                    .to_owned(),
            }),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.retrieve_all_available_students();

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::UnexpectedResponse,
                    details: "Unable to decode student listing JSON response: expected value"
                        .to_owned(),
                }
            }
        );
    }

    #[test]
    fn student_lessons_are_mapped_to_dtos() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.retrieve_student_lessons("1".to_owned());

        assert_eq!(
            result,
            RetrieveStudentLessonsOutcome::Success {
                lessons: StudentLessonsDto {
                    msa: Vec::new(),
                    method: Vec::new(),
                }
            }
        );
    }

    #[test]
    fn student_lessons_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Err(StudentLessonsGatewayError::UnableToPerformOperation {
                kind: FailureKind::Transient,
                details: "Request failed for operation 'student_lessons'".to_owned(),
            }),
        );

        let result = facade.retrieve_student_lessons("1".to_owned());

        assert_eq!(
            result,
            RetrieveStudentLessonsOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::Network,
                    details: "Request failed for operation 'student_lessons'".to_owned(),
                }
            }
        );
    }

    #[test]
    fn progress_assessment_success_is_mapped_to_a_dto() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Candidate,
                instrument: Some(Instrument::Violin),
            }),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert!(matches!(
            result,
            AssessStudentProgressOutcome::Success { .. }
        ));
    }

    #[test]
    fn progress_assessment_reports_missing_instrument() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Candidate,
                instrument: None,
            }),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(result, AssessStudentProgressOutcome::NoInstrumentAssigned);
    }

    #[test]
    fn progress_assessment_reports_unknown_level() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Unknown("EXÓTICO".to_owned()),
                instrument: Some(Instrument::Violin),
            }),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(
            result,
            AssessStudentProgressOutcome::UnknownLevel {
                raw_level: "EXÓTICO".to_owned()
            }
        );
    }

    #[test]
    fn progress_assessment_of_an_unlisted_student_is_an_unexpected_response() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(
            result,
            AssessStudentProgressOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::UnexpectedResponse,
                    details: "no student found with the given id".to_owned(),
                }
            }
        );
    }

    #[test]
    fn progress_assessment_reports_a_non_musician_without_calling_it_a_failure() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotAMusician),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(result, AssessStudentProgressOutcome::NotAMusician);
    }

    #[test]
    fn progress_assessment_reports_profile_retrieval_failures_with_kind_and_details() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::UnableToPerformOperation {
                kind: FailureKind::SessionExpired,
                details: "Session expired".to_owned(),
            }),
            Ok(StudentLessons::default()),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(
            result,
            AssessStudentProgressOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::SessionExpired,
                    details: "Session expired".to_owned(),
                }
            }
        );
    }

    #[test]
    fn progress_assessment_reports_lessons_retrieval_failures_with_kind_and_details() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Candidate,
                instrument: Some(Instrument::Violin),
            }),
            Err(StudentLessonsGatewayError::UnableToPerformOperation {
                kind: FailureKind::Transient,
                details: "connection refused".to_owned(),
            }),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(
            result,
            AssessStudentProgressOutcome::Failure {
                report: ErrorReportDto {
                    kind: ErrorKindDto::Network,
                    details: "connection refused".to_owned(),
                }
            }
        );
    }

    fn facade(
        credential_gateway_result: Result<AuthorizationResult, AuthorizationError>,
        stored_credential: Option<(String, String)>,
        students_result: Result<Vec<Student>, StudentGatewayError>,
        musician_profile_result: Result<MusicianProfile, MusicianProfileGatewayError>,
        student_lessons_result: Result<StudentLessons, StudentLessonsGatewayError>,
    ) -> ApplicationFacade {
        let credential_gateway: Arc<dyn AuthorizeCredentialGateway + Send + Sync> =
            Arc::new(FakeAuthorizationGateway::new(credential_gateway_result));

        let student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync> =
            Arc::new(FakeStudentLessonsGateway::new(student_lessons_result));

        let credential_store: Arc<InMemoryCredentialStore> =
            Arc::new(InMemoryCredentialStore::new());
        if let Some((email, password)) = stored_credential {
            // Saving to a fresh in-memory store cannot fail.
            credential_store
                .save(&Credential::new(Email(email), Password(password)))
                .ok();
        }

        ApplicationFacade {
            login_and_remember_credentials: LoginAndRememberCredentialsUseCase::new(
                credential_gateway.clone(),
                credential_store.clone(),
            ),
            logout: LogoutUseCase::new(credential_store.clone()),
            restore_session: RestoreSessionUseCase::new(
                credential_store.clone(),
                credential_store,
                credential_gateway,
            ),
            retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase::new(Arc::new(
                FakeStudentGateway::new(students_result),
            )),
            retrieve_student_lessons: RetrieveStudentLessonsUseCase::new(
                student_lessons_gateway.clone(),
            ),
            assess_student_progress: AssessStudentProgressUseCase::new(
                Arc::new(FakeMusicianProfileGateway::new(musician_profile_result)),
                student_lessons_gateway,
            ),
        }
    }

    fn student() -> Student {
        Student {
            id: "1".to_owned(),
            name: "Someone".to_owned(),
            position: StudentPosition::Musician {
                level: MusicianLevel::YouthService,
                instrument: Some(Instrument::Violin),
                instrument_name: Some("VIOLINO".to_owned()),
            },
            location: "Somewhere".to_owned(),
            region: Region::Other("Somewhere".to_owned()),
        }
    }

    struct FakeAuthorizationGateway {
        result: Result<AuthorizationResult, AuthorizationError>,
    }

    impl FakeAuthorizationGateway {
        const fn new(result: Result<AuthorizationResult, AuthorizationError>) -> Self {
            Self { result }
        }
    }

    impl AuthorizeCredentialGateway for FakeAuthorizationGateway {
        fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, AuthorizationError> {
            self.result.clone()
        }
    }

    struct FakeStudentGateway {
        result: Result<Vec<Student>, StudentGatewayError>,
    }

    impl FakeStudentGateway {
        const fn new(result: Result<Vec<Student>, StudentGatewayError>) -> Self {
            Self { result }
        }
    }

    impl StudentGateway for FakeStudentGateway {
        fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
            self.result.clone()
        }
    }

    struct FakeMusicianProfileGateway {
        result: Result<MusicianProfile, MusicianProfileGatewayError>,
    }

    impl FakeMusicianProfileGateway {
        const fn new(result: Result<MusicianProfile, MusicianProfileGatewayError>) -> Self {
            Self { result }
        }
    }

    impl MusicianProfileGateway for FakeMusicianProfileGateway {
        fn get_by_id(&self, _id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError> {
            self.result.clone()
        }
    }

    struct FakeStudentLessonsGateway {
        result: Result<StudentLessons, StudentLessonsGatewayError>,
    }

    impl FakeStudentLessonsGateway {
        const fn new(result: Result<StudentLessons, StudentLessonsGatewayError>) -> Self {
            Self { result }
        }
    }

    impl StudentLessonsGateway for FakeStudentLessonsGateway {
        fn get_all_for_student_with_id(
            &self,
            _student_id: &str,
        ) -> Result<StudentLessons, StudentLessonsGatewayError> {
            self.result.clone()
        }
    }
}
