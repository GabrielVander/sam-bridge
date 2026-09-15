use crate::lessons::domain::entities::{Clef, Lesson, Range, StudentLessons};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonDto {
    pub id: Option<String>,
    pub date: Option<chrono::NaiveDate>,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: Option<String>,
    pub method: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentLessonsDto {
    pub approved: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}

impl From<StudentLessons> for StudentLessonsDto {
    fn from(lessons: StudentLessons) -> Self {
        Self {
            approved: lessons.approved.into_iter().map(LessonDto::from).collect(),
            method: lessons.method.into_iter().map(LessonDto::from).collect(),
        }
    }
}

impl From<Lesson> for LessonDto {
    fn from(lesson: Lesson) -> Self {
        Self {
            id: lesson.id,
            date: lesson.date,
            phase: lesson.phase,
            page: lesson.page,
            lesson: lesson.lesson,
            clef: lesson.clef,
            description: lesson.description,
            instructor: lesson.instructor,
            method: lesson.method,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LessonDto, StudentLessonsDto};
    use crate::lessons::domain::entities::{Clef, Lesson, Range, StudentLessons};
    use chrono::NaiveDate;
    use pretty_assertions::assert_eq;

    #[test]
    fn maps_a_full_bundle() {
        let lessons: StudentLessons = StudentLessons {
            approved: vec![Lesson {
                id: Some("1".to_owned()),
                date: NaiveDate::from_ymd_opt(2025, 9, 9),
                phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
                page: None,
                lesson: None,
                clef: Some(Clef::G),
                description: Some("desc".to_owned()),
                instructor: Some("instructor".to_owned()),
                method: None,
            }],
            method: vec![Lesson::default()],
        };

        let dto: StudentLessonsDto = lessons.into();

        assert_eq!(
            dto,
            StudentLessonsDto {
                approved: vec![LessonDto {
                    id: Some("1".to_owned()),
                    date: NaiveDate::from_ymd_opt(2025, 9, 9),
                    phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
                    page: None,
                    lesson: None,
                    clef: Some(Clef::G),
                    description: Some("desc".to_owned()),
                    instructor: Some("instructor".to_owned()),
                    method: None,
                }],
                method: vec![LessonDto {
                    id: None,
                    date: None,
                    phase: None,
                    page: None,
                    lesson: None,
                    clef: None,
                    description: None,
                    instructor: None,
                    method: None,
                }],
            }
        );
    }
}
