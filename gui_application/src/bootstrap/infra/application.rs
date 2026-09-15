use std::sync::Arc;

use authentication::application::use_cases::{LoginCommand, LoginUseCase, LoginUseCaseError};
use sam::{
    authentication::adapters::gateways::CredentialGatewaySamImpl, client::SamClientImpl,
    http::SamOperations, roster::adapters::gateways::StudentGatewaySamImpl,
};
use student::application::use_cases::{
    RetrieveAllAvailableStudentsError, RetrieveAllAvailableStudentsResult,
    RetrieveAllAvailableStudentsUseCase,
};

use crate::infra::{Config, RetrieveAllAvailableStudentsOutcome, StudentSummaryDto};

pub struct ApplicationFacade {
    login_use_case: LoginUseCase,
    retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase,
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
        );

        let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

        let sam_credential_gateway: Arc<CredentialGatewaySamImpl> =
            Arc::new(CredentialGatewaySamImpl::new(sam_client.clone()));

        let login_use_case: LoginUseCase = LoginUseCase::new(sam_credential_gateway);

        let sam_student_gateway: Arc<StudentGatewaySamImpl> =
            Arc::new(StudentGatewaySamImpl::new(sam_client));

        let retrieve_all_available_students_use_case: RetrieveAllAvailableStudentsUseCase =
            RetrieveAllAvailableStudentsUseCase::new(sam_student_gateway);

        Ok(Self {
            login_use_case,
            retrieve_all_available_students_use_case,
        })
    }

    pub async fn login(&self, email: String, password: String) -> LoginResult {
        self.login_use_case
            .execute(LoginCommand::new(email, password))
            .await
            .into()
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
            RetrieveAllAvailableStudentsResult::Failure(
                RetrieveAllAvailableStudentsError::GatewayError { context },
            ) => RetrieveAllAvailableStudentsOutcome::Failure(context),
        }
    }
}

pub enum LoginResult {
    Successful,
    InvalidEmailOrPassword,
    UnableToPerformAuthorization,
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
