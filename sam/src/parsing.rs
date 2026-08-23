mod authentication;
mod method_lessons;
mod msa_lessons;
mod session;
mod students_listing;

pub(crate) use authentication::{AuthOutcome, parse_authentication};
pub(crate) use method_lessons::parse_method_lessons_body;
pub(crate) use msa_lessons::parse_msa_lessons_body;
pub(crate) use session::parse_session_status;

pub use method_lessons::MtdLesson;
pub use msa_lessons::MsaLesson;
pub use students_listing::SamStudent;
pub(crate) use students_listing::parse_students_listing;

pub(crate) fn parse_student_lessons_page(
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StudentLessonsPage {
    pub msa: Vec<MsaLesson>,
    pub method: Vec<MtdLesson>,
}
