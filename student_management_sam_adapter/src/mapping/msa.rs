use sam::client::MsaLesson;
use student_management::api::domain::Lesson;

use super::common::{parse_clef, parse_naive_date, parse_range};

pub fn map(dto: &MsaLesson) -> Lesson {
    Lesson {
        id: dto.id.clone().filter(|s| !s.trim().is_empty()),
        date: dto.date.as_deref().and_then(|d| parse_naive_date(d).ok()),
        phase: dto
            .phases
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_range(s).ok()),
        page: dto
            .pages
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_range(s).ok()),
        lesson: dto
            .lessons
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_range(s).ok()),
        clef: dto
            .clefs
            .as_deref()
            .filter(|s| !s.trim().is_empty())
            .and_then(|s| parse_clef(s).ok()),
        description: dto
            .description
            .clone()
            .filter(|s| !s.trim().is_empty()),
        instructor: dto
            .authorizer
            .clone()
            .filter(|s| !s.trim().is_empty()),
        method: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MsaLesson;
    use student_management::api::domain::{Clef, Range};

    fn dto_with(overrides: impl FnOnce(&mut MsaLesson)) -> MsaLesson {
        let mut dto = MsaLesson {
            id: Some("559783".to_owned()),
            date: Some("09/09/2025".to_owned()),
            phases: Some("4.5 - 4.5".to_owned()),
            pages: Some("38 - 38".to_owned()),
            lessons: Some("7 - 8".to_owned()),
            clefs: Some("Sol".to_owned()),
            description: Some("Passou lições 7 e 8".to_owned()),
            authorizer: Some("MARCOS ROGÉRIO COSME".to_owned()),
        };
        overrides(&mut dto);
        dto
    }

    #[test]
    fn given_valid_msa_dto_should_map_all_fields() {
        let lesson = map(&dto_with(|_| {}));

        assert_eq!(lesson.id.as_deref(), Some("559783"));
        assert_eq!(lesson.instructor.as_deref(), Some("MARCOS ROGÉRIO COSME"));
        assert_eq!(lesson.phase.as_ref().map(|r| r.from.as_str()), Some("4.5"));
        assert_eq!(lesson.lesson.as_ref().map(|r| r.from.as_str()), Some("7"));
        assert_eq!(lesson.clef, Some(Clef::G));
        assert_eq!(lesson.method, None);
    }

    #[test]
    fn given_every_field_absent_should_map_to_default_lesson() {
        let lesson = map(&MsaLesson::default());

        assert_eq!(lesson, Lesson::default());
    }

    #[test]
    fn given_whitespace_only_cells_should_be_none() {
        let lesson = map(&dto_with(|d| {
            d.phases = Some("  ".to_owned());
            d.description = Some(" ".to_owned());
            d.authorizer = Some(" ".to_owned());
        }));

        assert_eq!(lesson.phase, None);
        assert_eq!(lesson.description, None);
        assert_eq!(lesson.instructor, None);
    }

    #[test]
    fn given_unparseable_fields_should_degrade_to_none_not_fail() {
        let lesson = map(&dto_with(|d| {
            d.date = Some("not-a-date".to_owned());
            d.pages = Some("-".to_owned());
            d.clefs = Some("Unknown".to_owned());
        }));

        assert_eq!(lesson.date, None);
        assert_eq!(lesson.page, None);
        assert_eq!(lesson.clef, None);
    }

    #[test]
    fn given_single_value_range_should_duplicate_bounds() {
        let lesson = map(&dto_with(|d| {
            d.pages = Some("38".to_owned());
        }));

        assert_eq!(
            lesson.page,
            Some(Range {
                from: "38".to_owned(),
                to: "38".to_owned()
            })
        );
    }
}
