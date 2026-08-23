use anyhow::{Result, anyhow};

use crate::http::sam_transport::{RawResponse, SamTransport};
pub use crate::parsing::{MsaLesson, MtdLesson, SamStudent, StudentLessonsPage};
use crate::parsing;

#[derive(Debug, Clone)]
pub struct SamClient {
    transport: SamTransport,
    authenticated: bool,
}

pub trait RosterReader {
    fn students(&self) -> Result<Vec<SamStudent>>;
}

pub trait LessonsReader {
    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage>;
}

impl RosterReader for SamClient {
    fn students(&self) -> Result<Vec<SamStudent>> {
        self.students()
    }
}

impl LessonsReader for SamClient {
    fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage> {
        self.student_lessons(student_id)
    }
}

impl SamClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        // Fixed configuration cannot fail; unreachable error arm compiled out under coverage.
        #[cfg(coverage)]
        let transport: SamTransport =
            SamTransport::new(base_url).expect("Fixed HTTP client configuration must be valid");
        #[cfg(not(coverage))]
        let transport: SamTransport = SamTransport::new(base_url)?;

        Ok(Self {
            transport,
            authenticated: false,
        })
    }

    pub fn login(&mut self, credentials: &SamCredentials) -> Result<()> {
        let response: RawResponse =
            self.transport
                .authenticate(&credentials.login, &credentials.password)?;

        match parsing::parse_authentication(response.status, &response.body) {
            parsing::AuthOutcome::Authenticated => {
                self.authenticated = true;
                Ok(())
            }
            parsing::AuthOutcome::InvalidCredentials => Err(anyhow!("Invalid credentials")),
            parsing::AuthOutcome::Unexpected => {
                Err(anyhow!("Http error. Received unexpected response"))
            }
        }
    }

    pub fn students(&self) -> Result<Vec<SamStudent>> {
        self.ensure_authenticated()?;
        self.ensure_session_active()?;

        let response: RawResponse = self.transport.fetch_student_listing()?;

        parsing::parse_students_listing(response.status, &response.body)
    }

    pub fn student_lessons(&self, student_id: &str) -> Result<StudentLessonsPage> {
        self.ensure_authenticated()?;

        let response: RawResponse = self.transport.fetch_student_lessons(student_id)?;

        parsing::parse_student_lessons_page(response.status, &response.body)
    }

    fn ensure_authenticated(&self) -> Result<()> {
        if self.authenticated {
            Ok(())
        } else {
            Err(anyhow!("Not authenticated"))
        }
    }

    fn ensure_session_active(&self) -> Result<()> {
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
        let client = SamClient::new("http://127.0.0.1:1").expect("client builds without I/O");

        assert!(client.students().is_err());
        assert!(client.student_lessons("1").is_err());
    }

    #[test]
    fn unauthenticated_errors_name_the_problem() {
        let client = SamClient::new("http://127.0.0.1:1").expect("client builds without I/O");

        let err = client.students().unwrap_err().to_string();
        assert_eq!(err, "Not authenticated");

        let err = client.student_lessons("1").unwrap_err().to_string();
        assert_eq!(err, "Not authenticated");
    }

    #[test]
    fn readers_reach_the_same_guards() {
        let client = SamClient::new("http://127.0.0.1:1").expect("client builds without I/O");

        assert!(RosterReader::students(&client).is_err());
        assert!(LessonsReader::student_lessons(&client, "1").is_err());
    }
}
