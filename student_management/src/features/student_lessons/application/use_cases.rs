use crate::api::{application::StudentLessonsGateway, domain::StudentLessons};

pub struct RetrieveStudentLessonsUseCase<'a, T: StudentLessonsGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentLessonsGateway> RetrieveStudentLessonsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, student_id: &str) -> anyhow::Result<StudentLessons> {
        self.gateway.get_all_for_student_with_id(student_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::domain::{Clef, Lesson, Range};
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
            _student_id: &str,
        ) -> anyhow::Result<StudentLessons> {
            if self.fail {
                anyhow::bail!("Student lessons request failed");
            }
            Ok(self.bundle.clone())
        }
    }

    fn fixture_bundle() -> StudentLessons {
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
            let bundle = fixture_bundle();
            let gateway = FakeStudentLessonsGateway {
                bundle: bundle.clone(),
                fail: false,
            };
            let use_case = RetrieveStudentLessonsUseCase::new(&gateway);

            let result = use_case.execute("500132").await.expect("should succeed");

            assert_eq!(result, bundle);
        });
    }

    #[test]
    fn propagates_gateway_errors() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };
            let use_case = RetrieveStudentLessonsUseCase::new(&gateway);

            let result = use_case.execute("500132").await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Student lessons"));
        });
    }
}
