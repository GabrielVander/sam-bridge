#![allow(dead_code)]

use sam::client::{
    SamClient, SamClientError, SamClientImpl, SamCredentials, SamStudent, StudentLessonsPage,
};
use sam::http::SamOperations;

pub const UNREACHABLE_SITE: &str = "http://127.0.0.1:1";

pub fn sam_operations_for(base_url: &str) -> Result<SamOperations, reqwest::Error> {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()?;

    Ok(SamOperations::new(
        http_client,
        base_url,
        "autenticar",
        "painel",
        "alunos/listagem",
        "licoes/index",
    ))
}

pub fn sam_client_for(base_url: &str) -> Result<SamClientImpl, reqwest::Error> {
    sam_operations_for(base_url).map(SamClientImpl::new)
}

#[derive(Default)]
pub struct FakeSamClient {
    students: Vec<SamStudent>,
    lessons_page: StudentLessonsPage,
}

impl FakeSamClient {
    #[must_use]
    pub fn listing(students: Vec<SamStudent>) -> Self {
        Self {
            students,
            ..Self::default()
        }
    }

    #[must_use]
    pub fn showing_lessons(lessons_page: StudentLessonsPage) -> Self {
        Self {
            lessons_page,
            ..Self::default()
        }
    }
}

impl SamClient for FakeSamClient {
    fn login(&self, _credentials: &SamCredentials) -> Result<(), SamClientError> {
        Ok(())
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        Ok(self.students.clone())
    }

    fn student_lessons(&self, _student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        Ok(self.lessons_page.clone())
    }
}

#[must_use]
pub fn sam_student(role: &str, level: &str, instrument: &str, location: &str) -> SamStudent {
    SamStudent {
        id: "99999".to_owned(),
        name: "CARLOS ALBERTO DE NOBREGA".to_owned(),
        location: location.to_owned(),
        role: role.to_owned(),
        instrument: instrument.to_owned(),
        level: level.to_owned(),
    }
}
