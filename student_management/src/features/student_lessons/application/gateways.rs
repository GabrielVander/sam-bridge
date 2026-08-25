use async_trait::async_trait;

use crate::features::student_lessons::domain::entities::StudentLessons;

#[async_trait]
pub trait StudentLessonsGateway: Send + Sync {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<StudentLessons>;
}
