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
use student::domain::entities::StudentId;

use crate::api::authentication::{LoginOutcomeDto, LogoutOutcomeDto, RestoreSessionOutcomeDto};
use crate::api::error_report::ErrorReportDto;
use crate::api::lessons::RetrieveStudentLessonsOutcomeDto;
use crate::api::progress::AssessStudentProgressOutcomeDto;
use crate::api::roster::RetrieveAllAvailableStudentsOutcomeDto;
use crate::composition::{self, Config};
use flutter_rust_bridge::frb;

pub fn build_main_application() -> Result<ApplicationFacade, ErrorReportDto> {
    composition::build_application(&Config::production()).map_err(ErrorReportDto::from)
}

pub struct ApplicationFacade {
    login_and_remember_credentials: LoginAndRememberCredentialsUseCase,
    restore_session: RestoreSessionUseCase,
    logout: LogoutUseCase,
    retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase,
    retrieve_student_lessons: RetrieveStudentLessonsUseCase,
    assess_student_progress: AssessStudentProgressUseCase,
}

impl ApplicationFacade {
    #[frb(ignore)]
    #[must_use]
    pub const fn new(
        login_and_remember_credentials: LoginAndRememberCredentialsUseCase,
        restore_session: RestoreSessionUseCase,
        logout: LogoutUseCase,
        retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase,
        retrieve_student_lessons: RetrieveStudentLessonsUseCase,
        assess_student_progress: AssessStudentProgressUseCase,
    ) -> Self {
        Self {
            login_and_remember_credentials,
            restore_session,
            logout,
            retrieve_all_available_students,
            retrieve_student_lessons,
            assess_student_progress,
        }
    }

    #[must_use]
    pub fn login(&self, email: String, password: String) -> LoginOutcomeDto {
        self.login_and_remember_credentials
            .execute(email, password)
            .into()
    }

    #[must_use]
    pub fn restore_session(&self) -> RestoreSessionOutcomeDto {
        self.restore_session.execute().into()
    }

    #[must_use]
    pub fn logout(&self) -> LogoutOutcomeDto {
        self.logout.execute().into()
    }

    #[must_use]
    pub fn retrieve_all_available_students(&self) -> RetrieveAllAvailableStudentsOutcomeDto {
        self.retrieve_all_available_students.execute().into()
    }

    #[must_use]
    pub fn retrieve_student_lessons(&self, student_id: String) -> RetrieveStudentLessonsOutcomeDto {
        self.retrieve_student_lessons
            .execute(&StudentId::new(student_id))
            .into()
    }

    #[must_use]
    pub fn assess_student_progress(&self, student_id: String) -> AssessStudentProgressOutcomeDto {
        self.assess_student_progress
            .execute(&StudentId::new(student_id))
            .into()
    }
}
