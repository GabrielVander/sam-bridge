use async_trait::async_trait;

use crate::domain::entities::Student;

#[async_trait]
pub trait StudentGateway: Send + Sync {
    async fn get_available_records(&self) -> anyhow::Result<Vec<Student>>;
}
