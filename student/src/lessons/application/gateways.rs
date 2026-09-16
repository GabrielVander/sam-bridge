use async_trait::async_trait;
use thiserror::Error;

use crate::lessons::domain::entities::{MusicianProfile, StudentLessons};

#[async_trait]
pub trait StudentLessonsGateway: Send + Sync {
    async fn get_all_for_student_with_id(
        &self,
        id: &str,
    ) -> Result<StudentLessons, StudentLessonsGatewayError>;
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudentLessonsGatewayError {
    #[error("Unable to retrieve student lessons")]
    UnableToPerformOperation,
}

#[async_trait]
pub trait MusicianProfileGateway: Send + Sync {
    async fn get_by_id(&self, id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError>;
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MusicianProfileGatewayError {
    #[error("no student found with the given id")]
    NotFound,
    #[error("student is not a musician")]
    NotAMusician,
    #[error("Unable to retrieve student profile")]
    UnableToPerformOperation,
}
