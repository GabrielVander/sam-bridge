use async_trait::async_trait;
use sam::client::{Authenticated, SamClient};
use student_management::api::{application::MsaGateway, domain::Lesson};

use crate::{mapping::msa as msa_mapping, ports::MsaSource};

pub struct SamMsaGateway<S: MsaSource = SamClient<Authenticated>> {
    source: S,
}

impl<S: MsaSource> SamMsaGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> MsaGateway for SamMsaGateway<S>
where
    S: MsaSource + Clone + Send + Sync + 'static,
{
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        let source = self.source.clone();
        let id = id.to_owned();
        // sam is blocking — offload to thread pool so we don't block the async executor
        let dtos = smol::unblock(move || source.msa_lessons(&id)).await?;
        dtos.iter().map(msa_mapping::map).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::MsaLesson;
    use student_management::api::application::MsaGateway;

    use crate::ports::MsaSource;

    #[derive(Clone)]
    struct StubMsaSource {
        lessons: Vec<MsaLesson>,
    }

    impl MsaSource for StubMsaSource {
        fn msa_lessons(&self, _id: &str) -> anyhow::Result<Vec<MsaLesson>> {
            Ok(self.lessons.clone())
        }
    }

    #[derive(Clone)]
    struct FailingMsaSource;

    impl MsaSource for FailingMsaSource {
        fn msa_lessons(&self, _id: &str) -> anyhow::Result<Vec<MsaLesson>> {
            anyhow::bail!("MSA lessons request failed")
        }
    }

    fn msa_lesson(id: &str) -> MsaLesson {
        MsaLesson {
            id: id.to_owned(),
            date: "09/09/2025".to_owned(),
            phases: "4.5 - 4.5".to_owned(),
            pages: "38 - 38".to_owned(),
            lessons: Some("7 - 8".to_owned()),
            clefs: Some("Sol".to_owned()),
            description: Some("Passou lições 7 e 8".to_owned()),
            authorizer: "MARCOS ROGÉRIO COSME".to_owned(),
        }
    }

    #[test]
    fn given_msa_gateway_should_map_dtos_to_generic_lessons() {
        smol::block_on(async {
            let stub = StubMsaSource {
                lessons: vec![msa_lesson("559783")],
            };
            let gateway = SamMsaGateway::new(stub);

            let lessons = gateway.get_all_for_student_with_id("500132").await.unwrap();

            assert_eq!(lessons.len(), 1);
            assert_eq!(lessons[0].id, "559783");
            assert_eq!(lessons[0].instructor, "MARCOS ROGÉRIO COSME");
            assert_eq!(lessons[0].method, None);
        });
    }

    #[test]
    fn given_msa_gateway_with_empty_table_should_return_empty() {
        smol::block_on(async {
            let stub = StubMsaSource { lessons: vec![] };
            let gateway = SamMsaGateway::new(stub);
            let lessons = gateway.get_all_for_student_with_id("500132").await.unwrap();
            assert!(lessons.is_empty());
        });
    }

    #[test]
    fn given_msa_gateway_when_source_fails_should_propagate_error() {
        smol::block_on(async {
            let gateway = SamMsaGateway::new(FailingMsaSource);
            let result = gateway.get_all_for_student_with_id("500132").await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("MSA lessons request failed"));
        });
    }
}
