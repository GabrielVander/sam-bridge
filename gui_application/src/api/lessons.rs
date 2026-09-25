use chrono::Datelike;
use student::application::gateways::StudentLessonsGatewayError;
use student::domain::entities::{Clef, Lesson, Range, StudentLessons};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangeDto {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClefDto {
    G,
    C,
    F,
}

/// A calendar date; Flutter decides how to write it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateDto {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonDto {
    pub id: Option<String>,
    pub date: Option<DateDto>,
    pub phase: Option<RangeDto>,
    pub page: Option<RangeDto>,
    pub lesson: Option<RangeDto>,
    pub clef: Option<ClefDto>,
    pub description: Option<String>,
    pub instructor: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentLessonsDto {
    pub msa: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveStudentLessonsOutcome {
    Success { lessons: StudentLessonsDto },
    Failure { report: ErrorReportDto },
}

impl From<Result<StudentLessons, StudentLessonsGatewayError>> for RetrieveStudentLessonsOutcome {
    fn from(result: Result<StudentLessons, StudentLessonsGatewayError>) -> Self {
        match result {
            Ok(lessons) => Self::Success {
                lessons: lessons.into(),
            },
            Err(error) => Self::Failure {
                report: error.into(),
            },
        }
    }
}

impl From<StudentLessons> for StudentLessonsDto {
    fn from(lessons: StudentLessons) -> Self {
        Self {
            msa: lessons.msa.into_iter().map(LessonDto::from).collect(),
            method: lessons.method.into_iter().map(LessonDto::from).collect(),
        }
    }
}

impl From<Lesson> for LessonDto {
    fn from(lesson: Lesson) -> Self {
        Self {
            id: lesson.id,
            date: lesson.date.map(DateDto::from),
            phase: lesson.phase.map(RangeDto::from),
            page: lesson.page.map(RangeDto::from),
            lesson: lesson.lesson.map(RangeDto::from),
            clef: lesson.clef.map(ClefDto::from),
            description: lesson.description,
            instructor: lesson.instructor,
            method: lesson.method,
        }
    }
}

impl From<chrono::NaiveDate> for DateDto {
    fn from(date: chrono::NaiveDate) -> Self {
        Self {
            year: date.year(),
            month: date.month(),
            day: date.day(),
        }
    }
}

impl From<Range> for RangeDto {
    fn from(range: Range) -> Self {
        Self {
            from: range.from,
            to: range.to,
        }
    }
}

impl From<Clef> for ClefDto {
    fn from(clef: Clef) -> Self {
        match clef {
            Clef::G => Self::G,
            Clef::C => Self::C,
            Clef::F => Self::F,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClefDto, DateDto, LessonDto, RangeDto, StudentLessonsDto};
    use student::domain::entities::{Clef, Lesson, Range, StudentLessons};

    #[test]
    fn every_clef_variant_is_mapped() {
        assert_eq!(ClefDto::from(Clef::G), ClefDto::G);
        assert_eq!(ClefDto::from(Clef::C), ClefDto::C);
        assert_eq!(ClefDto::from(Clef::F), ClefDto::F);
    }

    #[test]
    fn range_fields_are_mapped() {
        let range = Range::new("1".to_owned(), "2".to_owned());

        let mapped = RangeDto::from(range);

        assert_eq!(
            mapped,
            RangeDto {
                from: "1".to_owned(),
                to: "2".to_owned(),
            }
        );
    }

    #[test]
    fn a_full_lesson_bundle_is_mapped() {
        let lessons = StudentLessons {
            msa: vec![Lesson {
                id: Some("1".to_owned()),
                date: chrono::NaiveDate::from_ymd_opt(2025, 9, 9),
                phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
                page: None,
                lesson: None,
                clef: Some(Clef::G),
                description: Some("desc".to_owned()),
                instructor: Some("instructor".to_owned()),
                method: None,
            }],
            method: Vec::new(),
        };

        let mapped = StudentLessonsDto::from(lessons);

        assert_eq!(
            mapped,
            StudentLessonsDto {
                msa: vec![LessonDto {
                    id: Some("1".to_owned()),
                    date: Some(DateDto {
                        year: 2025,
                        month: 9,
                        day: 9,
                    }),
                    phase: Some(RangeDto {
                        from: "4.5".to_owned(),
                        to: "4.5".to_owned(),
                    }),
                    page: None,
                    lesson: None,
                    clef: Some(ClefDto::G),
                    description: Some("desc".to_owned()),
                    instructor: Some("instructor".to_owned()),
                    method: None,
                }],
                method: Vec::new(),
            }
        );
    }

    #[test]
    fn a_lesson_without_a_date_maps_to_no_date() {
        let mapped = LessonDto::from(Lesson::default());

        assert_eq!(mapped.date, None);
    }
}
