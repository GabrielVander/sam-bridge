use std::sync::Arc;

use sam::client::SamReader;
use student_core::{application::gateways::StudentGateway, domain::entities::Student};

use crate::students::infra::mappers::SamStudentMapper;

pub struct StudentsGatewaySamImpl {
    client: Arc<dyn SamReader + Send + Sync>,
}

impl StudentsGatewaySamImpl {
    pub fn new(client: Arc<dyn SamReader + Send + Sync>) -> Self {
        Self { client }
    }
}

#[async_trait::async_trait]
impl StudentGateway for StudentsGatewaySamImpl {
    async fn get_available_records(&self) -> anyhow::Result<Vec<Student>> {
        let client = self.client.clone();

        Ok(smol::unblock(move || client.students())
            .await?
            .iter()
            .map(SamStudentMapper::to_student_entity)
            .collect())
    }
}
