#![allow(non_snake_case)]
use std::sync::Arc;

use crate::features::student_lessons::application::dto::{LessonDto, StudentLessonsDto};
use crate::features::student_lessons::application::gateways::StudentLessonsGateway;
use crate::features::student_lessons::domain::entities::Lesson;

pub struct RetrieveStudentLessonsUseCase {
    gateway: Arc<dyn StudentLessonsGateway>,
}

impl RetrieveStudentLessonsUseCase {
    pub fn new(gateway: Arc<dyn StudentLessonsGateway>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, studentId: &str) -> anyhow::Result<StudentLessonsDto> {
        let bundle = self.gateway.get_all_for_student_with_id(studentId).await?;
        Ok(StudentLessonsDto {
            approved: bundle.approved.iter().map(mapLesson).collect(),
            method: bundle.method.iter().map(mapLesson).collect(),
        })
    }
}

fn mapLesson(l: &Lesson) -> LessonDto {
    LessonDto {
        id: l.id.clone(),
        date: l.date,
        phase: l.phase.clone(),
        page: l.page.clone(),
        lesson: l.lesson.clone(),
        clef: l.clef.clone(),
        description: l.description.clone(),
        instructor: l.instructor.clone(),
        method: l.method.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::domain::{Clef, Lesson, Range, StudentLessons};
    use async_trait::async_trait;
    use chrono::NaiveDate;

    struct FakeStudentLessonsGateway {
        bundle: StudentLessons,
        fail: bool,
    }

    #[async_trait]
    impl StudentLessonsGateway for FakeStudentLessonsGateway {
        async fn get_all_for_student_with_id(
            &self,
            _studentId: &str,
        ) -> anyhow::Result<StudentLessons> {
            if self.fail {
                anyhow::bail!("Student lessons request failed");
            }
            Ok(self.bundle.clone())
        }
    }

    fn fixtureBundle() -> StudentLessons {
        StudentLessons {
            approved: vec![Lesson {
                id: Some("559783".to_owned()),
                date: Some(NaiveDate::from_ymd_opt(2025, 9, 9).expect("valid date")),
                phase: Some(Range {
                    from: "4.5".to_owned(),
                    to: "4.5".to_owned(),
                }),
                page: None,
                lesson: None,
                clef: Some(Clef::G),
                description: None,
                instructor: Some("MARCOS ROGÉRIO COSME".to_owned()),
                method: None,
            }],
            method: vec![Lesson::default()],
        }
    }

    #[test]
    fn returns_the_bundle_from_the_gateway() {
        smol::block_on(async {
            let bundle = fixtureBundle();
            let gateway = FakeStudentLessonsGateway {
                bundle: bundle.clone(),
                fail: false,
            };
            let use_case = RetrieveStudentLessonsUseCase::new(Arc::new(gateway));

            let result = use_case.execute("500132").await.expect("should succeed");

            assert_eq!(result.approved.len(), 1);
            assert_eq!(result.approved[0].id, Some("559783".to_owned()));
            assert_eq!(result.approved[0].phase, Some(Range { from: "4.5".to_owned(), to: "4.5".to_owned() }));
            assert_eq!(result.approved[0].clef, Some(Clef::G));
            assert_eq!(result.method.len(), 1);
        });
    }

    #[test]
    fn propagates_gateway_errors() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };
            let use_case = RetrieveStudentLessonsUseCase::new(Arc::new(gateway));

            let result = use_case.execute("500132").await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Student lessons"));
        });
    }
}