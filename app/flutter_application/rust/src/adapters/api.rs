use student_management::{
    features::{
        authentication::application::use_cases::LoginUseCase,
        student_lessons::application::use_cases::RetrieveStudentLessonsUseCase,
        student_roster::application::use_cases::RetrieveStudentsUseCase,
    },
    shared::infra::sam_site_adapter::SamSiteAdapter,
};

use crate::adapters::view_models::{SingleLessonViewModel, SingleStudentViewModel};

pub struct Api {
    sam_site_adapter: SamSiteAdapter,
}

impl Api {
    pub fn new() -> Self {
        let adapter: SamSiteAdapter = SamSiteAdapter::new("https://musical.congregacao.org.br/")
            .map_err(|e| format!("{:#?}", e))
            .unwrap();

        Self {
            sam_site_adapter: adapter,
        }
    }

    pub async fn login(&self, username: String, password: String) {
        LoginUseCase::new(&self.sam_site_adapter)
            .execute(username, password)
            .await
            .unwrap()
    }

    pub async fn retrieve_students(&self) -> Result<Vec<SingleStudentViewModel>, String> {
        RetrieveStudentsUseCase::new(&self.sam_site_adapter)
            .execute()
            .await
            .map(|students| students.iter().map(SingleStudentViewModel::from).collect())
            .map_err(|e| format!("{:#?}", e))
    }

    pub async fn retrieve_student_lessons(
        &self,
        student_id: &str,
    ) -> Result<Vec<SingleLessonViewModel>, String> {
        RetrieveStudentLessonsUseCase::new(&self.sam_site_adapter)
            .execute(student_id)
            .await
            .map(|lessons| lessons.iter().map(SingleLessonViewModel::from).collect())
            .map_err(|e| format!("{:#?}", e))
    }
}
