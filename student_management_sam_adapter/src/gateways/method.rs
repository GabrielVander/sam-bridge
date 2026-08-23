use async_trait::async_trait;
use sam::client::{Authenticated, SamClient};
use student_management::api::{application::MethodGateway, domain::Lesson};

use crate::{mapping::method as method_mapping, ports::MethodSource};

pub struct SamMethodGateway<S: MethodSource = SamClient<Authenticated>> {
    source: S,
}

impl<S: MethodSource> SamMethodGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> MethodGateway for SamMethodGateway<S>
where
    S: MethodSource + Clone + Send + Sync + 'static,
{
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        let source = self.source.clone();
        let id = id.to_owned();
        let dtos = smol::unblock(move || source.method_lessons(&id)).await?;
        dtos.iter().map(method_mapping::map).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MtdLesson;
    use student_management::api::application::MethodGateway;

    use crate::ports::MethodSource;

    #[derive(Clone)]
    struct StubMethodSource {
        lessons: Vec<MtdLesson>,
    }

    impl MethodSource for StubMethodSource {
        fn method_lessons(&self, _id: &str) -> anyhow::Result<Vec<MtdLesson>> {
            Ok(self.lessons.clone())
        }
    }

    #[derive(Clone)]
    struct FailingMethodSource;

    impl MethodSource for FailingMethodSource {
        fn method_lessons(&self, _id: &str) -> anyhow::Result<Vec<MtdLesson>> {
            anyhow::bail!("Method lessons request failed")
        }
    }

    fn mtd_lesson(id: &str, lesson: Option<&str>) -> MtdLesson {
        MtdLesson {
            id: id.to_owned(),
            pages: "00".to_owned(),
            lesson: lesson.map(|s| s.to_owned()),
            method: "MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned(),
            date: "04/12/2023".to_owned(),
            authorizer: "MURILO FAGNER CARDOSO".to_owned(),
            registration_date: "04/12/2023 21:17:17".to_owned(),
            observations: Some("Postura do violino".to_owned()),
        }
    }

    #[test]
    fn given_method_gateway_should_map_dtos_to_generic_lessons() {
        smol::block_on(async {
            let stub = StubMethodSource {
                lessons: vec![mtd_lesson("214020", Some("00"))],
            };
            let gateway = SamMethodGateway::new(stub);

            let lessons = gateway.get_all_for_student_with_id("500132").await.unwrap();

            assert_eq!(lessons.len(), 1);
            assert_eq!(lessons[0].id, "214020");
            assert_eq!(lessons[0].instructor, "MURILO FAGNER CARDOSO");
            assert_eq!(
                lessons[0].method.as_deref().unwrap(),
                "MÉTODO CCB - SCHIMOLL - VIOLINO"
            );
            assert_eq!(lessons[0].phase, None);
        });
    }

    #[test]
    fn given_method_gateway_with_empty_table_should_return_empty() {
        smol::block_on(async {
            let stub = StubMethodSource { lessons: vec![] };
            let gateway = SamMethodGateway::new(stub);
            let lessons = gateway.get_all_for_student_with_id("500132").await.unwrap();
            assert!(lessons.is_empty());
        });
    }

    #[test]
    fn given_method_gateway_with_optional_lesson_none_should_map() {
        smol::block_on(async {
            let stub = StubMethodSource {
                lessons: vec![{
                    let mut l = mtd_lesson("1", None);
                    l.observations = None;
                    l
                }],
            };
            let gateway = SamMethodGateway::new(stub);
            let lessons = gateway.get_all_for_student_with_id("500132").await.unwrap();
            assert_eq!(lessons[0].lesson, None);
            assert_eq!(lessons[0].description, None);
        });
    }

    #[test]
    fn given_method_gateway_when_source_fails_should_propagate_error() {
        smol::block_on(async {
            let gateway = SamMethodGateway::new(FailingMethodSource);
            let result = gateway.get_all_for_student_with_id("500132").await;
            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Method lessons request failed"));
        });
    }
}
