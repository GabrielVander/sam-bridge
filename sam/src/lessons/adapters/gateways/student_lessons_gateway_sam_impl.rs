use std::sync::Arc;
use student::application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError};
use student::domain::entities::{Clef, Lesson, Range, StudentId, StudentLessons};

use crate::client::{MsaLesson, MtdLesson, SamClient, StudentLessonsPage};
use crate::diagnostics::{error_chain, failure_kind};

pub struct StudentLessonsGatewaySamImpl {
    client: Arc<dyn SamClient>,
}

impl StudentLessonsGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient>) -> Self {
        Self { client }
    }
}
impl StudentLessonsGateway for StudentLessonsGatewaySamImpl {
    fn get_all_for_student_with_id(
        &self,
        id: &StudentId,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        let page: StudentLessonsPage =
            self.client.student_lessons(id.as_str()).map_err(|error| {
                StudentLessonsGatewayError::UnableToPerformOperation {
                    kind: failure_kind(&error),
                    details: error_chain(&error),
                }
            })?;

        Ok(StudentLessons {
            msa: page.msa.into_iter().map(Lesson::from).collect(),
            method: page.method.into_iter().map(Lesson::from).collect(),
        })
    }
}

impl From<MsaLesson> for Lesson {
    fn from(lesson: MsaLesson) -> Self {
        Self {
            id: lesson.id,
            date: lesson.date.as_deref().and_then(parse_date),
            phase: lesson.phases.as_deref().and_then(parse_range),
            page: lesson.pages.as_deref().and_then(parse_range),
            lesson: lesson.lessons.as_deref().and_then(parse_range),
            clef: lesson.clefs.as_deref().and_then(parse_clef),
            description: lesson.description,
            instructor: lesson.authorizer,
            method: None,
        }
    }
}

impl From<MtdLesson> for Lesson {
    fn from(lesson: MtdLesson) -> Self {
        Self {
            id: lesson.id,
            date: lesson.date.as_deref().and_then(parse_date),
            phase: None,
            page: lesson.pages.as_deref().and_then(parse_range),
            lesson: lesson.lesson.as_deref().and_then(parse_range),
            clef: None,
            description: lesson.observations,
            instructor: lesson.authorizer,
            method: lesson.method,
        }
    }
}

fn parse_date(raw: &str) -> Option<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(raw, "%d/%m/%Y").ok()
}

fn parse_range(raw: &str) -> Option<Range> {
    let trimmed: &str = raw.trim();
    if trimmed.is_empty() {
        return None;
    }

    trimmed.split_once(" - ").map_or_else(
        || Some(Range::single(trimmed.to_owned())),
        |(from, to)| Some(Range::new(from.trim().to_owned(), to.trim().to_owned())),
    )
}

fn parse_clef(raw: &str) -> Option<Clef> {
    match raw.trim() {
        "Sol" => Some(Clef::G),
        "Dó" | "Do" => Some(Clef::C),
        "Fá" | "Fa" => Some(Clef::F),
        _ => None,
    }
}
