use crate::api::{
    application::{MethodGateway, MsaGateway},
    domain::Lesson,
};

#[allow(deprecated)]
use crate::api::application::StudentLessonsGateway;

pub struct RetrieveMsaLessonsUseCase<'a, T: MsaGateway> {
    gateway: &'a T,
}

impl<'a, T: MsaGateway> RetrieveMsaLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, student_id: &str) -> anyhow::Result<Vec<Lesson>> {
        self.gateway.get_all_for_student_with_id(student_id).await
    }
}

#[allow(deprecated)]
#[deprecated(note = "Use `RetrieveMsaLessonsUseCase` instead")]
pub struct RetrieveStudentLessonsUseCase<'a, T: StudentLessonsGateway> {
    gateway: &'a T,
}

#[allow(deprecated)]
impl<'a, T: StudentLessonsGateway> RetrieveStudentLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, student_id: &str) -> anyhow::Result<Vec<Lesson>> {
        self.gateway.get_all_for_student_with_id(student_id).await
    }
}

pub struct RetrieveMethodLessonsUseCase<'a, T: MethodGateway> {
    gateway: &'a T,
}

impl<'a, T: MethodGateway> RetrieveMethodLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, student_id: &str) -> anyhow::Result<Vec<Lesson>> {
        self.gateway.get_all_for_student_with_id(student_id).await
    }
}
