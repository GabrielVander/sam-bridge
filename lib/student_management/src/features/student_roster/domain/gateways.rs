use async_trait::async_trait;

use crate::features::student_roster::domain::entities::Student;

#[async_trait]
pub trait StudentsRetrievalGateway {
    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String>;
}
