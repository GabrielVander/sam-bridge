use std::sync::Arc;

use crate::lessons::{
    application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError},
    domain::entities::StudentLessons,
};
use crate::shared::domain::entities::StudentId;

#[derive(Clone)]
pub struct RetrieveStudentLessonsUseCase {
    gateway: Arc<dyn StudentLessonsGateway>,
}

impl RetrieveStudentLessonsUseCase {
    pub fn new(gateway: Arc<dyn StudentLessonsGateway>) -> Self {
        Self { gateway }
    }

    pub fn execute(
        &self,
        student_id: &StudentId,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.gateway.get_all_for_student_with_id(student_id)
    }
}
