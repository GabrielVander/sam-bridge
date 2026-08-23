use sam::client::MtdLesson;
use student_management::api::domain::Lesson;

use super::common::{parse_naive_date, parse_range};

pub fn map(dto: &MtdLesson) -> anyhow::Result<Lesson> {
    let id = dto.id.clone();
    if id.trim().is_empty() {
        anyhow::bail!("Id not present");
    }
    let date = parse_naive_date(&dto.date)?;
    // Method lessons: pages is always present, lesson is optional
    let page = {
        let raw = dto.pages.trim();
        if raw.is_empty() {
            None
        } else {
            Some(parse_range(raw)?)
        }
    };
    let lesson = dto
        .lesson
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(parse_range)
        .transpose()?;
    let method = Some(dto.method.clone()).filter(|s| !s.trim().is_empty());
    let description = dto.observations.clone().filter(|s| !s.trim().is_empty());
    let instructor = dto.authorizer.clone();
    if instructor.trim().is_empty() {
        anyhow::bail!("Expected instructor value but none was found");
    }

    Ok(Lesson {
        id,
        date,
        phase: None,
        page,
        lesson,
        clef: None,
        description,
        instructor,
        method,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MtdLesson;

    fn dto_with(overrides: impl FnOnce(&mut MtdLesson)) -> MtdLesson {
        let mut dto = MtdLesson {
            id: "214020".to_owned(),
            pages: "00".to_owned(),
            lesson: Some("00".to_owned()),
            method: "MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned(),
            date: "04/12/2023".to_owned(),
            authorizer: "MURILO FAGNER CARDOSO".to_owned(),
            registration_date: "04/12/2023 21:17:17".to_owned(),
            observations: Some("Postura do violino".to_owned()),
        };
        overrides(&mut dto);
        dto
    }

    #[test]
    fn given_valid_method_dto_should_map_to_generic_lesson() {
        let dto = dto_with(|_| {});
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.id, "214020");
        assert_eq!(lesson.instructor, "MURILO FAGNER CARDOSO");
        assert_eq!(lesson.page.unwrap().from, "00");
        assert_eq!(lesson.lesson.unwrap().from, "00");
        assert_eq!(
            lesson.method.unwrap(),
            "MÉTODO CCB - SCHIMOLL - VIOLINO"
        );
        assert_eq!(lesson.phase, None);
        assert_eq!(lesson.clef, None);
    }

    #[test]
    fn given_empty_optional_lesson_should_be_none() {
        let dto = dto_with(|d| d.lesson = None);
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.lesson, None);
    }

    #[test]
    fn given_invalid_date_should_fail() {
        let dto = dto_with(|d| d.date = "bad".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_id_should_fail() {
        let dto = dto_with(|d| d.id = " ".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_instructor_should_fail() {
        let dto = dto_with(|d| d.authorizer = "  ".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_empty_pages_cell_should_map_to_none() {
        let dto = dto_with(|d| d.pages = " ".to_owned());
        let lesson = map(&dto).unwrap();
        assert_eq!(lesson.page, None);
    }

    #[test]
    fn given_unparseable_page_should_fail() {
        let dto = dto_with(|d| d.pages = "-".to_owned());
        assert!(map(&dto).is_err());
    }

    #[test]
    fn given_unparseable_lesson_should_fail() {
        let dto = dto_with(|d| d.lesson = Some("-".to_owned()));
        assert!(map(&dto).is_err());
    }
}
