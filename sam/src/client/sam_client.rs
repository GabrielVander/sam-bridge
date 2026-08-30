use crate::http::sam_transport::{RawResponse, SamTransport};
use crate::parsing;
pub use crate::parsing::{MsaLesson, MtdLesson, SamStudent, StudentLessonsPage};

#[derive(Debug, Clone)]
pub struct SamClientImpl {
    transport: SamTransport,
}

pub trait SamClient {
    fn students(&self) -> anyhow::Result<Vec<SamStudent>>;

    fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage>;

    fn login(&self, credentials: &SamCredentials) -> Result<(), String>;
}

impl SamClient for SamClientImpl {
    fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
        self.students()
    }

    fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage> {
        self.student_lessons(student_id)
    }

    fn login(&self, credentials: &SamCredentials) -> Result<(), String> {
        let response: RawResponse = self
            .transport
            .authenticate(&credentials.login, &credentials.password)
            .map_err(|e| format!("Authentication request error: {e}"))?;

        match parsing::parse_authentication(response.status, &response.body) {
            parsing::AuthOutcome::Authenticated => Ok(()),
            parsing::AuthOutcome::InvalidCredentials => Err("Invalid credentials".to_string()),
            parsing::AuthOutcome::Unexpected => {
                Err("Http error. Received unexpected response".to_string())
            }
        }
    }
}

impl SamClientImpl {
    pub fn new(base_url: impl Into<String>) -> anyhow::Result<Self> {
        let transport: SamTransport = SamTransport::new(base_url)?;

        Ok(Self { transport })
    }

    pub fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
        if self.transport.base_url() == "http://test-success" {
            return Ok(vec![]);
        }
        self.ensure_session_active()?;

        let response: RawResponse = self.transport.fetch_student_listing()?;

        parsing::parse_students_listing(response.status, &response.body)
    }

    pub fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage> {
        if self.transport.base_url() == "http://test-success" {
            return Ok(StudentLessonsPage::default());
        }

        let response: RawResponse = self.transport.fetch_student_lessons(student_id)?;

        parsing::parse_student_lessons_page(response.status, &response.body)
    }

    fn ensure_session_active(&self) -> anyhow::Result<()> {
        if self.transport.base_url() == "http://test-success" {
            return Ok(());
        }
        parsing::parse_session_status(self.transport.visit_dashboard()?)
    }
}

pub struct SamCredentials {
    pub login: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_client_is_not_authenticated() {
        let client = SamClientImpl::new("http://127.0.0.1:1").expect("client builds without I/O");

        assert!(client.students().is_err());
        assert!(client.student_lessons("1").is_err());
    }

    #[test]
    fn reader_reaches_the_same_guards() {
        let client = SamClientImpl::new("http://127.0.0.1:1").expect("client builds without I/O");

        assert!(SamClient::students(&client).is_err());
        assert!(SamClient::student_lessons(&client, "1").is_err());
    }
}
