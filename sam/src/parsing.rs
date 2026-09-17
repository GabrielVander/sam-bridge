mod authentication_parser;
mod dashboard_parser;
mod dom;
mod method_lessons;
mod msa_lessons;
mod students_listing;

pub use authentication_parser::*;
pub use dashboard_parser::*;
pub use method_lessons::*;
pub use msa_lessons::*;
pub use students_listing::*;

pub fn parse_student_lessons_page(
    response_status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Result<StudentLessonsPage> {
    if response_status != reqwest::StatusCode::OK {
        anyhow::bail!("Unexpected status for student lessons response: {response_status:?}");
    }

    Ok(StudentLessonsPage {
        msa: parse_msa_lessons_body(body),
        method: parse_method_lessons_body(body),
    })
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct StudentLessonsPage {
    pub msa: Vec<MsaLesson>,
    pub method: Vec<MtdLesson>,
}
