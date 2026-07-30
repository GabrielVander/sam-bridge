use sam_integration::api::infrastructure::SamClient;
use student_management::api::application::{
    LoginUseCase, RetrieveStudentLessonsUseCase, RetrieveStudentsUseCase,
};

use crate::adapters::{
    gateways::{
        auth_sam_gateway::AuthSamGateway, student_lesson_sam_gateway::StudentLessonSamGateway,
        student_sam_gateway::StudentSamGateway,
    },
    view_models::{SingleLessonViewModel, SingleStudentViewModel},
};

pub struct SamSiteFacade {
    sam_client: SamClient,
}

impl SamSiteFacade {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> anyhow::Result<Self> {
        let client: SamClient = SamClient::new("https://musical.congregacao.org.br")?;

        Ok(Self { sam_client: client })
    }

    pub async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        LoginUseCase::new(&AuthSamGateway::new(&self.sam_client))
            .execute(username, password)
            .await
    }

    pub async fn retrieve_students(&self) -> anyhow::Result<Vec<SingleStudentViewModel>> {
        RetrieveStudentsUseCase::new(&StudentSamGateway::new(&self.sam_client))
            .execute()
            .await
            .map(|students| students.iter().map(SingleStudentViewModel::from).collect())
    }

    pub async fn retrieve_student_lessons(
        &self,
        student_id: &str,
    ) -> anyhow::Result<Vec<SingleLessonViewModel>> {
        RetrieveStudentLessonsUseCase::new(&StudentLessonSamGateway::new(&self.sam_client))
            .execute(student_id)
            .await
            .map(|lessons| lessons.iter().map(SingleLessonViewModel::from).collect())
    }
}
