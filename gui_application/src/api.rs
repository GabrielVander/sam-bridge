use std::sync::OnceLock;

use app_bootstrap::App;
use student_management::api::application::{LoginInput, StudentLessonsDto, StudentSummaryDto};

use crate::{lessons as lessons_mapper, roster as roster_mapper};
use crate::view_models::{ProgressResult, ProgressViewModel, StudentLessonsView, StudentListItem};

static APP: OnceLock<App> = OnceLock::new();

fn app() -> &'static App {
    APP.get_or_init(App::new)
}

pub fn has_saved_credentials() -> bool {
    app().has_saved_credentials()
}

pub async fn try_restore_session() -> bool {
    app().try_restore_session().await
}

pub async fn login(base_url: String, username: String, password: String) -> anyhow::Result<()> {
    app().login(LoginInput {
        baseUrl: base_url,
        username,
        password,
    })
    .await
    .map(|_| ())
}

pub fn logout() {
    app().logout();
}

pub fn is_logged_in() -> bool {
    app().is_logged_in()
}

pub async fn retrieve_students() -> anyhow::Result<Vec<StudentListItem>> {
    let dtos: Vec<StudentSummaryDto> = app().retrieve_students().await?;
    Ok(roster_mapper::to_list_items(&dtos))
}

pub async fn retrieve_student_progress(
    student_id: String,
    assigned_level: String,
) -> anyhow::Result<ProgressResult> {
    let level = student_management::api::domain::MusicianLevel::parse_named(&assigned_level);
    if level.is_unknown() {
        return Ok(ProgressResult::unknown(
            level.unknown_raw().unwrap_or_default().to_owned(),
            "nível não reconhecido".to_owned(),
        ));
    }
    match app().calculate_progress(&student_id, &level).await {
        Ok(report) => Ok(ProgressResult::available(ProgressViewModel::from(&report))),
        Err(e) if e.to_string().contains("UnknownLevel") => Ok(ProgressResult::unknown(
            assigned_level,
            "nível não reconhecido".to_owned(),
        )),
        Err(e) => Err(e),
    }
}

pub async fn retrieve_student_lessons(student_id: String) -> anyhow::Result<StudentLessonsView> {
    let bundle: StudentLessonsDto = app().retrieve_student_lessons(&student_id).await?;
    Ok(lessons_mapper::to_view(&bundle))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use app_bootstrap::AuthSession;
    use student_management::api::{
        application::{StudentLessonsGateway, StudentsRetrievalGateway},
        domain::{Lesson as DomainLesson, Region, Student, StudentLessons},
    };

    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .unwrap_or_else(|e| e.into_inner())
    }

    #[derive(Clone, Default)]
    struct StubRoster {
        students: Vec<Student>,
    }
    #[async_trait::async_trait]
    impl StudentsRetrievalGateway for StubRoster {
        async fn get_available_records(&self) -> anyhow::Result<Vec<Student>> {
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
        let _ = has_saved_credentials();
    }

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

    #[test]
    fn retrieve_student_progress_known_and_unknown() {
        let _guard = test_lock();
        let app = app();
        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster { students: vec![] }),
            Arc::new(StubLessons { approved: vec![] }),
        ));
        smol::block_on(async {
            let known = retrieve_student_progress("7".to_owned(), "Candidate".to_owned()).await.expect("known should succeed");
            assert!(!known.is_unknown);
            assert!(!known.unknown.raw.contains("Candidate"));
            let unknown = retrieve_student_progress("7".to_owned(), "UnknownLevelXYZ".to_owned()).await.expect("unknown should return ProgressResult not err");
            assert!(unknown.is_unknown);
            assert_eq!(unknown.unknown.raw, "UnknownLevelXYZ");
            let via_calculate_error = retrieve_student_progress("7".to_owned(), "Candidate".to_owned()).await;
            assert!(via_calculate_error.is_ok());
        });
        logout();
    }

    #[test]
    fn retrieve_students_empty_and_progress_default() {
        let _guard = test_lock();
        let app = app();
        app.seed_session(AuthSession::from_gateways(
            Arc::new(StubRoster { students: vec![] }),
            Arc::new(StubLessons { approved: vec![] }),
        ));
        smol::block_on(async {
            let students = retrieve_students().await.expect("students");
            assert!(students.is_empty());
            let progress = retrieve_student_progress("7".to_owned(), "Candidate".to_owned()).await.expect("progress");
            assert!(!progress.is_unknown);
        });
        logout();
    }
}
