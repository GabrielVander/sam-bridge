use std::sync::Arc;

use crate::roster::{
    application::gateways::{StudentGateway, StudentGatewayError},
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

    pub fn execute(&self) -> Result<Vec<Student>, StudentGatewayError> {
        self.student_gateway.get_available_records()
    }
}
