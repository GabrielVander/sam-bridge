use async_trait::async_trait;

use crate::features::student_lessons::domain::entities::Lesson;

#[async_trait]
pub trait MsaGateway {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>>;
}

#[async_trait]
#[deprecated(note = "Use `MsaGateway` instead")]
pub trait StudentLessonsGateway {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>>;
}

#[allow(deprecated)]
#[async_trait]
impl<T: MsaGateway> StudentLessonsGateway for T
where
    T: Send + Sync,
{
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        MsaGateway::get_all_for_student_with_id(self, id).await
    }
}

#[async_trait]
pub trait MethodGateway {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>>;
}

#[async_trait]
#[deprecated(note = "Use `MethodGateway` instead")]
pub trait MethodLessonsGateway {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>>;
}

#[allow(deprecated)]
#[async_trait]
impl<T: MethodGateway> MethodLessonsGateway for T
where
    T: Send + Sync,
{
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        MethodGateway::get_all_for_student_with_id(self, id).await
    }
}
