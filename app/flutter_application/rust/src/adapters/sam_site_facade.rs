use sam_integration::features::basic_sam_site_interop::infra::sam_client::SamClient;
use student_management::features::{
    authentication::application::use_cases::LoginUseCase,
    student_lessons::application::use_cases::RetrieveStudentLessonsUseCase,
    student_roster::application::use_cases::RetrieveStudentsUseCase,
};

use crate::adapters::view_models::{SingleLessonViewModel, SingleStudentViewModel};

pub struct SamSiteFacade {
    sam_client: SamClient,
}

impl SamSiteFacade {
    #[flutter_rust_bridge::frb(sync)]
    pub fn new() -> anyhow::Result<Self> {
        let client: SamClient = SamClient::new("https://musical.congregacao.org.br/")?;

        Ok(Self { sam_client: client })
    }

    pub async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        LoginUseCase::new(&self.sam_client)
            .execute(username, password)
            .await
    }

    pub async fn retrieve_students(&self) -> anyhow::Result<Vec<SingleStudentViewModel>> {
        RetrieveStudentsUseCase::new(&self.sam_client)
            .execute()
            .await
            .map(|students| students.iter().map(SingleStudentViewModel::from).collect())
    }

    pub async fn retrieve_student_lessons(
        &self,
        student_id: &str,
    ) -> anyhow::Result<Vec<SingleLessonViewModel>> {
        RetrieveStudentLessonsUseCase::new(&self.sam_client)
            .execute(student_id)
            .await
            .map(|lessons| lessons.iter().map(SingleLessonViewModel::from).collect())
    }
}
