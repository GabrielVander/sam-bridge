use std::sync::Arc;

use crate::roster::{
    application::{
        dto::StudentSummaryDto,
        gateways::{StudentGateway, StudentGatewayError},
    },
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

    pub fn execute(&self) -> RetrieveAllAvailableStudentsResult {
        let students_result: Result<Vec<Student>, StudentGatewayError> =
            self.student_gateway.get_available_records();

        match students_result {
            Ok(students) => RetrieveAllAvailableStudentsResult::Success(
                students.into_iter().map(StudentSummaryDto::from).collect(),
            ),
            Err(error) => RetrieveAllAvailableStudentsResult::Failure(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveAllAvailableStudentsResult {
    Success(Vec<StudentSummaryDto>),
    Failure(StudentGatewayError),
}
