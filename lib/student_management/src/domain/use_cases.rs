use crate::domain::{entities::Lesson, gateway::StudentGateway};

use super::entities::Student;

pub struct RetrieveStudentsImpl<'a, T: StudentGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentGateway> RetrieveStudentsImpl<'a, T> {
    pub async fn retrieve_students(&self) -> Result<Vec<Student>, RetrieveStudentsError> {
        self.gateway
            .get_avaliable_records()
            .await
            .map_err(RetrieveStudentsError::from)
    }
}

pub enum RetrieveStudentsError {
    Generic(String),
}

impl From<String> for RetrieveStudentsError {
    fn from(value: String) -> RetrieveStudentsError {
        RetrieveStudentsError::Generic(value)
    }
}

pub struct RetrieveStudentLessons<'a, T: StudentGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentGateway> RetrieveStudentLessons<'a, T> {
    pub async fn execute(
        &self,
        student_id: &str,
    ) -> Result<Vec<Lesson>, RetrieveStudentLessonsError> {
        self.gateway
            .get_all_lessons_for_student_with_id(student_id)
            .await
            .map_err(RetrieveStudentLessonsError::from)
    }
}

pub enum RetrieveStudentLessonsError {
    Generic(String),
}

impl From<String> for RetrieveStudentLessonsError {
    fn from(value: String) -> RetrieveStudentLessonsError {
        RetrieveStudentLessonsError::Generic(value)
    }
}
