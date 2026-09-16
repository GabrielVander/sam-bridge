use async_trait::async_trait;
use thiserror::Error;

use crate::lessons::domain::entities::StudentLessons;

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
