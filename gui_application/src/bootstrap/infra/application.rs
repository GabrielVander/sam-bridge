use std::sync::Arc;
use std::time::Duration;

use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LoginUseCaseError, RestoreSessionResult,
    RestoreSessionUseCase,
};
use credential_store::{FileCredentialStore, NoDataDirectory};
use sam::{
    authentication::adapters::gateways::CredentialGatewaySamImpl,
    client::{CacheTtl, SamClient, SamClientCacheDecorator, SamClientImpl, SystemClock},
    diagnostics::error_chain,
    http::SamOperations,
    lessons::adapters::gateways::{MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl},
    roster::adapters::gateways::StudentGatewaySamImpl,
};
use student::application::gateways::{
    MusicianProfileGateway, MusicianProfileGatewayError, StudentLessonsGateway,
};
use student::application::use_cases::{
    AssessStudentProgressError, AssessStudentProgressUseCase, RetrieveAllAvailableStudentsResult,
    RetrieveAllAvailableStudentsUseCase, RetrieveStudentLessonsUseCase,
};
use student::domain::entities::AssessError;

use crate::infra::{
    AssessStudentProgressOutcome, Config, ErrorReportDto, ProgressAssessmentDto,
    RetrieveAllAvailableStudentsOutcome, RetrieveStudentLessonsOutcome, StudentSummaryDto,
};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const STUDENTS_CACHE_TTL: Duration = Duration::from_secs(300);
const LESSONS_CACHE_TTL: Duration = Duration::from_secs(60);

pub struct ApplicationFacade {
    login_and_remember_credentials_use_case: LoginAndRememberCredentialsUseCase,
    restore_session_use_case: RestoreSessionUseCase,
    retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase,
    sam_student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync>,
    sam_musician_profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync>,
}

impl ApplicationFacade {
    pub(crate) fn new(config: &Config) -> Result<Self, String> {
        Self::with_credential_store(
            config,
            FileCredentialStore::new(),
            Self::http_client_builder(),
        )
    }

    fn http_client_builder() -> reqwest::blocking::ClientBuilder {
        Self::http_client_builder_with_timeout(REQUEST_TIMEOUT)
    }

    fn http_client_builder_with_timeout(timeout: Duration) -> reqwest::blocking::ClientBuilder {
        reqwest::blocking::Client::builder()
            .timeout(timeout)
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
    }

    fn with_credential_store(
        config: &Config,
        credential_store: Result<FileCredentialStore, NoDataDirectory>,
        http_client_builder: reqwest::blocking::ClientBuilder,
    ) -> Result<Self, String> {
        let file_credential_store: Arc<FileCredentialStore> =
            Arc::new(credential_store.map_err(|e| error_chain(&e))?);

        let reqwest_client: reqwest::blocking::Client =
            http_client_builder.build().map_err(|e| error_chain(&e))?;

        let sam_operations: SamOperations = SamOperations::new(
            reqwest_client,
            &config.sam_client_base_url,
            &config.sam_auth_endpoint,
            &config.sam_dashboard_endpoint,
            &config.sam_students_listing_endpoint,
            &config.sam_student_lessons_endpoint,
        );

        let sam_client: Arc<dyn SamClient + Send + Sync> = Arc::new(SamClientCacheDecorator::new(
            Arc::new(SamClientImpl::new(sam_operations)),
            Arc::new(SystemClock),
            CacheTtl {
                students: STUDENTS_CACHE_TTL,
                lessons: LESSONS_CACHE_TTL,
            },
        ));

        let sam_credential_gateway: Arc<CredentialGatewaySamImpl> =
            Arc::new(CredentialGatewaySamImpl::new(sam_client.clone()));

        let login_and_remember_credentials_use_case: LoginAndRememberCredentialsUseCase =
            LoginAndRememberCredentialsUseCase::new(
                sam_credential_gateway.clone(),
                file_credential_store.clone(),
            );

        let restore_session_use_case: RestoreSessionUseCase =
            RestoreSessionUseCase::new(file_credential_store, sam_credential_gateway);

        let sam_student_gateway: Arc<StudentGatewaySamImpl> =
            Arc::new(StudentGatewaySamImpl::new(sam_client.clone()));

        let retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase =
            RetrieveAllAvailableStudentsUseCase::new(sam_student_gateway);

        let sam_student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync> =
            Arc::new(StudentLessonsGatewaySamImpl::new(sam_client.clone()));

        let sam_musician_profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync> =
            Arc::new(MusicianProfileGatewaySamImpl::new(sam_client));

        Ok(Self {
            login_and_remember_credentials_use_case,
            restore_session_use_case,
            retrieve_all_available_students_use_case,
            sam_student_lessons_gateway,
            sam_musician_profile_gateway,
        })
    }

    #[must_use]
    pub fn login(&self, email: String, password: String) -> LoginResult {
        self.login_and_remember_credentials_use_case
            .execute(email, password)
            .into()
    }

    #[must_use]
    pub fn restore_session(&self) -> RestoreSessionOutcome {
        match self.restore_session_use_case.execute() {
            RestoreSessionResult::Restored => RestoreSessionOutcome::Restored,
            RestoreSessionResult::NoStoredCredentials
            | RestoreSessionResult::CredentialsRejected
            | RestoreSessionResult::UnableToPerformOperation => RestoreSessionOutcome::NotAvailable,
        }
    }

    #[must_use]
    pub fn retrieve_all_available_students(&self) -> RetrieveAllAvailableStudentsOutcome {
        match self.retrieve_all_available_students_use_case.execute() {
            RetrieveAllAvailableStudentsResult::Success(students) => {
                RetrieveAllAvailableStudentsOutcome::Success(
                    students.into_iter().map(StudentSummaryDto::from).collect(),
                )
            }
            RetrieveAllAvailableStudentsResult::Failure(gateway_error) => {
                RetrieveAllAvailableStudentsOutcome::Failure(gateway_error.into())
            }
        }
    }

    #[must_use]
    // FRB bridges owned values, so the id cannot be taken as `&str`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn retrieve_student_lessons(&self, student_id: String) -> RetrieveStudentLessonsOutcome {
        let use_case =
            RetrieveStudentLessonsUseCase::new(self.sam_student_lessons_gateway.as_ref());

        match use_case.execute(&student_id) {
            Ok(lessons) => {
                let dto: student::application::dto::StudentLessonsDto = lessons.into();
                RetrieveStudentLessonsOutcome::Success(dto.into())
            }
            Err(err) => RetrieveStudentLessonsOutcome::Failure(err.into()),
        }
    }

    #[must_use]
    // FRB bridges owned values, so the id cannot be taken as `&str`.
    #[allow(clippy::needless_pass_by_value)]
    pub fn assess_student_progress(&self, student_id: String) -> AssessStudentProgressOutcome {
        let use_case = AssessStudentProgressUseCase::new(
            self.sam_musician_profile_gateway.as_ref(),
            self.sam_student_lessons_gateway.as_ref(),
        );

        match use_case.execute(&student_id) {
            Ok(assessment) => {
                AssessStudentProgressOutcome::Success(ProgressAssessmentDto::from(assessment))
            }
            Err(AssessStudentProgressError::NoInstrumentAssigned) => {
                AssessStudentProgressOutcome::NoInstrumentAssigned
            }
            Err(AssessStudentProgressError::Assessment(AssessError::UnknownLevel(raw))) => {
                AssessStudentProgressOutcome::UnknownLevel(raw)
            }
            Err(AssessStudentProgressError::Profile(MusicianProfileGatewayError::NotAMusician)) => {
                AssessStudentProgressOutcome::NotAMusician
            }
            Err(err) => AssessStudentProgressOutcome::Failure(err.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization(ErrorReportDto),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestoreSessionOutcome {
    Restored,
    NotAvailable,
}

impl From<Result<(), LoginUseCaseError>> for LoginResult {
    fn from(value: Result<(), LoginUseCaseError>) -> Self {
        match value {
            Ok(()) => Self::Successful,
            Err(err) => err.into(),
        }
    }
}

impl From<LoginUseCaseError> for LoginResult {
    fn from(value: LoginUseCaseError) -> Self {
        match value {
            LoginUseCaseError::InvalidEmailOrPassword => Self::InvalidEmailOrPassword,
            LoginUseCaseError::UnableToPerformAuthorization { kind, details } => {
                Self::UnableToPerformAuthorization(ErrorReportDto {
                    kind: kind.into(),
                    details,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::{ErrorKindDto, ErrorReportDto, StudentLessonsDto, StudentPositionDto};
    use authentication::application::gateways::{
        AuthorizationResult, CredentialGateway, CredentialGatewayError,
    };
    use credential_store::{FileCredentialStore, NoDataDirectory};
    use student::application::gateways::{
        FailureKind, MusicianProfileGatewayError, StudentGatewayError, StudentLessonsGatewayError,
    };
    use student::domain::entities::{
        Instrument, MusicianLevel, MusicianProfile, Region, Student, StudentLessons,
        StudentPosition,
    };
    use test_support::credentials::{FakeCredentialGateway, InMemoryCredentialStore};
    use test_support::students::{
        FakeMusicianProfileGateway, FakeStudentGateway, FakeStudentLessonsGateway,
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

        assert_eq!(result, LoginResult::Successful);
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

        assert_eq!(result, LoginResult::InvalidEmailOrPassword);
    }

    #[test]
    fn login_gateway_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Err(CredentialGatewayError::UnableToPerformOperation {
                kind: authentication::application::gateways::FailureKind::Network,
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
            LoginResult::UnableToPerformAuthorization(ErrorReportDto {
                kind: ErrorKindDto::Network,
                details: "Request failed for operation 'authentication'".to_owned(),
            })
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
            RetrieveAllAvailableStudentsOutcome::Success(vec![StudentSummaryDto {
                id: "1".to_owned(),
                name: "Someone".to_owned(),
                position: StudentPositionDto::YouthService,
                location: "Somewhere".to_owned(),
                instrument_name: Some("VIOLINO".to_owned()),
            }])
        );
    }

    #[test]
    fn available_students_failure_is_reported_with_its_kind_and_details() {
        let facade = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Err(StudentGatewayError::UnableToPerformOperation {
                kind: FailureKind::UnexpectedResponse,
                details: "Unable to decode student listing JSON response: expected value"
                    .to_owned(),
            }),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = facade.retrieve_all_available_students();

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcome::Failure(ErrorReportDto {
                kind: ErrorKindDto::UnexpectedResponse,
                details: "Unable to decode student listing JSON response: expected value"
                    .to_owned(),
            })
        );
    }

    #[test]
    fn every_failure_kind_has_a_matching_error_kind_dto() {
        let cases = [
            (FailureKind::Network, ErrorKindDto::Network),
            (
                FailureKind::UnexpectedResponse,
                ErrorKindDto::UnexpectedResponse,
            ),
            (FailureKind::SessionExpired, ErrorKindDto::SessionExpired),
            (FailureKind::Unknown, ErrorKindDto::Unknown),
        ];

        for (kind, expected) in cases {
            assert_eq!(ErrorKindDto::from(kind), expected);
        }
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
            RetrieveStudentLessonsOutcome::Success(StudentLessonsDto {
                approved: Vec::new(),
                method: Vec::new(),
            })
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
                kind: FailureKind::Network,
                details: "Request failed for operation 'student_lessons'".to_owned(),
            }),
        );

        let result = facade.retrieve_student_lessons("1".to_owned());

        assert_eq!(
            result,
            RetrieveStudentLessonsOutcome::Failure(ErrorReportDto {
                kind: ErrorKindDto::Network,
                details: "Request failed for operation 'student_lessons'".to_owned(),
            })
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

        assert!(matches!(result, AssessStudentProgressOutcome::Success(_)));
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
            AssessStudentProgressOutcome::UnknownLevel("EXÓTICO".to_owned())
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
            AssessStudentProgressOutcome::Failure(ErrorReportDto {
                kind: ErrorKindDto::UnexpectedResponse,
                details: "no student found with the given id".to_owned(),
            })
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
            AssessStudentProgressOutcome::Failure(ErrorReportDto {
                kind: ErrorKindDto::SessionExpired,
                details: "Session expired".to_owned(),
            })
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
                kind: FailureKind::Network,
                details: "connection refused".to_owned(),
            }),
        );

        let result = facade.assess_student_progress("1".to_owned());

        assert_eq!(
            result,
            AssessStudentProgressOutcome::Failure(ErrorReportDto {
                kind: ErrorKindDto::Network,
                details: "connection refused".to_owned(),
            })
        );
    }

    /// The real facade wired to a stand-in SAM site, with credentials stored in
    /// a temporary directory that lives as long as the returned guard.
    fn facade_talking_to(
        mock_server: &wiremock::MockServer,
    ) -> (ApplicationFacade, tempfile::TempDir) {
        let config: Config = Config {
            sam_client_base_url: mock_server.uri(),
            sam_auth_endpoint: "autenticar".to_owned(),
            sam_dashboard_endpoint: "painel".to_owned(),
            sam_students_listing_endpoint: "alunos/listagem".to_owned(),
            sam_student_lessons_endpoint: "licoes/index".to_owned(),
        };
        let credential_dir = tempfile::tempdir().expect("tempdir");
        let facade: ApplicationFacade = ApplicationFacade::with_credential_store(
            &config,
            Ok(FileCredentialStore::with_dir(
                &credential_dir.path().to_string_lossy(),
            )),
            ApplicationFacade::http_client_builder(),
        )
        .expect("facade should be built");

        (facade, credential_dir)
    }

    // SAM answers a successful login with a redirect and tracks the session in
    // a cookie, so the application's HTTP client must do neither of the usual
    // things: follow the redirect, or forget the cookie.

    #[test]
    fn a_login_redirect_is_read_as_success_instead_of_being_followed() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("POST"))
                .and(path("/autenticar"))
                .respond_with(ResponseTemplate::new(303).insert_header("Location", "/painel"))
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            let (facade, _credential_dir) = facade_talking_to(&mock_server);

            let result = facade.login("user@example.com".to_owned(), "hunter2".to_owned());

            assert_eq!(result, LoginResult::Successful);
        });
    }

    #[test]
    fn the_session_cookie_from_login_is_sent_on_later_requests() {
        use wiremock::matchers::{header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("POST"))
                .and(path("/autenticar"))
                .respond_with(
                    ResponseTemplate::new(303).insert_header("Set-Cookie", "session=abc; Path=/"),
                )
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .and(header("cookie", "session=abc"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/alunos/listagem"))
                .and(header("cookie", "session=abc"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(
                            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                        )
                        .insert_header("Content-Type", "application/json"),
                )
                .mount(&mock_server)
                .await;
            let (facade, _credential_dir) = facade_talking_to(&mock_server);
            assert_eq!(
                facade.login("user@example.com".to_owned(), "hunter2".to_owned()),
                LoginResult::Successful
            );

            let result = facade.retrieve_all_available_students();

            assert_eq!(
                result,
                RetrieveAllAvailableStudentsOutcome::Success(Vec::new())
            );
        });
    }

    #[test]
    fn repeated_reads_of_the_students_listing_hit_the_site_once() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;

            Mock::given(method("GET"))
                .and(path("/alunos/listagem"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(
                            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                        )
                        .insert_header("Content-Type", "application/json"),
                )
                .mount(&mock_server)
                .await;

            let (facade, _credential_dir) = facade_talking_to(&mock_server);

            let _ = facade.retrieve_all_available_students();
            let _ = facade.retrieve_all_available_students();

            let listing_requests: usize = mock_server
                .received_requests()
                .await
                .expect("requests should have been recorded")
                .iter()
                .filter(|request| request.url.path() == "/alunos/listagem")
                .count();
            assert_eq!(listing_requests, 1);
        });
    }

    fn any_config() -> Config {
        Config {
            sam_client_base_url: "http://127.0.0.1:1".to_owned(),
            sam_auth_endpoint: "autenticar".to_owned(),
            sam_dashboard_endpoint: "painel".to_owned(),
            sam_students_listing_endpoint: "alunos/listagem".to_owned(),
            sam_student_lessons_endpoint: "licoes/index".to_owned(),
        }
    }

    #[test]
    fn without_a_data_directory_the_application_cannot_be_built() {
        let result = ApplicationFacade::with_credential_store(
            &any_config(),
            Err(NoDataDirectory),
            ApplicationFacade::http_client_builder(),
        );

        assert_eq!(
            result.err(),
            Some("no platform data directory is available to store credentials".to_owned())
        );
    }

    #[test]
    fn when_the_http_client_cannot_be_built_the_application_reports_why() {
        let credential_dir = tempfile::tempdir().expect("tempdir");
        let impossible_tls_range = reqwest::blocking::Client::builder()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .max_tls_version(reqwest::tls::Version::TLS_1_2);

        let result = ApplicationFacade::with_credential_store(
            &any_config(),
            Ok(FileCredentialStore::under(credential_dir.path())),
            impossible_tls_range,
        );

        let message = result.err().expect("building should fail");
        assert!(
            message.contains(": "),
            "the message should include the underlying cause, got: {message}"
        );
    }

    // SAM has been seen taking 87 seconds to answer, more than the 30 seconds
    // reqwest allows by default, which made every request fail. The timeout is
    // therefore set deliberately rather than inherited.

    #[test]
    fn a_response_slower_than_the_request_timeout_fails_as_a_timeout() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(1500)))
                .mount(&mock_server)
                .await;
            let client: reqwest::blocking::Client =
                ApplicationFacade::http_client_builder_with_timeout(Duration::from_millis(200))
                    .build()
                    .expect("HTTP client should be built");

            let error: reqwest::Error = client
                .get(format!("{}/painel", mock_server.uri()))
                .send()
                .expect_err("the response should have been abandoned");

            assert!(error.is_timeout(), "expected a timeout, got: {error:?}");
        });
    }

    #[test]
    fn the_request_timeout_outlasts_the_slowest_sam_response_observed() {
        const SLOWEST_OBSERVED_RESPONSE: Duration = Duration::from_millis(87_500);

        assert!(REQUEST_TIMEOUT > SLOWEST_OBSERVED_RESPONSE);
    }

    /// The facade over fakes. Credentials live in the in-memory store, which the
    /// contract suite keeps faithful to the real one; only the tests that build
    /// the production wiring (`facade_talking_to`) use the encrypted file store.
    fn facade(
        credential_gateway_result: Result<AuthorizationResult, CredentialGatewayError>,
        stored_credential: Option<(String, String)>,
        students_result: Result<Vec<Student>, StudentGatewayError>,
        musician_profile_result: Result<MusicianProfile, MusicianProfileGatewayError>,
        student_lessons_result: Result<StudentLessons, StudentLessonsGatewayError>,
    ) -> ApplicationFacade {
        let credential_gateway: Arc<dyn CredentialGateway + Send + Sync> =
            Arc::new(FakeCredentialGateway::new(credential_gateway_result));

        let credential_store: Arc<InMemoryCredentialStore> = Arc::new(
            stored_credential.map_or_else(InMemoryCredentialStore::new, |(email, password)| {
                InMemoryCredentialStore::holding(email, password)
            }),
        );

        ApplicationFacade {
            login_and_remember_credentials_use_case: LoginAndRememberCredentialsUseCase::new(
                credential_gateway.clone(),
                credential_store.clone(),
            ),
            restore_session_use_case: RestoreSessionUseCase::new(
                credential_store,
                credential_gateway,
            ),
            retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase::new(
                Arc::new(FakeStudentGateway::new(students_result)),
            ),
            sam_student_lessons_gateway: Arc::new(FakeStudentLessonsGateway::new(
                student_lessons_result,
            )),
            sam_musician_profile_gateway: Arc::new(FakeMusicianProfileGateway::new(
                musician_profile_result,
            )),
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
}
