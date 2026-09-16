use async_trait::async_trait;
use thiserror::Error;

use crate::domain::entities::Student;

#[async_trait]
pub trait StudentGateway: Send + Sync {
    async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError>;
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudentGatewayError {
    #[error("Unable to retrieve student records")]
    UnableToPerformOperation,
}
