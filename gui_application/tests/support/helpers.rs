//! Not every item here is used by every test binary that includes this
//! module, since each `tests/*.rs` file is compiled separately.
#![allow(dead_code)]

use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use authentication::application::gateways::{
    AuthorizationError, AuthorizationResult, AuthorizeCredentialGateway, ClearCredentialGateway,
    ClearCredentialGatewayError, LoadCredentialGateway, SaveCredentialGateway,
    SaveCredentialGatewayError,
};
use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LogoutUseCase, RestoreSessionUseCase,
};
use authentication::domain::entities::{Credential, Email, Password};
use gui_application::api::ApplicationFacade;
use student::application::gateways::{
    MusicianProfileGateway, MusicianProfileGatewayError, StudentGateway, StudentGatewayError,
    StudentLessonsGateway, StudentLessonsGatewayError,
};
use student::application::use_cases::{
    AssessStudentProgressUseCase, RetrieveAllAvailableStudentsUseCase,
    RetrieveStudentLessonsUseCase,
};
use student::domain::entities::{MusicianProfile, Student, StudentId, StudentLessons};

pub struct FakeSam {
    authorization: Result<AuthorizationResult, AuthorizationError>,
    stored_credential: Option<Credential>,
    clear_failure: Option<String>,
    students: Result<Vec<Student>, StudentGatewayError>,
    musician_profile: Result<MusicianProfile, MusicianProfileGatewayError>,
    lessons: Result<StudentLessons, StudentLessonsGatewayError>,
}

impl Default for FakeSam {
    fn default() -> Self {
        Self {
            authorization: Ok(AuthorizationResult::Authorized),
            stored_credential: None,
            clear_failure: None,
            students: Ok(Vec::new()),
            musician_profile: Err(MusicianProfileGatewayError::NotFound),
            lessons: Ok(StudentLessons::default()),
        }
    }
}

impl FakeSam {
    #[must_use]
    pub fn authorizing(
        self,
        authorization: Result<AuthorizationResult, AuthorizationError>,
    ) -> Self {
        Self {
            authorization,
            ..self
        }
    }

    #[must_use]
    pub fn with_a_stored_credential(self) -> Self {
        Self {
            stored_credential: Some(Credential::new(
                Email::new("someone@example.com".to_owned()),
                Password::new("hunter2".to_owned()),
            )),
            ..self
        }
    }

    #[must_use]
    pub fn failing_to_clear(self, details: &str) -> Self {
        Self {
            clear_failure: Some(details.to_owned()),
            ..self
        }
    }

    #[must_use]
    pub fn listing(self, students: Result<Vec<Student>, StudentGatewayError>) -> Self {
        Self { students, ..self }
    }

    #[must_use]
    pub fn profiling(
        self,
        musician_profile: Result<MusicianProfile, MusicianProfileGatewayError>,
    ) -> Self {
        Self {
            musician_profile,
            ..self
        }
    }

    #[must_use]
    pub fn teaching(self, lessons: Result<StudentLessons, StudentLessonsGatewayError>) -> Self {
        Self { lessons, ..self }
    }

    #[must_use]
    pub fn build(self) -> ApplicationFacade {
        let authorizer: Arc<dyn AuthorizeCredentialGateway> =
            Arc::new(Answering(self.authorization));
        let credential_store = Arc::new(FakeCredentialStore {
            stored: Mutex::new(self.stored_credential),
            clear_failure: self.clear_failure,
        });
        let lessons: Arc<dyn StudentLessonsGateway> = Arc::new(Answering(self.lessons));

        ApplicationFacade::new(
            LoginAndRememberCredentialsUseCase::new(authorizer.clone(), credential_store.clone()),
            RestoreSessionUseCase::new(
                credential_store.clone(),
                credential_store.clone(),
                authorizer,
            ),
            LogoutUseCase::new(credential_store),
            RetrieveAllAvailableStudentsUseCase::new(Arc::new(Answering(self.students))),
            RetrieveStudentLessonsUseCase::new(lessons.clone()),
            AssessStudentProgressUseCase::new(Arc::new(Answering(self.musician_profile)), lessons),
        )
    }
}

struct Answering<T>(T);

impl AuthorizeCredentialGateway for Answering<Result<AuthorizationResult, AuthorizationError>> {
    fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, AuthorizationError> {
        self.0.clone()
    }
}

impl StudentGateway for Answering<Result<Vec<Student>, StudentGatewayError>> {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        self.0.clone()
    }
}

impl MusicianProfileGateway for Answering<Result<MusicianProfile, MusicianProfileGatewayError>> {
    fn get_by_id(&self, _id: &StudentId) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        self.0.clone()
    }
}

impl StudentLessonsGateway for Answering<Result<StudentLessons, StudentLessonsGatewayError>> {
    fn get_all_for_student_with_id(
        &self,
        _id: &StudentId,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.0.clone()
    }
}

struct FakeCredentialStore {
    stored: Mutex<Option<Credential>>,
    clear_failure: Option<String>,
}

impl FakeCredentialStore {
    fn stored(&self) -> MutexGuard<'_, Option<Credential>> {
        self.stored.lock().unwrap_or_else(PoisonError::into_inner)
    }
}

impl SaveCredentialGateway for FakeCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), SaveCredentialGatewayError> {
        *self.stored() = Some(credential.clone());
        Ok(())
    }
}

impl LoadCredentialGateway for FakeCredentialStore {
    fn load(&self) -> Option<Credential> {
        self.stored().clone()
    }
}

impl ClearCredentialGateway for FakeCredentialStore {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        if let Some(details) = &self.clear_failure {
            return Err(ClearCredentialGatewayError::UnableToPerformOperation {
                details: details.clone(),
            });
        }
        *self.stored() = None;
        Ok(())
    }
}
