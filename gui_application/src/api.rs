use std::sync::OnceLock;

use app_bootstrap::App;
use student_management::api::domain::{Student, StudentLessons};

use crate::slices::{lessons as lessons_mapper, roster as roster_mapper};
use crate::view_models::{ProgressViewModel, StudentLessonsView, StudentListItem};

static APP: OnceLock<App> = OnceLock::new();

fn app() -> &'static App {
    APP.get_or_init(App::new)
}

/// Whether persisted credentials exist on disk (cheap synchronous check).
pub fn has_saved_credentials() -> bool {
    app().has_saved_credentials()
}

/// Attempts silent re-authentication using persisted credentials.
/// Returns true when a usable session was established.
pub async fn try_restore_session() -> bool {
    app().try_restore_session().await
}

pub async fn login(base_url: String, username: String, password: String) -> anyhow::Result<()> {
    app().login(base_url, username, password).await
}

pub fn logout() {
    app().logout();
}

pub fn is_logged_in() -> bool {
    app().is_logged_in()
}

pub async fn retrieve_students() -> anyhow::Result<Vec<StudentListItem>> {
    let students: Vec<Student> = app().retrieve_students().await?;
    Ok(roster_mapper::to_list_items(&students))
}

pub async fn retrieve_student_progress(
    student_id: String,
    assigned_level: String,
) -> anyhow::Result<ProgressViewModel> {
    let level = student_management::api::domain::MusicianLevel::parse_named(&assigned_level);
    let report = app().calculate_progress(&student_id, &level).await?;
    Ok(ProgressViewModel::from(&report))
}

pub async fn retrieve_student_lessons(student_id: String) -> anyhow::Result<StudentLessonsView> {
    let bundle: StudentLessons =
        app().retrieve_student_lessons(&student_id).await?;
    Ok(lessons_mapper::to_view(&bundle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use app_bootstrap::AuthSession;
    use student_management::api::{
        application::{StudentLessonsGateway, StudentsRetrievalGateway},
        domain::{Lesson as DomainLesson, Region, StudentLessons},
    };

    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("Test mutex is never poisoned")
    }

    #[derive(Clone, Default)]
    struct StubRoster {
        students: Vec<Student>,
    }
    #[async_trait::async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_avaliable_records(&self) -> anyhow::Result<Vec<Student>> {
            Ok(self.students.clone())
        }
    }

    #[derive(Clone, Default)]
    struct StubLessons {
        approved: Vec<DomainLesson>,
    }
    #[async_trait::async_trait]
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

    #[test]
    fn given_no_session_queries_should_fail_not_panic() {
        let _guard = test_lock();
        logout();

        assert!(!is_logged_in());
        assert!(smol::block_on(retrieve_students()).is_err());
        assert!(smol::block_on(retrieve_student_lessons("7".to_owned())).is_err());
    }

    #[test]
    fn given_seeded_session_queries_should_return_domain_data_sorted() {
        let _guard = test_lock();
        let app = app();

        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster {
                students: vec![Student {
                    id: "7".to_owned(),
                    name: "ALUNA SETE".to_owned(),
                    position: student_management::api::domain::StudentPosition::Unknown(
                String::new(),
            ),
                    location: String::new(),
                    region: Region::Other(String::new()),
                }],
            }),
            Arc::new(StubLessons {
                approved: vec![
                    dated_lesson("11", 2025, 6, 1),
                    dated_lesson("12", 2025, 6, 2),
                ],
            }),
        ));

        assert!(is_logged_in());

        smol::block_on(async {
            let students = retrieve_students().await.expect("students");
            assert_eq!(students[0].name, "ALUNA SETE");

            let view = retrieve_student_lessons("7".to_owned()).await.expect("lessons");
            let ids: Vec<&str> = view.msa.iter().map(|i| i.id.as_str()).collect();
            assert_eq!(ids, vec!["12", "11"], "most recent first");
        });

        logout();
        assert!(!is_logged_in());
    }

    #[cfg(coverage)]
    #[test]
    fn login_under_coverage_should_seed_session() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            // Coverage builds skip the network hop inside authenticate().
            login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned())
                .await
                .expect("coverage login seeds a session");
            assert!(is_logged_in());
        });
    }

    #[cfg(not(coverage))]
    #[cfg(coverage)]
    #[test]
    fn try_restore_session_without_credentials_returns_false() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            let restored = try_restore_session().await;
            assert!(!restored);
            assert!(!is_logged_in());
        });
    }

    #[test]
    fn has_saved_credentials_delegates_to_app() {
        let _guard = test_lock();
        logout();
        // Just exercises the delegation; the underlying store is file-backed.
        let _ = has_saved_credentials();
    }

    #[cfg(not(coverage))]
    #[test]
    fn login_reports_failures_from_the_opener_chain() {
        let _guard = test_lock();
        logout();

        smol::block_on(async {
            let result =
                login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
            assert!(!is_logged_in());
        });
    }
}
