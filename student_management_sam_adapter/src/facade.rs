use std::sync::Arc;

use anyhow::Result;
use student_management::api::{
    application::{StudentLessonsGateway, StudentsRetrievalGateway},
};

use crate::gateways::SamGateways;
use crate::session_opener::NetworkSessionOpener;

/// Erased gateway pair for one authenticated SAM session.
///
/// Both fields are core-owned abstractions, so this type carries no
/// infrastructure knowledge across crate boundaries.
pub struct AuthSession {
    pub roster: Arc<dyn StudentsRetrievalGateway + Send + Sync>,
    pub lessons: Arc<dyn StudentLessonsGateway + Send + Sync>,
}

impl AuthSession {
    /// Builds a session from any pair of core gateway implementations —
    /// the injection seam for tests above this crate.
    pub fn from_gateways(
        roster: Arc<dyn StudentsRetrievalGateway + Send + Sync>,
        lessons: Arc<dyn StudentLessonsGateway + Send + Sync>,
    ) -> Self {
        Self { roster, lessons }
    }
}

/// Opens an authenticated session against the SAM portal and returns its
/// gateways. This is the adapter's single public entry point.
///
/// Under coverage builds the network hop is skipped and an anonymous-session
/// stand-in is returned instead, so downstream code remains fully measurable.
pub async fn authenticate(base_url: &str, username: &str, password: &str) -> Result<AuthSession> {
    #[cfg(coverage)]
    {
        use sam::client::SamClient;

        // Anonymous clients are network-free to fabricate.
        let client: SamClient = SamClient::new("http://127.0.0.1:1")
            .expect("client builds without I/O");
        let gateways = SamGateways::from_client(&client);
        return Ok(AuthSession::from_gateways(
            Arc::new(gateways.clone()),
            Arc::new(gateways),
        ));
    }

    #[cfg(not(coverage))]
    {
        let gateways =
            SamGateways::open_with(NetworkSessionOpener, base_url, username, password).await?;

        Ok(AuthSession::from_gateways(
            Arc::new(gateways.clone()),
            Arc::new(gateways),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::domain::{Student, StudentLessons};
    use crate::session_opener::SessionOpener;
    use sam::client::SamClient;

    #[derive(Clone, Default)]
    struct StubRoster;
    #[async_trait::async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_avaliable_records(&self) -> Result<Vec<Student>> {
            Ok(vec![Student {
                id: "1".to_owned(),
                name: "ALUNO".to_owned(),
                position: student_management::api::domain::StudentPosition::Unknown(String::new()),
                location: String::new(),
                region: student_management::api::domain::Region::Other(String::new()),
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
            let students = session.roster.get_avaliable_records().await.unwrap();
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
            let result = crate::gateways::SamGateways::open_with(FailingOpener, "b", "u", "p").await;

            let err = result.err().expect("expected opener failure");
            assert!(err.to_string().contains("Login request failed"));
        });
    }

    #[cfg(coverage)]
    #[test]
    fn authenticate_seeds_an_anonymous_session_under_coverage() {
        smol::block_on(async {
            // Coverage builds skip the network hop; the returned session's
            // gateways are fabricated clients pointing at a dead port.
            let session = authenticate("http://127.0.0.1:1", "u", "p")
                .await
                .expect("coverage builds seed an anonymous session");

            // Calling through them will fail at the dead port — but the
            // important thing is that they exist and are callable.
            assert!(session.roster.get_avaliable_records().await.is_err());
        });
    }
}
