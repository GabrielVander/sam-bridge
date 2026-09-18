use thiserror::Error;

use crate::domain::entities::Student;
pub trait StudentGateway: Send + Sync {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError>;
}

#[derive(Error, Debug, Clone, Copy, PartialEq, Eq)]
pub enum StudentGatewayError {
    #[error("Unable to retrieve student records")]
    UnableToPerformOperation,
}
