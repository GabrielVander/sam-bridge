use std::sync::Arc;

use crate::roster::{
    application::{dto::StudentSummaryDto, gateways::StudentGateway},
    domain::entities::Student,
};

#[derive(Clone)]
pub struct RetrieveAllAvailableStudentsUseCase {
    student_gateway: Arc<dyn StudentGateway + Send + Sync>,
}

impl RetrieveAllAvailableStudentsUseCase {
    pub fn new(gateway: Arc<dyn StudentGateway + Send + Sync>) -> Self {
        Self {
            student_gateway: gateway,
        }
    }

    pub async fn execute(&self) -> RetrieveAllAvailableStudentsResult {
        let students_result: anyhow::Result<Vec<Student>> =
            self.student_gateway.get_available_records().await;

        match students_result {
            Ok(students) => RetrieveAllAvailableStudentsResult::Success(
                students.into_iter().map(StudentSummaryDto::from).collect(),
            ),
            Err(error) => RetrieveAllAvailableStudentsResult::Failure(
                RetrieveAllAvailableStudentsError::GatewayError {
                    context: error.to_string(),
                },
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveAllAvailableStudentsResult {
    Success(Vec<StudentSummaryDto>),
    Failure(RetrieveAllAvailableStudentsError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveAllAvailableStudentsError {
    GatewayError { context: String },
}
