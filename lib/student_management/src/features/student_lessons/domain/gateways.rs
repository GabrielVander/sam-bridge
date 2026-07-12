use async_trait::async_trait;

use crate::features::student_lessons::domain::entities::Lesson;

#[async_trait]
pub trait StudentLessonsGateway {
    async fn get_all_for_student_with_id(&self, id: &str) -> Result<Vec<Lesson>, String>;
}
