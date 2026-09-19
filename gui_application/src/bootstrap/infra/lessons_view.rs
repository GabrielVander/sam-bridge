use student::application::dto as student_dto;
use student::domain::entities::Clef;

use crate::infra::ErrorReportDto;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonDto {
    pub id: Option<String>,
    pub date: Option<String>,
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
    pub approved: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveStudentLessonsOutcome {
    Success(StudentLessonsDto),
    Failure(ErrorReportDto),
}

impl From<student_dto::StudentLessonsDto> for StudentLessonsDto {
    fn from(dto: student_dto::StudentLessonsDto) -> Self {
        Self {
            approved: dto.approved.into_iter().map(LessonDto::from).collect(),
            method: dto.method.into_iter().map(LessonDto::from).collect(),
        }
    }
}

impl From<student_dto::LessonDto> for LessonDto {
    fn from(dto: student_dto::LessonDto) -> Self {
        Self {
            id: dto.id,
            date: dto.date.map(|d| d.format("%d/%m/%Y").to_string()),
            phase: dto.phase.map(RangeDto::from),
            page: dto.page.map(RangeDto::from),
            lesson: dto.lesson.map(RangeDto::from),
            clef: dto.clef.map(ClefDto::from),
            description: dto.description,
            instructor: dto.instructor,
            method: dto.method,
        }
    }
}

impl From<student::domain::entities::Range> for RangeDto {
    fn from(range: student::domain::entities::Range) -> Self {
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
    use super::{student_dto, ClefDto, LessonDto, RangeDto, StudentLessonsDto};
    use student::domain::entities::{Clef, Range};

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
        let dto = student_dto::StudentLessonsDto {
            approved: vec![student_dto::LessonDto {
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

        let mapped = StudentLessonsDto::from(dto);

        assert_eq!(
            mapped,
            StudentLessonsDto {
                approved: vec![LessonDto {
                    id: Some("1".to_owned()),
                    date: Some("09/09/2025".to_owned()),
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
        let dto = student_dto::LessonDto {
            id: None,
            date: None,
            phase: None,
            page: None,
            lesson: None,
            clef: None,
            description: None,
            instructor: None,
            method: None,
        };

        let mapped = LessonDto::from(dto);

        assert_eq!(mapped.date, None);
    }
}
