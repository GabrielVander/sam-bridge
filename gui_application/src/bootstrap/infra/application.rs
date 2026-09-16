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
    sam_student_lessons_gateway: Arc<StudentLessonsGatewaySamImpl>,
    sam_musician_profile_gateway: Arc<MusicianProfileGatewaySamImpl>,
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

        let sam_student_lessons_gateway: Arc<StudentLessonsGatewaySamImpl> =
            Arc::new(StudentLessonsGatewaySamImpl::new(sam_client.clone()));

        let sam_musician_profile_gateway: Arc<MusicianProfileGatewaySamImpl> =
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

pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization,
}

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
