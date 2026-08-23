use async_trait::async_trait;
use sam::client::StudentLessonsPage;
use student_management::api::{
    application::StudentLessonsGateway,
    domain::StudentLessons,
};

use crate::ports::LessonsSource;

#[derive(Clone)]
pub struct SamLessonsGateway<S: LessonsSource> {
    source: S,
}

impl<S: LessonsSource> SamLessonsGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> StudentLessonsGateway for SamLessonsGateway<S>
where
    S: LessonsSource + Clone + Send + Sync + 'static,
{
    async fn get_all_for_student_with_id(
        &self,
        id: &str,
    ) -> anyhow::Result<StudentLessons> {
        let source = self.source.clone();
        let id = id.to_owned();
        // sam is blocking — offload to thread pool so we don't block the async executor
        let page: StudentLessonsPage = smol::unblock(move || source.student_lessons(&id)).await?;

        Ok(StudentLessons {
            approved: page.msa.iter().map(crate::mapping::msa::map).collect(),
            method: page.method.iter().map(crate::mapping::method::map).collect(),
        })
    }
}

/// Domain `Lesson` mapping happens in `crate::mapping`.
#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::{MsaLesson, MtdLesson};
    use student_management::api::application::StudentLessonsGateway;

    use crate::ports::LessonsSource;

    #[derive(Clone)]
    struct StubLessonsSource {
        page: StudentLessonsPage,
    }

    impl LessonsSource for StubLessonsSource {
        fn student_lessons(&self, _id: &str) -> anyhow::Result<StudentLessonsPage> {
            Ok(self.page.clone())
        }
    }

    #[derive(Clone)]
    struct FailingLessonsSource;

    impl LessonsSource for FailingLessonsSource {
        fn student_lessons(&self, _id: &str) -> anyhow::Result<StudentLessonsPage> {
            anyhow::bail!("Student lessons request failed")
        }
    }

    #[test]
    fn given_lessons_gateway_should_map_both_kinds() {
        smol::block_on(async {
            let stub = StubLessonsSource {
                page: StudentLessonsPage {
                    msa: vec![MsaLesson {
                        id: Some("559783".to_owned()),
                        authorizer: Some("MARCOS ROGÉRIO COSME".to_owned()),
                        ..Default::default()
                    }],
                    method: vec![MtdLesson {
                        id: Some("214020".to_owned()),
                        method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                        ..Default::default()
                    }],
                },
            };
            let gateway = SamLessonsGateway::new(stub);

            let lessons = gateway
                .get_all_for_student_with_id("500132")
                .await
                .expect("Should load");

            assert_eq!(lessons.approved.len(), 1);
            assert_eq!(lessons.method.len(), 1);
            assert_eq!(lessons.approved[0].id.as_deref(), Some("559783"));
            assert_eq!(lessons.method[0].id.as_deref(), Some("214020"));
        });
    }

    #[test]
    fn given_empty_page_should_return_empty_bundle_not_error() {
        smol::block_on(async {
            let gateway = SamLessonsGateway::new(StubLessonsSource {
                page: StudentLessonsPage::default(),
            });

            let lessons = gateway.get_all_for_student_with_id("500132").await.expect("Should load");

            assert!(lessons.approved.is_empty());
            assert!(lessons.method.is_empty());
        });
    }

    #[test]
    fn given_source_failure_should_propagate_error() {
        smol::block_on(async {
            let gateway = SamLessonsGateway::new(FailingLessonsSource);

            let result = gateway.get_all_for_student_with_id("500132").await;

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Student lessons request failed"));
        });
    }
}
