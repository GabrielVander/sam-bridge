use async_trait::async_trait;

use crate::{
    features::student_lessons::domain::{entities::Lesson, gateways::StudentLessonsGateway},
    shared::infra::sam_site_adapter::SamSiteAdapter,
};

#[async_trait]
impl StudentLessonsGateway for SamSiteAdapter {
    async fn get_all_for_student_with_id(&self, id: &str) -> Result<Vec<Lesson>, String> {
        self.get_student_lessons(id)
            .await
            .map_err(|e| e.to_string())
    }
}
