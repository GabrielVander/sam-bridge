use sam::client::MtdLesson;
use student_core::domain::entities::Lesson;

use crate::mapping::common::{parse_naive_date, parse_range};

pub fn map(dto: &MtdLesson) -> Lesson {
    Lesson {
        id: dto.id.clone().filter(|s| !s.trim().is_empty()),
        date: dto.date.as_deref().and_then(|d| parse_naive_date(d).ok()),
        phase: None,
        page: dto
            .pages
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_range(s).ok()),
        lesson: dto
            .lesson
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_range(s).ok()),
        clef: None,
        description: dto.observations.clone().filter(|s| !s.trim().is_empty()),
        instructor: dto.authorizer.clone().filter(|s| !s.trim().is_empty()),
        method: dto.method.clone().filter(|s| !s.trim().is_empty()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MtdLesson;

    fn dto_with(overrides: impl FnOnce(&mut MtdLesson)) -> MtdLesson {
        let mut dto = MtdLesson {
            id: Some("214020".to_owned()),
            pages: Some("00".to_owned()),
            lesson: Some("00".to_owned()),
            method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
            date: Some("04/12/2023".to_owned()),
            authorizer: Some("MURILO FAGNER CARDOSO".to_owned()),
            registration_date: Some("04/12/2023 21:17:17".to_owned()),
            observations: Some("Postura do violino".to_owned()),
        };
        overrides(&mut dto);
        dto
    }

    #[test]
    fn given_valid_method_dto_should_map_all_fields() {
        let lesson = map(&dto_with(|_| {}));

        assert_eq!(lesson.id.as_deref(), Some("214020"));
        assert_eq!(lesson.instructor.as_deref(), Some("MURILO FAGNER CARDOSO"));
        assert_eq!(lesson.page.as_ref().map(|r| r.from.as_str()), Some("00"));
        assert_eq!(lesson.lesson.as_ref().map(|r| r.from.as_str()), Some("00"));
        assert_eq!(
            lesson.method.as_deref(),
            Some("MÉTODO CCB - SCHIMOLL - VIOLINO")
        );
        assert_eq!(lesson.phase, None);
        assert_eq!(lesson.clef, None);
    }

    #[test]
    fn given_every_field_absent_should_map_to_default_lesson() {
        let lesson = map(&MtdLesson::default());

        assert_eq!(lesson, Lesson::default());
    }

    #[test]
    fn given_optional_lesson_and_observations_absent_should_be_none() {
        let lesson = map(&dto_with(|d| {
            d.lesson = None;
            d.observations = None;
        }));

        assert_eq!(lesson.lesson, None);
        assert_eq!(lesson.description, None);
    }

    #[test]
    fn given_unparseable_fields_should_degrade_to_none_not_fail() {
        let lesson = map(&dto_with(|d| {
            d.date = Some("bad".to_owned());
            d.pages = Some("-".to_owned());
            d.lesson = Some("-".to_owned());
        }));

        assert_eq!(lesson.date, None);
        assert_eq!(lesson.page, None);
        assert_eq!(lesson.lesson, None);
    }
}
