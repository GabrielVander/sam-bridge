use crate::features::student_lessons::domain::{entities::Lesson, gateways::StudentLessonsGateway};

pub struct RetrieveStudentLessonsUseCase<'a, T: StudentLessonsGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentLessonsGateway> RetrieveStudentLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(
        &self,
        student_id: &str,
    ) -> Result<Vec<Lesson>, RetrieveStudentLessonsError> {
        self.gateway
            .get_all_for_student_with_id(student_id)
            .await
            .map_err(RetrieveStudentLessonsError::from)
    }
}

#[derive(Debug)]
pub enum RetrieveStudentLessonsError {
    Generic(String),
}

impl From<String> for RetrieveStudentLessonsError {
    fn from(value: String) -> RetrieveStudentLessonsError {
        RetrieveStudentLessonsError::Generic(value)
    }
}
