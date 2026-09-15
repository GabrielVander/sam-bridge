use thiserror::Error;

use crate::http::{SamOperationError, SamOperations, SamResponse};
use crate::parsing::{
    self, AuthResponse, AuthenticationParser, DashboardParser, DashboardResponse,
};

pub use crate::parsing::SamStudent;

#[derive(Debug, Clone)]
pub struct SamClientImpl {
    sam_ops: SamOperations,
}

pub trait SamClient {
    fn login(&self, credentials: &SamCredentials) -> Result<(), SamClientError>;

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError>;

    // fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage>;
}

impl SamClientImpl {
    #[must_use]
    pub const fn new(sam_ops: SamOperations) -> Self {
        Self { sam_ops }
    }

    // pub fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage> {
    //     if self.transport.base_url() == "http://test-success" {
    //         return Ok(StudentLessonsPage::default());
    //     }
    //
    //     let response: RawResponse = self.transport.fetch_student_lessons(student_id)?;
    //
    //     parsing::parse_student_lessons_page(response.status, &response.body)
    // }

    fn ensure_session_active(&self) -> Result<(), SamClientError> {
        let response: SamResponse = self.sam_ops.dashboard().map_err(SamClientError::from)?;

        match DashboardParser::parse_response(&response) {
            DashboardResponse::Accessed => Ok(()),
            DashboardResponse::Unauthenticated => Err(SamClientError::SessionExpired),
        }
    }
}

impl SamClient for SamClientImpl {
    fn login(&self, credentials: &SamCredentials) -> Result<(), SamClientError> {
        let response: SamResponse = self
            .sam_ops
            .authenticate(&credentials.login, &credentials.password)
            .map_err(SamClientError::from)?;

        match AuthenticationParser::parse_response(&response) {
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

        let status: reqwest::StatusCode = reqwest::StatusCode::from_u16(response.status)
            .unwrap_or(reqwest::StatusCode::INTERNAL_SERVER_ERROR);

        parsing::parse_students_listing(status, &response.body).map_err(|e| {
            SamClientError::UnexpectedResponse {
                context: e.to_string(),
            }
        })
    }

    // fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage> {
    //     self.student_lessons(student_id)
    // }
}

#[derive(Error, Debug)]
pub enum SamClientError {
    #[error("Request failed")]
    RequestError {
        #[from]
        http_error: SamOperationError,
    },
    #[error("Unable to determine request result")]
    UnexpectedResponse { context: String },
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Session expired")]
    SessionExpired,
}

pub struct SamCredentials {
    pub login: String,
    pub password: String,
}
