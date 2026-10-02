use thiserror::Error;

use crate::http::{SamOperationError, SamOperations, SamResponse};
use crate::parsing::{
    self, AuthResponse, DashboardResponse, parse_authentication_response, parse_dashboard_response,
};

pub use crate::parsing::{MsaLesson, MtdLesson, SamStudent, StudentLessonsPage};

pub trait SamClient: Send + Sync {
    fn login(&self, credentials: &SamCredentials) -> Result<(), SamClientError>;

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError>;

    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage, SamClientError>;
}

pub struct SamCredentials {
    pub login: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum SamClientError {
    #[error("Request failed")]
    RequestError {
        #[from]
        http_error: SamOperationError,
    },
    #[error("Unexpected response: {context}")]
    UnexpectedResponse { context: String },
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Session expired")]
    SessionExpired,
}

#[derive(Debug, Clone)]
pub struct SamClientImpl {
    sam_ops: SamOperations,
}

impl SamClientImpl {
    #[must_use]
    pub const fn new(sam_ops: SamOperations) -> Self {
        Self { sam_ops }
    }
}

impl SamClient for SamClientImpl {
    fn login(&self, credentials: &SamCredentials) -> Result<(), SamClientError> {
        let response: SamResponse = self
            .sam_ops
            .authenticate(&credentials.login, &credentials.password)
            .map_err(SamClientError::from)?;

        match parse_authentication_response(&response) {
            AuthResponse::Authenticated => Ok(()),
            AuthResponse::InvalidCredentials => Err(SamClientError::InvalidCredentials),
            AuthResponse::Unexpected => Err(SamClientError::UnexpectedResponse {
                context: "Unexpected authentication response".to_string(),
            }),
        }
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        self.ensure_session_active()?;

        let response: SamResponse = self
            .sam_ops
            .students_listing()
            .map_err(SamClientError::from)?;

        parsing::parse_students_listing(status_of(&response), &response.body).map_err(|e| {
            SamClientError::UnexpectedResponse {
                context: format!("{e:#}"),
            }
        })
    }

    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        let response: SamResponse = self
            .sam_ops
            .student_lessons(student_id)
            .map_err(SamClientError::from)?;

        let status: reqwest::StatusCode = status_of(&response);

        if status.is_redirection() {
            return Err(SamClientError::SessionExpired);
        }

        parsing::parse_student_lessons_page(status, &response.body).map_err(|e| {
            SamClientError::UnexpectedResponse {
                context: e.to_string(),
            }
        })
    }
}

impl SamClientImpl {
    fn ensure_session_active(&self) -> Result<(), SamClientError> {
        let response: SamResponse = self.sam_ops.dashboard().map_err(SamClientError::from)?;

        match parse_dashboard_response(&response) {
            DashboardResponse::Accessed => Ok(()),
            DashboardResponse::Unauthenticated => Err(SamClientError::SessionExpired),
        }
    }
}

fn status_of(response: &SamResponse) -> reqwest::StatusCode {
    reqwest::StatusCode::from_u16(response.status)
        .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
}
