use crate::view_models::{LessonItem, LessonKind, StudentLessonsView};
use student_management::api::application::StudentLessonsGateway;
use student_management::api::domain::StudentLessons;

/// Loads both lesson kinds for a student and orders each list most-recent-first.
pub async fn load(
    lessons_gateway: &(dyn StudentLessonsGateway + Send + Sync),
    student_id: &str,
) -> anyhow::Result<StudentLessonsView> {
    let bundle: StudentLessons =
        lessons_gateway.get_all_for_student_with_id(student_id).await?;

    // Sort on typed dates first; ISO strings in view models keep the order.
    // Dateless lessons naturally sink to the end via Option ordering.
    let mut approved = bundle.approved;
    sort_most_recent_first(&mut approved);
    let mut method = bundle.method;
    sort_most_recent_first(&mut method);

    Ok(StudentLessonsView {
        msa: approved
            .iter()
            .map(|l| LessonItem::from_domain(LessonKind::Msa, l))
            .collect(),
        method: method
            .iter()
            .map(|l| LessonItem::from_domain(LessonKind::Method, l))
            .collect(),
    })
}

fn sort_most_recent_first(items: &mut [student_management::api::domain::Lesson]) {
    items.sort_by_key(|lesson| std::cmp::Reverse(lesson.date));
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use chrono::NaiveDate;
    use std::sync::Arc;
    use student_management::api::domain::Lesson;

    fn lesson_on(id: &str, date: Option<(i32, u32, u32)>) -> Lesson {
        Lesson {
            id: Some(id.to_owned()),
            date: date.map(|(y, m, d)| NaiveDate::from_ymd_opt(y, m, d).expect("valid date")),
            ..Default::default()
        }
    }

    struct FakeLessonsGateway {
        bundle: StudentLessons,
        fail: bool,
    }

    #[async_trait]
    impl StudentLessonsGateway for FakeLessonsGateway {
        async fn get_all_for_student_with_id(
            &self,
            _id: &str,
        ) -> anyhow::Result<StudentLessons> {
            if self.fail {
                anyhow::bail!("Student lessons request failed");
            }
            Ok(self.bundle.clone())
        }
    }

    #[test]
    fn given_shuffled_lessons_both_lists_should_be_most_recent_first() {
        smol::block_on(async {
            let gateway = FakeLessonsGateway {
                bundle: StudentLessons {
                    approved: vec![
                        lesson_on("old", Some((2023, 12, 4))),
                        lesson_on("newest", Some((2026, 3, 24))),
                        lesson_on("middle", Some((2024, 7, 1))),
                    ],
                    method: vec![
                        lesson_on("m-mid", Some((2024, 2, 15))),
                        lesson_on("m-old", Some((2023, 5, 5))),
                        lesson_on("m-new", Some((2025, 11, 30))),
                    ],
                },
                fail: false,
            };

            let view = load(&gateway, "500132").await.expect("Should load");

            let msa_ids: Vec<&str> = view.msa.iter().map(|i| i.id.as_str()).collect();
            let method_ids: Vec<&str> = view.method.iter().map(|i| i.id.as_str()).collect();

            assert_eq!(msa_ids, vec!["newest", "middle", "old"]);
            assert_eq!(method_ids, vec!["m-new", "m-mid", "m-old"]);
            assert_eq!(view.msa[0].kind, LessonKind::Msa);
            assert_eq!(view.method[0].kind, LessonKind::Method);
        });
    }

    #[test]
    fn given_dateless_lessons_they_should_sink_to_the_end() {
        smol::block_on(async {
            let gateway = FakeLessonsGateway {
                bundle: StudentLessons {
                    approved: vec![
                        lesson_on("dated", Some((2024, 1, 1))),
                        lesson_on("undated", None),
                    ],
                    method: vec![],
                },
                fail: false,
            };

            let view = load(&gateway, "500132").await.expect("Should load");

            assert_eq!(
                view.msa.iter().map(|i| i.id.as_str()).collect::<Vec<_>>(),
                vec!["dated", "undated"]
            );
            assert_eq!(view.msa[1].date, "", "Dateless lesson renders empty date");
        });
    }

    #[test]
    fn given_same_day_lessons_should_keep_source_order_within_day() {
        smol::block_on(async {
            let gateway = FakeLessonsGateway {
                bundle: StudentLessons {
                    approved: vec![
                        lesson_on("first-same-day", Some((2024, 1, 1))),
                        lesson_on("second-same-day", Some((2024, 1, 1))),
                    ],
                    method: vec![],
                },
                fail: false,
            };

            let view = load(&gateway, "500132").await.expect("Should load");

            assert_eq!(view.msa.len(), 2);
            assert_eq!(view.msa[0].id.as_str(), "first-same-day");
            assert_eq!(view.msa[1].id.as_str(), "second-same-day");
        });
    }

    #[test]
    fn given_no_lessons_view_should_be_empty_not_error() {
        smol::block_on(async {
            let gateway = FakeLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };

            let view = load(&gateway, "500132").await.expect("Should load");

            assert!(view.msa.is_empty());
            assert!(view.method.is_empty());
        });
    }

    #[test]
    fn given_gateway_failure_should_propagate_error() {
        smol::block_on(async {
            let gateway = FakeLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };

            let result = load(&gateway, "500132").await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Student"));
        });
    }

    #[test]
    fn fakes_should_be_shareable_across_threads_via_arc() {
        let gateway: Arc<FakeLessonsGateway> = Arc::new(FakeLessonsGateway {
            bundle: StudentLessons::default(),
            fail: false,
        });
        let _clone = Arc::clone(&gateway);
    }
}
