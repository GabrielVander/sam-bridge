use anyhow::anyhow;
use sam::client::MsaLesson;
use student_management::api::domain::Lesson;

use super::common::{parse_clef, parse_naive_date, parse_range};

pub fn map(dto: &MsaLesson) -> anyhow::Result<Lesson> {
    let id = dto.id.clone();
    if id.trim().is_empty() {
        anyhow::bail!("Id not present");
    }
    let date = parse_naive_date(&dto.date)?;
    let phase = if dto.phases.trim().is_empty() {
        None
    } else {
        Some(parse_range(&dto.phases)?)
    };
    let page = if dto.pages.trim().is_empty() {
        None
    } else {
        Some(parse_range(&dto.pages)?)
    };
    // lessons/clefs are Option<String> — None or empty → None
    let lesson = dto
        .lessons
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(parse_range)
        .transpose()?;
    let clef = dto
        .clefs
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(parse_clef)
        .transpose()?;
    let description = dto.description.clone().filter(|s| !s.trim().is_empty());
    let instructor = dto.authorizer.clone();
    if instructor.trim().is_empty() {
        return Err(anyhow!("Expected instructor value but none was found"));
    }

    Ok(Lesson {
        id,
        date,
        phase,
        page,
        lesson,
        clef,
        description,
        instructor,
        method: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MsaLesson;

    fn dto_with(overrides: impl FnOnce(&mut MsaLesson)) -> MsaLesson {
        let mut dto = MsaLesson {
            id: "559783".to_owned(),
            date: "09/09/2025".to_owned(),
            phases: "4.5 - 4.5".to_owned(),
            pages: "38 - 38".to_owned(),
            lessons: Some("7 - 8".to_owned()),
            clefs: Some("Sol".to_owned()),
            description: Some("Passou lições 7 e 8".to_owned()),
            authorizer: "MARCOS ROGÉRIO COSME".to_owned(),
        };
        overrides(&mut dto);
        dto
    }

    #[test]
    fn given_valid_msa_dto_should_map_to_generic_lesson() {
        let dto = dto_with(|_| {});
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.id, "559783");
        assert_eq!(lesson.instructor, "MARCOS ROGÉRIO COSME");
        assert_eq!(lesson.phase.unwrap().from, "4.5");
        assert_eq!(lesson.lesson.unwrap().from, "7");
        assert_eq!(lesson.clef.unwrap(), student_management::api::domain::Clef::G);
        assert_eq!(lesson.method, None);
    }

    #[test]
    fn given_empty_optional_fields_should_be_none() {
        let dto = dto_with(|d| {
            d.lessons = None;
            d.clefs = None;
            d.description = None;
        });
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.lesson, None);
        assert_eq!(lesson.clef, None);
        assert_eq!(lesson.description, None);
    }

    #[test]
    fn given_invalid_date_should_fail() {
        let dto = dto_with(|d| d.date = "invalid".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_invalid_clef_should_fail() {
        let dto = dto_with(|d| d.clefs = Some("Unknown".to_owned()));
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_id_should_fail() {
        let dto = dto_with(|d| d.id = "  ".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_instructor_should_fail() {
        let dto = dto_with(|d| d.authorizer = " ".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_phase_and_page_cells_should_map_to_none() {
        let dto = dto_with(|d| {
            d.phases = " ".to_owned();
            d.pages = " ".to_owned();
        });
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.phase, None);
        assert_eq!(lesson.page, None);
    }

    #[test]
    fn given_unparseable_phase_should_fail() {
        let dto = dto_with(|d| d.phases = "-".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_unparseable_page_should_fail() {
        let dto = dto_with(|d| d.pages = "-".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_unparseable_lessons_should_fail() {
        let dto = dto_with(|d| d.lessons = Some("-".to_owned()));
        assert!(map(&dto).is_err());
    }
}
