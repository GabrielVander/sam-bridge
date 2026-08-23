use async_trait::async_trait;
use sam::client::{LessonsReader, StudentLessonsPage};
use student_management::api::{
    application::StudentLessonsGateway,
    domain::StudentLessons,
};

pub struct SamLessonsGateway<S: LessonsReader> {
    source: S,
}

impl<S: LessonsReader> SamLessonsGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> StudentLessonsGateway for SamLessonsGateway<S>
where
    S: LessonsReader + Clone + Send + Sync + 'static,
{
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<StudentLessons> {
        let source = self.source.clone();
        let id = id.to_owned();
        // sam is blocking: run on smol's thread pool.
        let page: StudentLessonsPage = smol::unblock(move || source.student_lessons(&id)).await?;

        Ok(StudentLessons {
            approved: page.msa.iter().map(crate::mapping::msa::map).collect(),
            method: page.method.iter().map(crate::mapping::method::map).collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::{MsaLesson, MtdLesson};
    use student_management::api::application::StudentLessonsGateway;

    #[derive(Clone)]
    struct StubLessonsReader {
        page: StudentLessonsPage,
    }

    impl LessonsReader for StubLessonsReader {
        fn student_lessons(&self, _id: &str) -> anyhow::Result<StudentLessonsPage> {
            Ok(self.page.clone())
        }
    }

    #[derive(Clone)]
    struct FailingLessonsReader;

    impl LessonsReader for FailingLessonsReader {
        fn student_lessons(&self, _id: &str) -> anyhow::Result<StudentLessonsPage> {
            anyhow::bail!("Student lessons request failed")
        }
    }

    #[test]
    fn maps_both_kinds_from_the_reader_page() {
        smol::block_on(async {
            let stub = StubLessonsReader {
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
    fn empty_page_maps_to_empty_bundle() {
        smol::block_on(async {
            let gateway = SamLessonsGateway::new(StubLessonsReader {
                page: StudentLessonsPage::default(),
            });

            let lessons = gateway.get_all_for_student_with_id("1").await.expect("Should load");

            assert!(lessons.approved.is_empty());
            assert!(lessons.method.is_empty());
        });
    }

    #[test]
    fn reader_errors_propagate() {
        smol::block_on(async {
            let gateway = SamLessonsGateway::new(FailingLessonsReader);
            let result = gateway.get_all_for_student_with_id("1").await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Student lessons"));
        });
    }
}
