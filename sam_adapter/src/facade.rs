use std::sync::Arc;

use anyhow::Result;
use student_core::application::gateways::{StudentGateway, StudentLessonsGateway};

use crate::gateways::SamGateways;
use crate::session_opener::NetworkSessionOpener;

pub struct AuthSession {
    pub roster: Arc<dyn StudentGateway + Send + Sync>,
    pub lessons: Arc<dyn StudentLessonsGateway + Send + Sync>,
}

impl AuthSession {
    pub fn from_gateways(
        roster: Arc<dyn StudentGateway + Send + Sync>,
        lessons: Arc<dyn StudentLessonsGateway + Send + Sync>,
    ) -> Self {
        Self { roster, lessons }
    }
}

pub async fn authenticate(base_url: &str, username: &str, password: &str) -> Result<AuthSession> {
    let gateways =
        SamGateways::open_with(NetworkSessionOpener, base_url, username, password).await?;

    Ok(AuthSession::from_gateways(
        Arc::new(gateways.clone()),
        Arc::new(gateways),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::session_opener::SessionOpener;
    use sam::client::SamClient;
    use student_core::domain::entities::{Region, Student, StudentLessons, StudentPosition};

    #[derive(Clone, Default)]
    struct StubRoster;
    #[async_trait::async_trait]
    impl StudentGateway for StubRoster {
        async fn get_available_records(&self) -> Result<Vec<Student>> {
            Ok(vec![Student {
                id: "1".to_owned(),
                name: "ALUNO".to_owned(),
                position: StudentPosition::Unknown(String::new()),
                location: String::new(),
                region: Region::Other(String::new()),
            }])
        }
    }

    #[derive(Clone, Default)]
    struct StubLessons;
    #[async_trait::async_trait]
    impl StudentLessonsGateway for StubLessons {
        async fn get_all_for_student_with_id(&self, _id: &str) -> Result<StudentLessons> {
            Ok(StudentLessons::default())
        }
    }

    #[derive(Clone, Default)]
    struct FailingOpener;

    impl SessionOpener for FailingOpener {
        fn open(&self, _base: &str, _u: &str, _p: &str) -> Result<SamClient> {
            anyhow::bail!("Login request failed")
        }
    }

    #[test]
    fn from_gateways_exposes_erased_core_trait_objects() {
        let session = AuthSession::from_gateways(Arc::new(StubRoster), Arc::new(StubLessons));

        smol::block_on(async {
            let students = session.roster.get_available_records().await.unwrap();
            assert_eq!(students.len(), 1);

            let lessons = session
                .lessons
                .get_all_for_student_with_id("7")
                .await
                .unwrap();
            assert!(lessons.approved.is_empty());
        });
    }

    #[test]
    fn authenticate_delegates_to_the_session_opener() {
        smol::block_on(async {
            let result =
                crate::gateways::SamGateways::open_with(FailingOpener, "b", "u", "p").await;

            let err = result.err().expect("expected opener failure");
            assert!(err.to_string().contains("Login request failed"));
        });
    }
}
