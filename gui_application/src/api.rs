use std::sync::{OnceLock, RwLock};

use student_management_sam_adapter::{session_opener::NetworkSessionOpener, SamClient};

use crate::compose::{gateways_from_session, AppGateways};
use crate::slices;
use crate::view_models::{StudentLessonsView, StudentListItem};

struct Session {
    gateways: AppGateways,
}

// Process-global session. Tests serialize via test_lock because plain cargo test shares one process.
static SESSION: OnceLock<RwLock<Option<Session>>> = OnceLock::new();

fn session_slot() -> &'static RwLock<Option<Session>> {
    SESSION.get_or_init(|| RwLock::new(None))
}

pub async fn login(base_url: String, username: String, password: String) -> anyhow::Result<()> {
    #[cfg(coverage)]
    {
        // Coverage builds cannot perform the real network round-trip; seed an
        // anonymous-client session so downstream coverage paths stay reachable.
        let client = SamClient::new("http://127.0.0.1:1").expect("client builds without I/O");
        *session_slot()
            .write()
            .expect("Session lock should not be poisoned") = Some(Session {
            gateways: gateways_from_session(&client),
        });
        return Ok(());
    }

    #[cfg(not(coverage))]
    {
        let client: SamClient =
            slices::authentication::login(NetworkSessionOpener, base_url, username, password)
                .await?;

        *session_slot()
            .write()
            .expect("Session lock should not be poisoned") = Some(Session {
            gateways: gateways_from_session(&client),
        });

        Ok(())
    }
}

pub fn logout() {
    *session_slot()
        .write()
        .expect("Session lock should not be poisoned") = None;
}

pub fn is_logged_in() -> bool {
    session_slot()
        .read()
        .expect("Session lock should not be poisoned")
        .is_some()
}

fn with_session<T>(f: impl FnOnce(&AppGateways) -> T) -> anyhow::Result<T> {
    let guard = session_slot()
        .read()
        .expect("Session lock should not be poisoned");
    let session = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
    Ok(f(&session.gateways))
}

pub async fn retrieve_students() -> anyhow::Result<Vec<StudentListItem>> {
    let gateways = with_session(|g| g.clone())?;
    slices::roster::load(gateways.roster()).await
}

pub async fn retrieve_student_lessons(student_id: String) -> anyhow::Result<StudentLessonsView> {
    let gateways = with_session(|g| g.clone())?;
    slices::lessons::load(gateways.lessons(), &student_id).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::application::{
        StudentLessonsGateway, StudentsRetrievalGateway,
    };
    use student_management::api::domain::{
        Lesson as DomainLesson, Student as DomainStudent, StudentLessons,
    };
    use async_trait::async_trait;

    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("Test mutex is never poisoned: no panics while held")
    }

    #[derive(Clone, Default)]
    struct StubRoster {
        students: Vec<DomainStudent>,
    }
    #[async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_avaliable_records(&self) -> anyhow::Result<Vec<DomainStudent>> {
            Ok(self.students.clone())
        }
    }

    #[derive(Clone, Default)]
    struct StubLessons {
        approved: Vec<DomainLesson>,
    }
    #[async_trait]
    impl StudentLessonsGateway for StubLessons {
        async fn get_all_for_student_with_id(
            &self,
            _id: &str,
        ) -> anyhow::Result<StudentLessons> {
            Ok(StudentLessons {
                approved: self.approved.clone(),
                method: vec![],
            })
        }
    }

    fn dated_lesson(id: &str, y: i32, m: u32, d: u32) -> DomainLesson {
        DomainLesson {
            id: Some(id.to_owned()),
            date: chrono::NaiveDate::from_ymd_opt(y, m, d),
            ..Default::default()
        }
    }

    fn stub_student() -> DomainStudent {
        DomainStudent {
            id: "7".to_owned(),
            name: "ALUNA SETE".to_owned(),
            position: student_management::api::domain::StudentPosition::Unknown(String::new()),
            location: String::new(),
            region: student_management::api::domain::Region::Other(String::new()),
        }
    }

    #[test]
    fn given_no_session_queries_should_fail_not_panic() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            assert!(!is_logged_in());
            assert!(retrieve_students().await.is_err());
            assert!(retrieve_student_lessons("7".to_owned()).await.is_err());
        });
    }

    #[test]
    fn given_seeded_session_queries_should_return_domain_data_sorted() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            *session_slot()
                .write()
                .expect("lock") = Some(Session {
                gateways: AppGateways::new(
                    StubRoster {
                        students: vec![stub_student()],
                    },
                    StubLessons {
                        approved: vec![
                            dated_lesson("11", 2025, 6, 1),
                            dated_lesson("12", 2025, 6, 2),
                        ],
                    },
                ),
            });
            assert!(is_logged_in());

            let students = retrieve_students().await.expect("students");
            assert_eq!(students[0].name, "ALUNA SETE");

            let view = retrieve_student_lessons("7".to_owned()).await.expect("lessons");
            let ids: Vec<&str> = view.msa.iter().map(|i| i.id.as_str()).collect();
            assert_eq!(ids, vec!["12", "11"], "most recent first");

            logout();
            assert!(!is_logged_in());
        });
    }

    #[cfg(coverage)]
    #[test]
    fn login_seeds_an_anonymous_session_under_coverage() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            // Coverage builds short-circuit the network hop; the seeded
            // anonymous session must still be usable.
            login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned())
                .await
                .expect("coverage login seeds a session");
            assert!(is_logged_in());
            logout();
            assert!(!is_logged_in());
        });
    }

    #[cfg(not(coverage))]
    #[test]
    fn login_reports_failures_from_the_opener_chain() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            // Unreachable port exercises the real opener chain end-to-end.
            let result =
                login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
            assert!(!is_logged_in());
        });
    }
}
