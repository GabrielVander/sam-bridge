use std::sync::Arc;
use student::application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError};
use student::domain::entities::{Clef, Lesson, Range, StudentId, StudentLessons};

use crate::client::{MsaLesson, MtdLesson, SamClient, StudentLessonsPage};
use crate::diagnostics::{error_chain, failure_kind};
use crate::lessons::adapters::gateways::method_books::recognize_method_books;

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
            method_books: Vec::new(),
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
            method_books: lesson
                .method
                .as_deref()
                .map(recognize_method_books)
                .unwrap_or_default(),
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

    if let Some((from, to)) = trimmed.split_once(" - ") {
        return Some(Range::new(from.trim().to_owned(), to.trim().to_owned()));
    }

    parse_listed_range(trimmed)
}

fn parse_listed_range(listed: &str) -> Option<Range> {
    let mut values = listed
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let first: &str = values.next()?;

    Some(values.next_back().map_or_else(
        || Range::single(first.to_owned()),
        |last| Range::new(first.to_owned(), last.to_owned()),
    ))
}

fn parse_clef(raw: &str) -> Option<Clef> {
    match raw.trim() {
        "Sol" => Some(Clef::G),
        "Dó" | "Do" => Some(Clef::C),
        "Fá" | "Fa" => Some(Clef::F),
        _ => None,
    }
}
