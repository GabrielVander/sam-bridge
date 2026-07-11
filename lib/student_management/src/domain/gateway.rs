use async_trait::async_trait;

use crate::domain::entities::{Lesson, Student};

#[async_trait]
pub trait StudentGateway {
    async fn login(&self, username: String, password: String) -> Result<(), String>;

    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String>;

    async fn get_all_lessons_for_student_with_id(&self, id: &str) -> Result<Vec<Lesson>, String>;
}
