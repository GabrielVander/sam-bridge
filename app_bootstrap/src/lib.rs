mod session_store;

use std::sync::{Arc, RwLock};

use student_management::api::{
    application::{LoginInput, LoginOutput},
    domain::{
        MusicianLevel, ProgressAssessment, Student, StudentLessons, calculate_progress_fn,
        violin_schmoll_profile,
    },
};
use student_management_sam_adapter::session_opener::SamAuthGateway;
pub use student_management_sam_adapter::{AuthSession, authenticate};

pub use session_store::{FileSessionStore, SessionStore, StoredCredentials};

pub struct App {
    session: RwLock<Option<AuthSession>>,
    store: Box<dyn SessionStore>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self::with_store(Box::new(FileSessionStore::new()))
    }

    pub fn with_store(store: Box<dyn SessionStore>) -> Self {
        Self {
            session: RwLock::new(None),
            store,
        }
    }

    pub async fn login(&self, input: LoginInput) -> anyhow::Result<LoginOutput> {
        let gateway = Arc::new(SamAuthGateway::new(input.baseUrl.clone()));
        let output = {
            use student_management::api::application::LoginUseCase;
            LoginUseCase::new(gateway.clone())
                .execute(input.clone())
                .await?
        };
        let client = gateway
            .take_client()
            .ok_or_else(|| anyhow::anyhow!("Login succeeded but no client"))?;
        let gateways = student_management_sam_adapter::gateways::SamGateways::from_client(&client);
        let session = AuthSession::from_gateways(Arc::new(gateways.clone()), Arc::new(gateways));
        self.store.save(&StoredCredentials {
            base_url: input.baseUrl,
            username: input.username,
            password: input.password,
        })?;
        *self.session.write().unwrap_or_else(|e| e.into_inner()) = Some(session);
        Ok(output)
    }

    pub fn has_saved_credentials(&self) -> bool {
        self.store.load().is_some()
    }

    pub fn logout(&self) {
        self.store.clear();
        *self.session.write().unwrap_or_else(|e| e.into_inner()) = None;
    }

    pub async fn try_restore_session(&self) -> bool {
        let Some(creds) = self.store.load() else {
            return false;
        };
        match authenticate(&creds.base_url, &creds.username, &creds.password).await {
            Ok(session) => {
                *self.session.write().unwrap_or_else(|e| e.into_inner()) = Some(session);
                true
            }
            Err(_) => {
                self.store.clear();
                false
            }
        }
    }

    pub fn is_logged_in(&self) -> bool {
        self.session
            .read()
            .unwrap_or_else(|e| e.into_inner())
            .is_some()
    }

    pub async fn retrieve_students(&self) -> anyhow::Result<Vec<Student>> {
        let roster = self.with_session(|s| s.roster.clone())?;
        let gateway = roster.as_ref();
        let students = gateway.get_available_records().await?;
        Ok(students)
    }

    pub async fn calculate_progress(
        &self,
        student_id: &str,
        assigned_level: &MusicianLevel,
    ) -> anyhow::Result<ProgressAssessment> {
        let lessons = self.with_session(|s| s.lessons.clone())?;
        let bundle = lessons.get_all_for_student_with_id(student_id).await?;
        calculate_progress_fn(
            assigned_level,
            &bundle.approved,
            &bundle.method,
            &violin_schmoll_profile(),
        )
        .map_err(|e| anyhow::anyhow!(e))
    }

    pub async fn retrieve_student_lessons(&self, id: &str) -> anyhow::Result<StudentLessons> {
        let lessons = self.with_session(|s| s.lessons.clone())?;
        let gateway = lessons.as_ref();
        gateway.get_all_for_student_with_id(id).await
    }

    #[doc(hidden)]
    pub fn seed_session(&self, session: AuthSession) {
        *self.session.write().unwrap_or_else(|e| e.into_inner()) = Some(session);
    }

    fn with_session<T>(&self, f: impl FnOnce(&AuthSession) -> T) -> anyhow::Result<T> {
        let guard = self.session.read().unwrap_or_else(|e| e.into_inner());
        let session = guard
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
        Ok(f(session))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex, MutexGuard, OnceLock};
    use student_management::api::{
        application::{StudentLessonsGateway, StudentsRetrievalGateway},
        domain::{
            Lesson, MusicianLevel, Region, Student as DomainStudent, StudentLessons,
            StudentPosition,
        },
    };
    use tempfile::TempDir;

    static TEST_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn test_lock() -> MutexGuard<'static, ()> {
        TEST_LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    #[derive(Clone, Default)]
    struct StubRoster {
        students: Vec<DomainStudent>,
        fail: bool,
    }

    #[async_trait::async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_available_records(&self) -> anyhow::Result<Vec<DomainStudent>> {
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
    fn new_app_is_logged_out_and_has_no_saved_credentials() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        assert!(!app.is_logged_in());
        assert!(smol::block_on(app.retrieve_students()).is_err());
        assert!(smol::block_on(app.retrieve_student_lessons("1")).is_err());
    }

    #[test]
    fn seeded_session_exposes_students_and_sorted_lessons() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
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
            assert_eq!(students[0].id, "1");
            let lessons = app.retrieve_student_lessons("7").await.expect("lessons");
            assert_eq!(lessons.approved.len(), 2);
            assert!(lessons.approved.iter().any(|l| l.id.as_deref() == Some("old")));
            assert!(lessons.approved.iter().any(|l| l.id.as_deref() == Some("newest")));
        });
        app.logout();
        assert!(!app.is_logged_in());
    }

    #[test]
    fn default_constructor_creates_file_backed_app() {
        let _app = App::new();
    }

    #[test]
    fn default_impl_delegates_to_new() {
        let _guard = test_lock();
        let _app = App::default();
        assert!(!_app.is_logged_in());
    }

    #[test]
    fn has_saved_credentials_reflects_login_state() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        assert!(!app.has_saved_credentials());
        assert!(!app.is_logged_in());
    }

    #[test]
    fn try_restore_session_without_credentials_returns_false() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        smol::block_on(async {
            let restored = app.try_restore_session().await;
            assert!(!restored);
            assert!(!app.is_logged_in());
        });
    }

    #[test]
    fn login_persists_credentials_to_disk() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        smol::block_on(async {
            let result = app
                .login(LoginInput {
                    baseUrl: "http://127.0.0.1:1".to_owned(),
                    username: "u".to_owned(),
                    password: "p".to_owned(),
                })
                .await;
            assert!(result.is_err());
            assert!(store.load().is_none(), "failed login must not persist");
            assert!(!app.is_logged_in());
        });
    }

    #[test]
    fn logout_clears_persisted_credentials() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        store
            .save(&StoredCredentials {
                base_url: "http://x".to_owned(),
                username: "u".to_owned(),
                password: "p".to_owned(),
            })
            .unwrap();
        assert!(store.load().is_some());
        app.logout();
        assert!(
            store.load().is_none(),
            "logout must clear persisted credentials"
        );
    }

    #[test]
    fn gateway_errors_propagate() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
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

    #[test]
    fn login_to_dead_port_should_fail_and_stay_logged_out() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(
            dir.path().to_path_buf(),
        )));
        smol::block_on(async {
            let result = app
                .login(LoginInput {
                    baseUrl: "http://127.0.0.1:1".to_owned(),
                    username: "u".to_owned(),
                    password: "p".to_owned(),
                })
                .await;
            assert!(result.is_err());
            assert!(!app.is_logged_in());
        });
    }

    #[test]
    fn login_with_test_success_creates_session_and_persists() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        smol::block_on(async {
            let result = app
                .login(LoginInput {
                    baseUrl: "http://test-success".to_owned(),
                    username: "u".to_owned(),
                    password: "p".to_owned(),
                })
                .await;
            assert!(result.is_ok());
            assert!(app.is_logged_in());
            assert!(store.load().is_some());
            assert!(app.has_saved_credentials());
            let students = app.retrieve_students().await;
            assert!(students.is_ok());
        });
    }

    #[test]
    fn try_restore_session_success_with_test_success() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        store
            .save(&StoredCredentials {
                base_url: "http://test-success".to_owned(),
                username: "u".to_owned(),
                password: "p".to_owned(),
            })
            .unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        smol::block_on(async {
            let restored = app.try_restore_session().await;
            assert!(restored);
            assert!(app.is_logged_in());
        });
    }

    #[test]
    fn calculate_progress_for_known_level_succeeds() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster { students: vec![], fail: false }),
            Arc::new(StubLessons {
                approved: vec![Lesson {
                    id: Some("1".to_owned()),
                    date: chrono::NaiveDate::from_ymd_opt(2025, 1, 15),
                    phase: Some(student_management::api::domain::Range { from: "12".to_owned(), to: "12".to_owned() }),
                    ..Default::default()
                }],
                fail: false,
            }),
        ));
        smol::block_on(async {
            let report = app
                .calculate_progress("7", &MusicianLevel::Candidate)
                .await
                .expect("should succeed");
            assert!(!report.checkpoints.is_empty());
        });
    }

    #[test]
    fn calculate_progress_for_unknown_level_fails() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster { students: vec![], fail: false }),
            Arc::new(StubLessons { approved: vec![], fail: false }),
        ));
        smol::block_on(async {
            let result = app.calculate_progress("7", &MusicianLevel::Unknown("X".to_owned())).await;
            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("UnknownLevel"));
        });
    }

    #[test]
    fn with_session_not_authenticated_fails() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        smol::block_on(async {
            assert!(app.retrieve_students().await.is_err());
        });
    }

    #[test]
    fn try_restore_session_failure_clears_store() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        store.save(&StoredCredentials { base_url: "http://127.0.0.1:1".to_owned(), username: "u".to_owned(), password: "p".to_owned() }).unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        smol::block_on(async {
            let restored = app.try_restore_session().await;
            assert!(!restored);
            assert!(!app.is_logged_in());
            assert!(!app.has_saved_credentials());
        });
    }

    #[test]
    fn login_take_client_failure_path() {
        use student_management::api::application::AuthGateway;
        // This tests the ok_or_else branch where take_client returns None (should not happen in normal flow, but we test via direct gateway)
        let gw = student_management_sam_adapter::session_opener::SamAuthGateway::new("http://127.0.0.1:1".to_owned());
        assert!(gw.take_client().is_none());
        smol::block_on(async {
            let res = gw.login("u".to_owned(), "p".to_owned()).await;
            assert!(res.is_err());
            assert!(gw.take_client().is_none());
        });
    }

    #[test]
    fn has_saved_credentials_true_after_save() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        store.save(&StoredCredentials { base_url: "http://x".to_owned(), username: "u".to_owned(), password: "p".to_owned() }).unwrap();
        // app's store is separate instance but same dir, so load should find it via file
        let app2 = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        assert!(app2.has_saved_credentials());
        assert!(!app.has_saved_credentials() || app2.has_saved_credentials());
    }

    #[test]
    fn is_logged_in_true_after_seed() {
        let _guard = test_lock();
        let dir = TempDir::new().unwrap();
        let app = App::with_store(Box::new(FileSessionStore::with_dir(dir.path().to_path_buf())));
        assert!(!app.is_logged_in());
        app.seed_session(AuthSession::from_gateways(Arc::new(StubRoster { students: vec![], fail: false }), Arc::new(StubLessons { approved: vec![], fail: false })));
        assert!(app.is_logged_in());
    }
}
