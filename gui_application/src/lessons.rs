use crate::view_models::{clef_label, range_label, LessonItem, LessonKind, StudentLessonsView};
use student_management::api::application::StudentLessonsDto;

pub fn to_view(bundle: &StudentLessonsDto) -> StudentLessonsView {
    let mut msa: Vec<LessonItem> = bundle
        .approved
        .iter()
        .map(|dto| LessonItem {
            kind: LessonKind::Msa,
            id: dto.id.clone().unwrap_or_default(),
            date: dto.date.map(|d| d.to_string()).unwrap_or_default(),
            phase: range_label(dto.phase.as_ref()),
            page: range_label(dto.page.as_ref()),
            lesson: range_label(dto.lesson.as_ref()),
            clef: dto.clef.as_ref().map(clef_label).unwrap_or_default(),
            description: dto.description.clone().unwrap_or_default(),
            instructor: dto.instructor.clone().unwrap_or_default(),
            method: dto.method.clone().unwrap_or_default(),
        })
        .collect();
    let mut method: Vec<LessonItem> = bundle
        .method
        .iter()
        .map(|dto| LessonItem {
            kind: LessonKind::Method,
            id: dto.id.clone().unwrap_or_default(),
            date: dto.date.map(|d| d.to_string()).unwrap_or_default(),
            phase: range_label(dto.phase.as_ref()),
            page: range_label(dto.page.as_ref()),
            lesson: range_label(dto.lesson.as_ref()),
            clef: dto.clef.as_ref().map(clef_label).unwrap_or_default(),
            description: dto.description.clone().unwrap_or_default(),
            instructor: dto.instructor.clone().unwrap_or_default(),
            method: dto.method.clone().unwrap_or_default(),
        })
        .collect();

    sort_most_recent_first(&mut msa);
    sort_most_recent_first(&mut method);

    StudentLessonsView {
        msa,
        method,
    }
}

fn sort_most_recent_first(items: &mut [LessonItem]) {
    items.sort_by(|a, b| b.date.cmp(&a.date));
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::application::{LessonDto, StudentLessonsDto};

    fn dto_on(id: &str, date: &str) -> LessonDto {
        LessonDto {
            id: Some(id.to_owned()),
            date: if date.is_empty() {
                None
            } else {
                chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d").ok()
            },
            phase: None,
            page: None,
            lesson: None,
            clef: None,
            description: None,
            instructor: None,
            method: None,
        }
    }

    #[test]
    fn given_shuffled_lessons_should_be_most_recent_first() {
        let bundle = StudentLessonsDto {
            approved: vec![
                dto_on("old", "2023-12-04"),
                dto_on("newest", "2026-03-24"),
                dto_on("middle", "2024-07-01"),
            ],
            method: vec![dto_on("m-mid", "2024-02-15"), dto_on("m-new", "2025-11-30")],
        };

        let view = to_view(&bundle);

        assert_eq!(
            view.msa.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
            vec!["newest", "middle", "old"]
        );
        assert_eq!(
            view.method.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
            vec!["m-new", "m-mid"]
        );
        assert_eq!(view.msa[0].kind, LessonKind::Msa);
        assert_eq!(view.method[0].kind, LessonKind::Method);
    }

    #[test]
    fn given_dateless_lessons_they_should_sink_to_the_end() {
        let bundle = StudentLessonsDto {
            approved: vec![dto_on("dated", "2024-01-01"), dto_on("undated", "")],
            method: vec![],
        };

        let view = to_view(&bundle);

        assert_eq!(
            view.msa.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
            vec!["dated", "undated"],
            "Dateless lessons must sink to the end"
        );
        assert_eq!(view.msa[1].date, "");
    }

    #[test]
    fn given_empty_bundle_should_produce_empty_lists() {
        let bundle = StudentLessonsDto {
            approved: vec![],
            method: vec![],
        };
        let view = to_view(&bundle);

        assert!(view.msa.is_empty());
        assert!(view.method.is_empty());
    }
}
