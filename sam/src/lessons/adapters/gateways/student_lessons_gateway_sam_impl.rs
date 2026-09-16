use std::sync::Arc;

use async_trait::async_trait;
use student::application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError};
use student::domain::entities::{Clef, Lesson, Range, StudentLessons};

use crate::client::{MsaLesson, MtdLesson, SamClient, StudentLessonsPage};

pub struct StudentLessonsGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl StudentLessonsGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl StudentLessonsGateway for StudentLessonsGatewaySamImpl {
    async fn get_all_for_student_with_id(
        &self,
        id: &str,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        let page: StudentLessonsPage = self
            .client
            .student_lessons(id)
            .map_err(|_| StudentLessonsGatewayError::UnableToPerformOperation)?;

        Ok(StudentLessons {
            approved: page.msa.into_iter().map(Lesson::from).collect(),
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

#[cfg(test)]
mod tests {
    use super::{parse_clef, parse_date, parse_range};
    use student::domain::entities::{Clef, Range};

    #[test]
    fn parses_valid_date() {
        assert_eq!(
            parse_date("09/09/2025"),
            chrono::NaiveDate::from_ymd_opt(2025, 9, 9)
        );
    }

    #[test]
    fn unparseable_date_is_none() {
        assert_eq!(parse_date("not-a-date"), None);
    }

    #[test]
    fn parses_two_sided_range() {
        assert_eq!(
            parse_range("7 - 8"),
            Some(Range {
                from: "7".to_owned(),
                to: "8".to_owned()
            })
        );
    }

    #[test]
    fn parses_single_value_range() {
        assert_eq!(
            parse_range("00"),
            Some(Range {
                from: "00".to_owned(),
                to: "00".to_owned()
            })
        );
    }

    #[test]
    fn empty_range_is_none() {
        assert_eq!(parse_range(""), None);
        assert_eq!(parse_range("   "), None);
    }

    #[test]
    fn known_clefs_are_recognized() {
        assert_eq!(parse_clef("Sol"), Some(Clef::G));
        assert_eq!(parse_clef("Dó"), Some(Clef::C));
        assert_eq!(parse_clef("Fá"), Some(Clef::F));
    }

    #[test]
    fn unknown_clef_falls_through_to_none() {
        assert_eq!(parse_clef("Xyz"), None);
    }
}
