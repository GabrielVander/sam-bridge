use crate::api::{application::StudentLessonsGateway, domain::StudentLessons};

pub struct RetrieveStudentLessonsUseCase<'a, T: StudentLessonsGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentLessonsGateway> RetrieveStudentLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, student_id: &str) -> anyhow::Result<StudentLessons> {
        self.gateway.get_all_for_student_with_id(student_id).await
    }
}
