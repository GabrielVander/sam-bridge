use crate::view_models::{LessonItem, LessonKind, StudentLessonsView};
use student_management::api::domain::StudentLessons;

pub fn to_view(bundle: &StudentLessons) -> StudentLessonsView {
    let mut msa: Vec<LessonItem> = bundle
        .approved
        .iter()
        .map(|l| LessonItem::from_domain(LessonKind::Msa, l))
        .collect();
    let mut method: Vec<LessonItem> = bundle
        .method
        .iter()
        .map(|l| LessonItem::from_domain(LessonKind::Method, l))
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
    use chrono::NaiveDate;
    use student_management::api::domain::Lesson;

    fn lesson_on(id: &str, y: i32, m: u32, d: u32) -> Lesson {
        Lesson {
            id: Some(id.to_owned()),
            date: NaiveDate::from_ymd_opt(y, m, d),
            ..Default::default()
        }
    }

    #[test]
    fn given_shuffled_lessons_should_be_most_recent_first() {
        let bundle = StudentLessons {
            approved: vec![
                lesson_on("old", 2023, 12, 4),
                lesson_on("newest", 2026, 3, 24),
                lesson_on("middle", 2024, 7, 1),
            ],
            method: vec![
                lesson_on("m-mid", 2024, 2, 15),
                lesson_on("m-new", 2025, 11, 30),
            ],
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
        let mut undated = lesson_on("undated", 2000, 1, 1);
        undated.date = None;

        let bundle = StudentLessons {
            approved: vec![lesson_on("dated", 2024, 1, 1), undated],
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
        let bundle = StudentLessons::default();
        let view = to_view(&bundle);

        assert!(view.msa.is_empty());
        assert!(view.method.is_empty());
    }
}
