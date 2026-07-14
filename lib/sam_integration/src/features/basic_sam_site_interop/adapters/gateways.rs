use async_trait::async_trait;
use student_management::api::{
    application::{AuthGateway, StudentLessonsGateway, StudentsRetrievalGateway},
    domain::{Lesson, Student},
};

use crate::api::infrastructure::SamClient;

#[async_trait]
impl AuthGateway for SamClient {
    async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        self.login(&username, &password).await.map(|_| ())
    }
}

#[async_trait]
impl StudentLessonsGateway for SamClient {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        self.get_student_lessons(id).await
    }
}

#[async_trait]
impl StudentsRetrievalGateway for SamClient {
    async fn get_avaliable_records(&self) -> anyhow::Result<Vec<Student>> {
        self.get_students().await
    }
}
