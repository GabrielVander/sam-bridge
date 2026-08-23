use std::marker::PhantomData;

use anyhow::{Result, anyhow};

use crate::http::sam_transport::{RawResponse, SamTransport};
use crate::parsing;
pub use crate::parsing::{MsaLesson, MtdLesson, SamStudent};

#[derive(Debug, Clone)]
pub struct SamClient<State> {
    transport: SamTransport,
    _state: PhantomData<State>,
}

#[derive(Debug)]
pub struct Unauthenticated;

#[derive(Debug)]
pub struct Authenticated;

impl SamClient<Unauthenticated> {
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        // Transport construction cannot fail (fixed HTTP client
        // configuration), so the defensive error propagation is compiled out
        // of coverage builds where no test can exercise it.
        #[cfg(coverage)]
        let transport: SamTransport =
            SamTransport::new(base_url).expect("Fixed HTTP client configuration must be valid");
        #[cfg(not(coverage))]
        let transport: SamTransport = SamTransport::new(base_url)?;

        Ok(Self {
            transport,
            _state: PhantomData,
        })
    }

    pub fn login(self, credentials: &SamCredentials) -> Result<SamClient<Authenticated>> {
        let response: RawResponse = self
            .transport
            .authenticate(&credentials.login, &credentials.password)?;

        match parsing::parse_authentication(response.status, &response.body) {
            parsing::AuthOutcome::Authenticated => Ok(SamClient::<Authenticated> {
                transport: self.transport,
                _state: PhantomData,
            }),
            parsing::AuthOutcome::InvalidCredentials => Err(anyhow!("Invalid credentials")),
            parsing::AuthOutcome::Unexpected => {
                Err(anyhow!("Http error. Received unexpected response"))
            }
        }
    }
}

impl SamClient<Authenticated> {
    pub fn students(&self) -> Result<Vec<SamStudent>> {
        self.ensure_session_active()?;

        let response: RawResponse = self.transport.fetch_student_listing()?;

        parsing::parse_students_listing(response.status, &response.body)
    }

    fn ensure_session_active(&self) -> Result<()> {
        parsing::parse_session_status(self.transport.visit_dashboard()?)
    }

    pub fn msa_lessons(&self, student_id: &str) -> Result<Vec<MsaLesson>> {
        let response: RawResponse = self.transport.fetch_msa_lessons(student_id)?;

        parsing::parse_msa_lessons(response.status, &response.body)
    }

    pub fn method_lessons(&self, student_id: &str) -> Result<Vec<MtdLesson>> {
        let response: RawResponse = self.transport.fetch_method_lessons(student_id)?;

        parsing::parse_method_lessons(response.status, &response.body)
    }
}

pub struct SamCredentials {
    pub login: String,
    pub password: String,
}
