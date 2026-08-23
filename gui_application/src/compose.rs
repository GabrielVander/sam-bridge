use student_management_sam_adapter::SamClient;
use student_management::api::application::{StudentLessonsGateway, StudentsRetrievalGateway};
use student_management_sam_adapter::gateways::{SamLessonsGateway, SamRosterGateway};

/// Erased gateway pair backing one authenticated SAM session.
#[derive(Clone)]
pub struct AppGateways {
    roster: std::sync::Arc<dyn StudentsRetrievalGateway + Send + Sync>,
    lessons: std::sync::Arc<dyn StudentLessonsGateway + Send + Sync>,
}

impl AppGateways {
    pub fn new<R, L>(roster: R, lessons: L) -> Self
    where
        R: StudentsRetrievalGateway + Send + Sync + 'static,
        L: StudentLessonsGateway + Send + Sync + 'static,
    {
        Self {
            roster: std::sync::Arc::new(roster),
            lessons: std::sync::Arc::new(lessons),
        }
    }

    pub fn roster(&self) -> &(dyn StudentsRetrievalGateway + Send + Sync) {
        &*self.roster
    }

    pub fn lessons(&self) -> &(dyn StudentLessonsGateway + Send + Sync) {
        &*self.lessons
    }
}

pub fn gateways_from_session(client: &SamClient) -> AppGateways {
    AppGateways::new(
        SamRosterGateway::new(client.clone()),
        SamLessonsGateway::new(client.clone()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use student_management::api::domain::{
        Lesson, MusicianLevel, Student as DomainStudent, StudentLessons, StudentPosition,
    };

    #[derive(Clone, Default)]
    struct StubRoster;
    #[async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_avaliable_records(&self) -> anyhow::Result<Vec<DomainStudent>> {
            Ok(vec![DomainStudent {
                id: "1".to_owned(),
                name: "ALUNA UM".to_owned(),
                position: StudentPosition::Musician {
                    level: MusicianLevel::Candidate,
                },
                location: String::new(),
                region: student_management::api::domain::Region::AraraquaraSaoCarlos,
            }])
        }
    }

    #[derive(Clone, Default)]
    struct StubLessons;
    #[async_trait]
    impl StudentLessonsGateway for StubLessons {
        async fn get_all_for_student_with_id(
            &self,
            _id: &str,
        ) -> anyhow::Result<StudentLessons> {
            Ok(StudentLessons {
                approved: vec![Lesson::default()],
                method: vec![],
            })
        }
    }

    #[test]
    fn erases_and_exposes_gateways() {
        smol::block_on(async {
            let gateways = AppGateways::new(StubRoster, StubLessons);

            let students = gateways.roster().get_avaliable_records().await.unwrap();
            assert_eq!(students[0].name, "ALUNA UM");

            let lessons = gateways.lessons().get_all_for_student_with_id("7").await.unwrap();
            assert_eq!(lessons.approved.len(), 1);
        });
    }

    #[test]
    fn session_gateways_wrap_the_concrete_client() {
        // Anonymous client suffices: construction never performs I/O.
        let client = SamClient::new("http://127.0.0.1:1").expect("client builds without I/O");

        let _gateways = gateways_from_session(&client);
    }
}
