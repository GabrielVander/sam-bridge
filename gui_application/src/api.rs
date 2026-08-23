use std::sync::{OnceLock, RwLock};

use student_management_sam_adapter::{Authenticated, SamClient};

use crate::compose::AppGateways;
use crate::slices;
use crate::view_models::{StudentLessonsView, StudentListItem};

struct Session {
    gateways: AppGateways,
}

static SESSION: OnceLock<RwLock<Option<Session>>> = OnceLock::new();

fn session_slot() -> &'static RwLock<Option<Session>> {
    SESSION.get_or_init(|| RwLock::new(None))
}

/// Authenticates against SAM and keeps the session for subsequent calls.
pub async fn login(base_url: String, username: String, password: String) -> anyhow::Result<()> {
    let client: SamClient<Authenticated> =
        slices::authentication::login(base_url, username, password).await?;

    *session_slot()
        .write()
        .expect("Session lock should not be poisoned") = Some(Session {
        gateways: AppGateways::from_session(&client),
    });

    Ok(())
}

/// Clears the current session, if any.
pub fn logout() {
    *session_slot()
        .write()
        .expect("Session lock should not be poisoned") = None;
}

/// Whether a session is currently active.
pub fn is_logged_in() -> bool {
    session_slot()
        .read()
        .expect("Session lock should not be poisoned")
        .is_some()
}

fn with_session<T>(
    f: impl FnOnce(&AppGateways) -> T,
) -> anyhow::Result<T> {
    let guard = session_slot()
        .read()
        .expect("Session lock should not be poisoned");
    let session = guard
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("Not authenticated"))?;
    Ok(f(&session.gateways))
}

/// Lists all students available in SAM.
pub async fn retrieve_students() -> anyhow::Result<Vec<StudentListItem>> {
    let gateways = with_session(|g| g.clone())?;
    slices::roster::load(&gateways.roster).await
}

/// Loads both MSA and Method lessons for a student, most recent first.
pub async fn retrieve_student_lessons(student_id: String) -> anyhow::Result<StudentLessonsView> {
    let gateways = with_session(|g| g.clone())?;
    slices::lessons::load(&gateways.lessons, &student_id).await
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The SESSION slot is process-global. nextest isolates each test into
    /// its own process, but plain `cargo test` shares one — serialize every
    /// session-touching test so they cannot interleave.
    fn test_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<std::sync::Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("Test mutex is never poisoned: no panics while held")
    }

    /// nextest runs every test in its own process, so mutating the global
    /// SESSION slot is safe here.
    async fn mock_sam() -> wiremock::MockServer {
        let server = wiremock::MockServer::start().await;

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/autenticar"))
            .respond_with(wiremock::ResponseTemplate::new(303))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/painel"))
            .respond_with(wiremock::ResponseTemplate::new(200))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/alunos/listagem"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["7","ALUNA SETE","BAIRRO <span class='m-r-10'></span> | BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","VIOLINO","CANDIDATO(A)"]]}"#,
            ))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/licoes/index/7"))
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                r#"<html><body><div id="msa"><table id="datatable1"><tbody>
                    <tr id="msa_11"><td>01/06/2025</td><td>1.0 - 1.0</td><td>10 - 10</td><td></td><td></td><td>a</td><td>AUTH</td></tr>
                    <tr id="msa_12"><td>02/06/2025</td><td>1.0 - 1.0</td><td>10 - 10</td><td></td><td></td><td>b</td><td>AUTH</td></tr>
                </tbody></table></div></body></html>"#,
            ))
            .mount(&server)
            .await;

        // MSA and Method share /licoes/index/{id}; the page already carries
        // both tables, so no separate method endpoint is mocked.

        server
    }

    #[test]
    fn given_no_session_queries_should_fail_not_panic() {
        let _session_guard = test_lock();
        logout();

        smol::block_on(async {
            assert!(!is_logged_in());
            assert!(retrieve_students().await.is_err());
            assert!(retrieve_student_lessons("7".to_owned()).await.is_err());
        });
    }

    #[test]
    fn given_bad_credentials_should_fail_and_stay_logged_out() {
        let _session_guard = test_lock();
        // The SESSION slot is process-global; clear it so this test is
        // independent of sibling tests when run in a shared process.
        logout();

        smol::block_on(async {
            let server = wiremock::MockServer::start().await;
            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200)
                        .set_body_string("<p>* Oops...</p>"),
                )
                .mount(&server)
                .await;

            let result = login(server.uri(), "u".to_owned(), "wrong".to_owned()).await;

            assert!(result.is_err());
            assert!(!is_logged_in());
        });
    }

    #[test]
    fn given_login_full_flow_should_expose_students_and_sorted_lessons() {
        let _session_guard = test_lock();

        smol::block_on(async {
            let server = mock_sam().await;

            login(server.uri(), "u".to_owned(), "p".to_owned())
                .await
                .expect("Login should succeed");
            assert!(is_logged_in());

            let students = retrieve_students().await.expect("Students should load");
            assert_eq!(students.len(), 1);
            assert_eq!(students[0].name, "ALUNA SETE");

            let view = retrieve_student_lessons("7".to_owned())
                .await
                .expect("Lessons should load");
            let dates: Vec<&str> = view.msa.iter().map(|i| i.id.as_str()).collect();
            assert_eq!(dates, vec!["12", "11"], "Most recent lesson must come first");

            logout();
            assert!(!is_logged_in());
        });
    }
}
