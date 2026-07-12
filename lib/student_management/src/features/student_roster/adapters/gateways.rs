use async_trait::async_trait;

use crate::{
    features::student_roster::domain::{entities::Student, gateways::StudentsRetrievalGateway},
    shared::infra::sam_site_adapter::SamSiteAdapter,
};

#[async_trait]
impl StudentsRetrievalGateway for SamSiteAdapter {
    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String> {
        self.get_students().await.map_err(|e| e.to_string())
    }
}
