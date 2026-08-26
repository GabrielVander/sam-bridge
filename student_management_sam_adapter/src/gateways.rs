use std::sync::Arc;

use anyhow::Result;
use sam::client::{SamClient, SamReader};
use student_management::{
    application::gateways::{StudentLessonsGateway, StudentsRetrievalGateway},
    domain::entities::{Student, StudentLessons},
};

use crate::session_opener::{SessionOpener, open_session};

#[derive(Clone)]
pub struct SamGateways {
    reader: Arc<dyn SamReader + Send + Sync>,
}

impl SamGateways {
    pub fn from_client(client: &SamClient) -> Self {
        Self {
            reader: Arc::new(client.clone()),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_reader(reader: Arc<dyn SamReader + Send + Sync>) -> Self {
        Self { reader }
    }

    pub(crate) async fn open_with<O>(
        opener: O,
        base_url: &str,
        username: &str,
        password: &str,
    ) -> Result<Self>
    where
        O: SessionOpener + Send + Sync + 'static,
    {
        let client = open_session(
            opener,
            base_url.to_owned(),
            username.to_owned(),
            password.to_owned(),
        )
        .await?;
        Ok(Self::from_client(&client))
    }
}

#[async_trait::async_trait]
impl StudentsRetrievalGateway for SamGateways {
    async fn get_available_records(&self) -> Result<Vec<Student>> {
        let reader = self.reader.clone();
        let dtos = smol::unblock(move || reader.students()).await?;
        dtos.iter().map(crate::mapping::roster::map).collect()
    }
}

#[async_trait::async_trait]
impl StudentLessonsGateway for SamGateways {
    async fn get_all_for_student_with_id(&self, id: &str) -> Result<StudentLessons> {
        let reader = self.reader.clone();
        let id = id.to_owned();
        let page = smol::unblock(move || reader.student_lessons(&id)).await?;

        Ok(StudentLessons {
            approved: page.msa.iter().map(crate::mapping::msa::map).collect(),
            method: page
                .method
                .iter()
                .map(crate::mapping::method::map)
                .collect(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::{MsaLesson, MtdLesson, SamStudent, StudentLessonsPage};

    #[derive(Clone)]
    struct StubReader {
        students: Vec<SamStudent>,
        page: StudentLessonsPage,
        fail_students: bool,
        fail_lessons: bool,
    }

    impl SamReader for StubReader {
        fn students(&self) -> Result<Vec<SamStudent>> {
            if self.fail_students {
                anyhow::bail!("Students HTTP request failed");
            }
            Ok(self.students.clone())
        }

        fn student_lessons(&self, _id: &str) -> Result<StudentLessonsPage> {
            if self.fail_lessons {
                anyhow::bail!("Student lessons request failed");
            }
            Ok(self.page.clone())
        }
    }

    #[derive(Clone, Default)]
    struct DeadPortOpener;

    #[derive(Clone, Default)]
    struct FailingOpener;

    impl SessionOpener for FailingOpener {
        fn open(&self, _base: &str, _u: &str, _p: &str) -> Result<SamClient> {
            anyhow::bail!("Login request failed")
        }
    }

    impl SessionOpener for DeadPortOpener {
        fn open(&self, base_url: &str, _u: &str, _p: &str) -> Result<SamClient> {
            SamClient::new(base_url)
        }
    }

    fn student_dto(id: &str) -> SamStudent {
        SamStudent {
            id: id.to_owned(),
            name: format!("ALUNO {id}"),
            location: "BAIRRO | BR-SP-ARARAQUARA-SÃO CARLOS".to_owned(),
            role: "MÚSICO".to_owned(),
            instrument: "VIOLINO".to_owned(),
            level: "CANDIDATO(A)".to_owned(),
        }
    }

    fn page() -> StudentLessonsPage {
        StudentLessonsPage {
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
        }
    }

    fn gateway(students: Vec<SamStudent>, fail_students: bool, fail_lessons: bool) -> SamGateways {
        SamGateways::from_reader(Arc::new(StubReader {
            students,
            page: page(),
            fail_students,
            fail_lessons,
        }))
    }

    #[test]
    fn implements_both_core_ports_over_one_reader() {
        smol::block_on(async {
            let gateways = gateway(vec![student_dto("1")], false, false);

            let students = gateways.get_available_records().await.expect("roster");
            assert_eq!(students.len(), 1);
            assert_eq!(students[0].name, "ALUNO 1");

            let lessons = gateways
                .get_all_for_student_with_id("500132")
                .await
                .expect("lessons");
            assert_eq!(lessons.approved[0].id.as_deref(), Some("559783"));
            assert_eq!(lessons.method[0].id.as_deref(), Some("214020"));
        });
    }

    #[test]
    fn empty_reader_yields_empty_results() {
        smol::block_on(async {
            let gateways = SamGateways::from_reader(Arc::new(StubReader {
                students: vec![],
                page: StudentLessonsPage::default(),
                fail_students: false,
                fail_lessons: false,
            }));

            assert!(gateways.get_available_records().await.unwrap().is_empty());

            let lessons = gateways.get_all_for_student_with_id("1").await.unwrap();
            assert!(lessons.approved.is_empty());
            assert!(lessons.method.is_empty());
        });
    }

    #[test]
    fn roster_reader_errors_propagate() {
        smol::block_on(async {
            let gateways = gateway(vec![], true, false);

            let result = gateways.get_available_records().await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Students HTTP"));
        });
    }

    #[test]
    fn lessons_reader_errors_propagate() {
        smol::block_on(async {
            let gateways = gateway(vec![], false, true);

            let result = gateways.get_all_for_student_with_id("1").await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Student lessons"));
        });
    }

    #[test]
    fn tolerant_mapping_keeps_rows_with_absent_fields() {
        smol::block_on(async {
            let mut nameless = student_dto("3");
            nameless.name = " ".to_owned();

            let gateways = gateway(vec![nameless], false, false);

            let students = gateways.get_available_records().await.expect("tolerance");
            assert_eq!(students[0].name, "");
        });
    }

    #[test]
    fn opener_errors_propagate_through_open_with() {
        smol::block_on(async {
            let result =
                SamGateways::open_with(FailingOpener, "http://127.0.0.1:1", "u", "p").await;

            let err = result.err().expect("expected opener failure");
            assert!(err.to_string().contains("Login request failed"));
        });
    }

    #[test]
    fn opens_with_any_session_opener_without_touching_the_network_here() {
        smol::block_on(async {
            let gateways = SamGateways::open_with(DeadPortOpener, "http://127.0.0.1:1", "u", "p")
                .await
                .expect("delegation should succeed");

            assert!(gateways.get_available_records().await.is_err());
        });
    }
}
