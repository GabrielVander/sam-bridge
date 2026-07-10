use async_trait::async_trait;

use crate::domain::entities::Student;

#[async_trait]
pub trait StudentGateway {
    async fn login(&self, username: String, password: String) -> Result<(), String>;

    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String>;
}
