use thiserror::Error;

use crate::domain::entities::Student;
use shared_kernel::failure_kind::FailureKind;

pub trait StudentGateway: Send + Sync {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError>;
}

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum StudentGatewayError {
    #[error("Unable to retrieve student records: {details}")]
    UnableToPerformOperation { kind: FailureKind, details: String },
}
