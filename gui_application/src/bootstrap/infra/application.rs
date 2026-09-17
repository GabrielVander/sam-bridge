use std::sync::Arc;

use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LoginUseCaseError, RestoreSessionResult,
    RestoreSessionUseCase,
};
use credential_store::FileCredentialStore;
use sam::{
    authentication::adapters::gateways::CredentialGatewaySamImpl,
    client::SamClientImpl,
    http::SamOperations,
    lessons::adapters::gateways::{MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl},
    roster::adapters::gateways::StudentGatewaySamImpl,
};
use student::application::gateways::{MusicianProfileGateway, StudentLessonsGateway};
use student::application::use_cases::{
    AssessStudentProgressError, AssessStudentProgressUseCase, RetrieveAllAvailableStudentsResult,
    RetrieveAllAvailableStudentsUseCase, RetrieveStudentLessonsUseCase,
};
use student::domain::entities::AssessError;

use crate::infra::{
    AssessStudentProgressOutcome, Config, ProgressAssessmentDto,
    RetrieveAllAvailableStudentsOutcome, RetrieveStudentLessonsOutcome, StudentSummaryDto,
};

pub struct ApplicationFacade {
    login_and_remember_credentials_use_case: LoginAndRememberCredentialsUseCase,
    restore_session_use_case: RestoreSessionUseCase,
    retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase,
    sam_student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync>,
    sam_musician_profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync>,
}

impl ApplicationFacade {
    pub(crate) fn new(config: &Config) -> Result<Self, String> {
        let reqwest_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .map_err(|e| e.to_string())?;

        let sam_operations: SamOperations = SamOperations::new(
            reqwest_client,
            &config.sam_client_base_url,
            &config.sam_auth_endpoint,
            &config.sam_dashboard_endpoint,
            &config.sam_students_listing_endpoint,
            &config.sam_student_lessons_endpoint,
        );

        let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

        let sam_credential_gateway: Arc<CredentialGatewaySamImpl> =
            Arc::new(CredentialGatewaySamImpl::new(sam_client.clone()));

        let file_credential_store: Arc<FileCredentialStore> = Arc::new(FileCredentialStore::new());

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

    pub async fn login(&self, email: String, password: String) -> LoginResult {
        self.login_and_remember_credentials_use_case
            .execute(email, password)
            .await
            .into()
    }

    pub async fn restore_session(&self) -> RestoreSessionOutcome {
        match self.restore_session_use_case.execute().await {
            RestoreSessionResult::Restored => RestoreSessionOutcome::Restored,
            RestoreSessionResult::NoStoredCredentials
            | RestoreSessionResult::CredentialsRejected
            | RestoreSessionResult::UnableToPerformOperation => RestoreSessionOutcome::NotAvailable,
        }
    }

    pub async fn retrieve_all_available_students(&self) -> RetrieveAllAvailableStudentsOutcome {
        match self
            .retrieve_all_available_students_use_case
            .execute()
            .await
        {
            RetrieveAllAvailableStudentsResult::Success(students) => {
                RetrieveAllAvailableStudentsOutcome::Success(
                    students.into_iter().map(StudentSummaryDto::from).collect(),
                )
            }
            RetrieveAllAvailableStudentsResult::Failure(gateway_error) => {
                RetrieveAllAvailableStudentsOutcome::Failure(gateway_error.to_string())
            }
        }
    }

    pub async fn retrieve_student_lessons(
        &self,
        student_id: String,
    ) -> RetrieveStudentLessonsOutcome {
        let use_case =
            RetrieveStudentLessonsUseCase::new(self.sam_student_lessons_gateway.as_ref());

        match use_case.execute(&student_id).await {
            Ok(lessons) => {
                let dto: student::application::dto::StudentLessonsDto = lessons.into();
                RetrieveStudentLessonsOutcome::Success(dto.into())
            }
            Err(err) => RetrieveStudentLessonsOutcome::Failure(err.to_string()),
        }
    }

    pub async fn assess_student_progress(
        &self,
        student_id: String,
    ) -> AssessStudentProgressOutcome {
        let use_case = AssessStudentProgressUseCase::new(
            self.sam_musician_profile_gateway.as_ref(),
            self.sam_student_lessons_gateway.as_ref(),
        );

        match use_case.execute(&student_id).await {
            Ok(assessment) => {
                AssessStudentProgressOutcome::Success(ProgressAssessmentDto::from(assessment))
            }
            Err(AssessStudentProgressError::NoInstrumentAssigned) => {
                AssessStudentProgressOutcome::NoInstrumentAssigned
            }
            Err(AssessStudentProgressError::Assessment(AssessError::UnknownLevel(raw))) => {
                AssessStudentProgressOutcome::UnknownLevel(raw)
            }
            Err(err) => AssessStudentProgressOutcome::Failure(err.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization,
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
            LoginUseCaseError::UnableToPerformAuthorization => Self::UnableToPerformAuthorization,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infra::StudentLessonsDto;
    use async_trait::async_trait;
    use authentication::application::gateways::{
        AuthorizationResult, CredentialGateway, CredentialGatewayError, CredentialStore,
    };
    use authentication::domain::entities::{Credential, Email, Password};
    use credential_store::FileCredentialStore;
    use student::application::gateways::{
        MusicianProfileGatewayError, StudentGateway, StudentGatewayError,
        StudentLessonsGatewayError,
    };
    use student::domain::entities::{
        Instrument, MusicianLevel, MusicianProfile, Region, Student, StudentLessons,
        StudentPosition,
    };

    struct FakeCredentialGateway {
        result: Result<AuthorizationResult, CredentialGatewayError>,
    }

    #[async_trait]
    impl CredentialGateway for FakeCredentialGateway {
        async fn authorize(
            &self,
            _: &Credential,
        ) -> Result<AuthorizationResult, CredentialGatewayError> {
            self.result.clone()
        }
    }

    struct FakeStudentGateway {
        result: Result<Vec<Student>, StudentGatewayError>,
    }

    #[async_trait]
    impl StudentGateway for FakeStudentGateway {
        async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
            self.result.clone()
        }
    }

    struct FakeMusicianProfileGateway {
        result: Result<MusicianProfile, MusicianProfileGatewayError>,
    }

    #[async_trait]
    impl MusicianProfileGateway for FakeMusicianProfileGateway {
        async fn get_by_id(
            &self,
            _id: &str,
        ) -> Result<MusicianProfile, MusicianProfileGatewayError> {
            self.result.clone()
        }
    }

    struct FakeStudentLessonsGateway {
        result: Result<StudentLessons, StudentLessonsGatewayError>,
    }

    #[async_trait]
    impl StudentLessonsGateway for FakeStudentLessonsGateway {
        async fn get_all_for_student_with_id(
            &self,
            _student_id: &str,
        ) -> Result<StudentLessons, StudentLessonsGatewayError> {
            self.result.clone()
        }
    }

    fn facade(
        credential_gateway_result: Result<AuthorizationResult, CredentialGatewayError>,
        stored_credential: Option<(String, String)>,
        students_result: Result<Vec<Student>, StudentGatewayError>,
        musician_profile_result: Result<MusicianProfile, MusicianProfileGatewayError>,
        student_lessons_result: Result<StudentLessons, StudentLessonsGatewayError>,
    ) -> (ApplicationFacade, tempfile::TempDir) {
        let credential_gateway: Arc<dyn CredentialGateway + Send + Sync> =
            Arc::new(FakeCredentialGateway {
                result: credential_gateway_result,
            });

        let credential_dir = tempfile::tempdir().expect("tempdir");
        let credential_store: Arc<FileCredentialStore> = Arc::new(FileCredentialStore::with_dir(
            &credential_dir.path().to_string_lossy(),
        ));
        if let Some((email, password)) = stored_credential {
            smol::block_on(
                credential_store.save(&Credential::new(Email(email), Password(password))),
            )
            .expect("seeding the credential store should succeed");
        }

        let facade = ApplicationFacade {
            login_and_remember_credentials_use_case: LoginAndRememberCredentialsUseCase::new(
                credential_gateway.clone(),
                credential_store.clone(),
            ),
            restore_session_use_case: RestoreSessionUseCase::new(
                credential_store,
                credential_gateway,
            ),
            retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase::new(
                Arc::new(FakeStudentGateway {
                    result: students_result,
                }),
            ),
            sam_student_lessons_gateway: Arc::new(FakeStudentLessonsGateway {
                result: student_lessons_result,
            }),
            sam_musician_profile_gateway: Arc::new(FakeMusicianProfileGateway {
                result: musician_profile_result,
            }),
        };

        (facade, credential_dir)
    }

    fn student() -> Student {
        Student {
            id: "1".to_owned(),
            name: "Someone".to_owned(),
            position: StudentPosition::Musician {
                level: MusicianLevel::YouthService,
                instrument: Some(Instrument::Violin),
            },
            location: "Somewhere".to_owned(),
            region: Region::Other("Somewhere".to_owned()),
        }
    }

    #[test]
    fn login_success_is_reported() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.login("e".to_owned(), "p".to_owned()).await });

        assert_eq!(result, LoginResult::Successful);
    }

    #[test]
    fn login_failure_is_reported() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Unauthorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.login("e".to_owned(), "p".to_owned()).await });

        assert_eq!(result, LoginResult::InvalidEmailOrPassword);
    }

    #[test]
    fn login_gateway_failure_is_reported() {
        let (facade, _credential_dir) = facade(
            Err(CredentialGatewayError::UnableToPerformOperation),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.login("e".to_owned(), "p".to_owned()).await });

        assert_eq!(result, LoginResult::UnableToPerformAuthorization);
    }

    #[test]
    fn restore_session_with_valid_stored_credentials_is_restored() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            Some(("someone@example.com".to_owned(), "hunter2".to_owned())),
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.restore_session().await });

        assert_eq!(result, RestoreSessionOutcome::Restored);
    }

    #[test]
    fn restore_session_without_stored_credentials_is_not_available() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.restore_session().await });

        assert_eq!(result, RestoreSessionOutcome::NotAvailable);
    }

    #[test]
    fn available_students_are_mapped_to_dtos() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(vec![student()]),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.retrieve_all_available_students().await });

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcome::Success(vec![StudentSummaryDto::from(
                student::application::dto::StudentSummaryDto::from(student())
            )])
        );
    }

    #[test]
    fn available_students_failure_is_reported() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Err(StudentGatewayError::UnableToPerformOperation),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.retrieve_all_available_students().await });

        assert!(matches!(
            result,
            RetrieveAllAvailableStudentsOutcome::Failure(_)
        ));
    }

    #[test]
    fn student_lessons_are_mapped_to_dtos() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result =
            smol::block_on(async { facade.retrieve_student_lessons("1".to_owned()).await });

        assert_eq!(
            result,
            RetrieveStudentLessonsOutcome::Success(StudentLessonsDto {
                approved: Vec::new(),
                method: Vec::new(),
            })
        );
    }

    #[test]
    fn student_lessons_failure_is_reported() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Err(StudentLessonsGatewayError::UnableToPerformOperation),
        );

        let result =
            smol::block_on(async { facade.retrieve_student_lessons("1".to_owned()).await });

        assert!(matches!(result, RetrieveStudentLessonsOutcome::Failure(_)));
    }

    #[test]
    fn progress_assessment_success_is_mapped_to_a_dto() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Candidate,
                instrument: Some(Instrument::Violin),
            }),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.assess_student_progress("1".to_owned()).await });

        assert!(matches!(result, AssessStudentProgressOutcome::Success(_)));
    }

    #[test]
    fn progress_assessment_reports_missing_instrument() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Candidate,
                instrument: None,
            }),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.assess_student_progress("1".to_owned()).await });

        assert_eq!(result, AssessStudentProgressOutcome::NoInstrumentAssigned);
    }

    #[test]
    fn progress_assessment_reports_unknown_level() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Ok(MusicianProfile {
                level: MusicianLevel::Unknown("EXÓTICO".to_owned()),
                instrument: Some(Instrument::Violin),
            }),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.assess_student_progress("1".to_owned()).await });

        assert_eq!(
            result,
            AssessStudentProgressOutcome::UnknownLevel("EXÓTICO".to_owned())
        );
    }

    #[test]
    fn progress_assessment_reports_other_failures() {
        let (facade, _credential_dir) = facade(
            Ok(AuthorizationResult::Authorized),
            None,
            Ok(Vec::new()),
            Err(MusicianProfileGatewayError::NotFound),
            Ok(StudentLessons::default()),
        );

        let result = smol::block_on(async { facade.assess_student_progress("1".to_owned()).await });

        assert!(matches!(result, AssessStudentProgressOutcome::Failure(_)));
    }
}
