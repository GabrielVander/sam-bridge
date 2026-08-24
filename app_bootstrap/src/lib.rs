use std::sync::RwLock;

use student_management::api::domain::{Student, StudentLessons};
pub use student_management_sam_adapter::{authenticate, AuthSession};

pub struct App {
    session: RwLock<Option<AuthSession>>,
}

impl App {
    pub fn new() -> Self {
        Self {
            session: RwLock::new(None),
        }
    }

    pub async fn login(
        &self,
        base_url: String,
        username: String,
        password: String,
    ) -> anyhow::Result<()> {
    let session = authenticate(&base_url, &username, &password).await?;
    *self
        .session
        .write()
        .expect("Session lock should not be poisoned") = Some(session);
    Ok(())
}

    pub fn logout(&self) {
        *self
            .session
            .write()
            .expect("Session lock should not be poisoned") = None;
    }

    pub fn is_logged_in(&self) -> bool {
        self.session
            .read()
            .expect("Session lock should not be poisoned")
            .is_some()
    }

    pub async fn retrieve_students(&self) -> anyhow::Result<Vec<Student>> {
        let roster = self.with_session(|s| s.roster.clone())?;
        roster.get_avaliable_records().await
    }

    pub async fn retrieve_student_lessons(&self, id: &str) -> anyhow::Result<StudentLessons> {
        let lessons = self.with_session(|s| s.lessons.clone())?;
        lessons.get_all_for_student_with_id(id).await
    }

    /// Test seam: installs a pre-built session without going through login.
    #[doc(hidden)]
    pub fn seed_session(&self, session: AuthSession) {
        *self
            .session
            .write()
            .expect("Session lock should not be poisoned") = Some(session);
    }

    fn with_session<T>(&self, f: impl FnOnce(&AuthSession) -> T) -> anyhow::Result<T> {
        let guard = self
            .session
            .read()
            .expect("Session lock should not be poisoned");
        let session = guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        Ok(f(session))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex, OnceLock};
    use student_management::api::{
        application::{StudentLessonsGateway, StudentsRetrievalGateway},
        domain::{
            Lesson, MusicianLevel, Region, Student as DomainStudent, StudentLessons,
            StudentPosition,
        },
    };

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        TEST_LOCK
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("Test mutex is never poisoned: no panics while held")
    }



    #[derive(Clone, Default)]
    struct StubRoster {
        students: Vec<DomainStudent>,
        fail: bool,
    }

    #[async_trait::async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_avaliable_records(&self) -> anyhow::Result<Vec<DomainStudent>> {
            if self.fail {
                anyhow::bail!("Students HTTP request failed");
            }
            Ok(self.students.clone())
        }
    }

    #[derive(Clone, Default)]
    struct StubLessons {
        approved: Vec<Lesson>,
        fail: bool,
    }

    #[async_trait::async_trait]
    impl StudentLessonsGateway for StubLessons {
        async fn get_all_for_student_with_id(&self, _id: &str) -> anyhow::Result<StudentLessons> {
            if self.fail {
                anyhow::bail!("Student lessons request failed");
            }
            Ok(StudentLessons {
                approved: self.approved.clone(),
                method: vec![],
            })
        }
    }

    fn student(id: &str) -> DomainStudent {
        DomainStudent {
            id: id.to_owned(),
            name: format!("ALUNO {id}"),
            position: StudentPosition::Musician {
                level: MusicianLevel::Candidate,
            },
            location: String::new(),
            region: Region::Other(String::new()),
        }
    }

    fn lesson_on(id: &str, y: i32, m: u32, d: u32) -> Lesson {
        Lesson {
            id: Some(id.to_owned()),
            date: chrono::NaiveDate::from_ymd_opt(y, m, d),
            ..Default::default()
        }
    }

    #[test]
    fn new_app_is_logged_out() {
        let _guard = test_lock();
        let app = App::new();

        assert!(!app.is_logged_in());
        assert!(smol::block_on(app.retrieve_students()).is_err());
        assert!(smol::block_on(app.retrieve_student_lessons("1")).is_err());
    }

    #[test]
    fn seeded_session_exposes_students_and_sorted_lessons() {
        let _guard = test_lock();
        let app = App::new();

        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster {
                students: vec![student("1"), student("2")],
                fail: false,
            }),
            Arc::new(StubLessons {
                approved: vec![
                    lesson_on("old", 2023, 12, 4),
                    lesson_on("newest", 2026, 3, 24),
                ],
                fail: false,
            }),
        ));

        assert!(app.is_logged_in());

        smol::block_on(async {
            let students = app.retrieve_students().await.expect("students");
            assert_eq!(students.len(), 2);

            let lessons = app.retrieve_student_lessons("7").await.expect("lessons");
            let ids: Vec<&str> = lessons.approved.iter().map(|l| l.id.as_deref().unwrap()).collect();
            // Use cases return raw data; sorting is the gui's concern.
            assert!(ids.contains(&"old") && ids.contains(&"newest"));
        });

        app.logout();
        assert!(!app.is_logged_in());
    }

    #[cfg(not(coverage))]
    #[test]
    fn login_to_dead_port_should_fail_and_stay_logged_out() {
        let _guard = test_lock();
        let app = App::new();

        smol::block_on(async {
            let result = app
                .login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned())
                .await;

            assert!(result.is_err());
            assert!(!app.is_logged_in());
        });
    }

    #[cfg(coverage)]
    #[test]
    fn login_under_coverage_should_seed_anonymous_session() {
        let _guard = test_lock();
        let app = App::new();

        smol::block_on(async {
            // Coverage builds skip the network hop inside authenticate().
            app.login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned())
                .await
                .expect("coverage login seeds an anonymous session");
            assert!(app.is_logged_in());
            app.logout();
            assert!(!app.is_logged_in());
        });
    }

    #[test]
    fn gateway_errors_propagate() {
        let _guard = test_lock();
        let app = App::new();
        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster {
                students: vec![],
                fail: true,
            }),
            Arc::new(StubLessons {
                approved: vec![],
                fail: true,
            }),
        ));

        smol::block_on(async {
            assert!(app.retrieve_students().await.is_err());
            assert!(app.retrieve_student_lessons("7").await.is_err());
        });
    }
}
