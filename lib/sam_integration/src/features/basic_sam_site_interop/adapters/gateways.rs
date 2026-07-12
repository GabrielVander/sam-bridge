use async_trait::async_trait;
use student_management::features::{
    authentication::domain::gateways::AuthGateway,
    student_lessons::domain::{entities::Lesson, gateways::StudentLessonsGateway},
    student_roster::domain::{entities::Student, gateways::StudentsRetrievalGateway},
};

use crate::features::basic_sam_site_interop::infra::sam_client::SamClient;

#[async_trait]
impl AuthGateway for SamClient {
    async fn login(&self, username: String, password: String) -> Result<(), String> {
        self.login(&username, &password)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[async_trait]
impl StudentLessonsGateway for SamClient {
    async fn get_all_for_student_with_id(&self, id: &str) -> Result<Vec<Lesson>, String> {
        self.get_student_lessons(id)
            .await
            .map_err(|e| e.to_string())
    }
}

#[async_trait]
impl StudentsRetrievalGateway for SamClient {
    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String> {
        self.get_students().await.map_err(|e| e.to_string())
    }
}
