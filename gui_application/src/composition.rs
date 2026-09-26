use std::sync::Arc;
use std::time::Duration;

use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LogoutUseCase, RestoreSessionUseCase,
};
use credential_store::{FileCredentialStore, NoDataDirectory};
use sam::{
    authentication::adapters::gateways::AuthorizationGatewaySamImpl,
    client::{CacheTtl, SamClient, SamClientCacheDecorator, SamClientImpl, SystemClock},
    http::SamOperations,
    lessons::adapters::gateways::{MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl},
    roster::adapters::gateways::StudentGatewaySamImpl,
};
use student::application::gateways::{MusicianProfileGateway, StudentLessonsGateway};
use student::application::use_cases::{
    AssessStudentProgressUseCase, RetrieveAllAvailableStudentsUseCase,
    RetrieveStudentLessonsUseCase,
};

use crate::api::ApplicationFacade;

#[derive(Debug, thiserror::Error)]
pub enum StartupError {
    #[error("Unable to set up the credential storage")]
    CredentialStorage(#[from] NoDataDirectory),
    #[error("Unable to build the HTTP client")]
    HttpClient(#[source] reqwest::Error),
}

pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const STUDENTS_CACHE_TTL: Duration = Duration::from_secs(300);
const LESSONS_CACHE_TTL: Duration = Duration::from_secs(60);

pub struct Config {
    pub base_url: String,
    pub auth_endpoint: String,
    pub dashboard_endpoint: String,
    pub students_listing_endpoint: String,
    pub student_lessons_endpoint: String,
}

impl Config {
    #[must_use]
    pub fn production() -> Self {
        Self {
            base_url: "https://musical.congregacao.org.br".to_owned(),
            auth_endpoint: "autenticar".to_owned(),
            dashboard_endpoint: "painel".to_owned(),
            students_listing_endpoint: "alunos/listagem".to_owned(),
            student_lessons_endpoint: "licoes/index".to_owned(),
        }
    }
}

pub fn build_application(config: &Config) -> Result<ApplicationFacade, StartupError> {
    build_application_with(
        config,
        FileCredentialStore::new(),
        http_client_builder(REQUEST_TIMEOUT),
    )
}

pub fn http_client_builder(timeout: Duration) -> reqwest::blocking::ClientBuilder {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
}

pub fn build_application_with(
    config: &Config,
    credential_store: Result<FileCredentialStore, NoDataDirectory>,
    http_client_builder: reqwest::blocking::ClientBuilder,
) -> Result<ApplicationFacade, StartupError> {
    let file_credential_store: Arc<FileCredentialStore> = Arc::new(credential_store?);

    let reqwest_client: reqwest::blocking::Client = http_client_builder
        .build()
        .map_err(StartupError::HttpClient)?;

    let sam_operations: SamOperations = SamOperations::new(
        reqwest_client,
        &config.base_url,
        &config.auth_endpoint,
        &config.dashboard_endpoint,
        &config.students_listing_endpoint,
        &config.student_lessons_endpoint,
    );

    let sam_client: Arc<dyn SamClient + Send + Sync> = Arc::new(SamClientCacheDecorator::new(
        Arc::new(SamClientImpl::new(sam_operations)),
        Arc::new(SystemClock),
        CacheTtl {
            students: STUDENTS_CACHE_TTL,
            lessons: LESSONS_CACHE_TTL,
        },
    ));

    let sam_credential_gateway: Arc<AuthorizationGatewaySamImpl> =
        Arc::new(AuthorizationGatewaySamImpl::new(sam_client.clone()));

    let login_and_remember_credentials: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(
            sam_credential_gateway.clone(),
            file_credential_store.clone(),
        );

    let logout: LogoutUseCase = LogoutUseCase::new(file_credential_store.clone());

    let restore_session: RestoreSessionUseCase = RestoreSessionUseCase::new(
        file_credential_store.clone(),
        file_credential_store,
        sam_credential_gateway,
    );

    let sam_student_gateway: Arc<StudentGatewaySamImpl> =
        Arc::new(StudentGatewaySamImpl::new(sam_client.clone()));

    let retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase =
        RetrieveAllAvailableStudentsUseCase::new(sam_student_gateway);

    let sam_student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync> =
        Arc::new(StudentLessonsGatewaySamImpl::new(sam_client.clone()));

    let sam_musician_profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync> =
        Arc::new(MusicianProfileGatewaySamImpl::new(sam_client));

    Ok(ApplicationFacade::new(
        login_and_remember_credentials,
        restore_session,
        logout,
        retrieve_all_available_students,
        RetrieveStudentLessonsUseCase::new(sam_student_lessons_gateway.clone()),
        AssessStudentProgressUseCase::new(
            sam_musician_profile_gateway,
            sam_student_lessons_gateway,
        ),
    ))
}
