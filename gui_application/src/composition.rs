use std::sync::Arc;
use std::time::Duration;

use authentication::application::use_cases::{
    LoginAndRememberCredentialsUseCase, LogoutUseCase, RestoreSessionUseCase,
};
use credential_store::{FileCredentialStore, NoDataDirectory};
use sam::{
    authentication::adapters::gateways::AuthorizationGatewaySamImpl,
    client::{CacheTtl, SamClient, SamClientCacheDecorator, SamClientImpl, SystemClock},
    diagnostics::error_chain,
    http::SamOperations,
    lessons::adapters::gateways::{MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl},
    roster::adapters::gateways::StudentGatewaySamImpl,
};
use student::application::gateways::{MusicianProfileGateway, StudentLessonsGateway};
use student::application::use_cases::{
    AssessStudentProgressUseCase, RetrieveAllAvailableStudentsUseCase,
    RetrieveStudentLessonsUseCase,
};

use crate::api::ApplicationFacade;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);
const STUDENTS_CACHE_TTL: Duration = Duration::from_secs(300);
const LESSONS_CACHE_TTL: Duration = Duration::from_secs(60);

pub struct Config {
    pub base_url: String,
    pub auth_endpoint: String,
    pub dashboard_endpoint: String,
    pub students_listing_endpoint: String,
    pub student_lessons_endpoint: String,
}

impl Config {
    pub fn production() -> Self {
        Self {
            base_url: "https://musical.congregacao.org.br".to_owned(),
            auth_endpoint: "autenticar".to_owned(),
            dashboard_endpoint: "painel".to_owned(),
            students_listing_endpoint: "alunos/listagem".to_owned(),
            student_lessons_endpoint: "licoes/index".to_owned(),
        }
    }
}

pub fn build_application(config: &Config) -> Result<ApplicationFacade, String> {
    build_application_with(config, FileCredentialStore::new(), http_client_builder())
}

fn http_client_builder() -> reqwest::blocking::ClientBuilder {
    http_client_builder_with_timeout(REQUEST_TIMEOUT)
}

fn http_client_builder_with_timeout(timeout: Duration) -> reqwest::blocking::ClientBuilder {
    reqwest::blocking::Client::builder()
        .timeout(timeout)
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
}

fn build_application_with(
    config: &Config,
    credential_store: Result<FileCredentialStore, NoDataDirectory>,
    http_client_builder: reqwest::blocking::ClientBuilder,
) -> Result<ApplicationFacade, String> {
    let file_credential_store: Arc<FileCredentialStore> =
        Arc::new(credential_store.map_err(|e| error_chain(&e))?);

    let reqwest_client: reqwest::blocking::Client =
        http_client_builder.build().map_err(|e| error_chain(&e))?;

    let sam_operations: SamOperations = SamOperations::new(
        reqwest_client,
        &config.base_url,
        &config.auth_endpoint,
        &config.dashboard_endpoint,
        &config.students_listing_endpoint,
        &config.student_lessons_endpoint,
    );

    let sam_client: Arc<dyn SamClient + Send + Sync> = Arc::new(SamClientCacheDecorator::new(
        Arc::new(SamClientImpl::new(sam_operations)),
        Arc::new(SystemClock),
        CacheTtl {
            students: STUDENTS_CACHE_TTL,
            lessons: LESSONS_CACHE_TTL,
        },
    ));

    let sam_credential_gateway: Arc<AuthorizationGatewaySamImpl> =
        Arc::new(AuthorizationGatewaySamImpl::new(sam_client.clone()));

    let login_and_remember_credentials: LoginAndRememberCredentialsUseCase =
        LoginAndRememberCredentialsUseCase::new(
            sam_credential_gateway.clone(),
            file_credential_store.clone(),
        );

    let logout: LogoutUseCase = LogoutUseCase::new(file_credential_store.clone());

    let restore_session: RestoreSessionUseCase = RestoreSessionUseCase::new(
        file_credential_store.clone(),
        file_credential_store,
        sam_credential_gateway,
    );

    let sam_student_gateway: Arc<StudentGatewaySamImpl> =
        Arc::new(StudentGatewaySamImpl::new(sam_client.clone()));

    let retrieve_all_available_students: RetrieveAllAvailableStudentsUseCase =
        RetrieveAllAvailableStudentsUseCase::new(sam_student_gateway);

    let sam_student_lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync> =
        Arc::new(StudentLessonsGatewaySamImpl::new(sam_client.clone()));

    let sam_musician_profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync> =
        Arc::new(MusicianProfileGatewaySamImpl::new(sam_client));

    Ok(ApplicationFacade {
        login_and_remember_credentials,
        restore_session,
        logout,
        retrieve_all_available_students,
        retrieve_student_lessons: RetrieveStudentLessonsUseCase::new(
            sam_student_lessons_gateway.clone(),
        ),
        assess_student_progress: AssessStudentProgressUseCase::new(
            sam_musician_profile_gateway,
            sam_student_lessons_gateway,
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::authentication::LoginOutcome;
    use crate::api::roster::RetrieveAllAvailableStudentsOutcome;

    fn facade_talking_to(
        mock_server: &wiremock::MockServer,
    ) -> (ApplicationFacade, tempfile::TempDir) {
        let config: Config = Config {
            base_url: mock_server.uri(),
            auth_endpoint: "autenticar".to_owned(),
            dashboard_endpoint: "painel".to_owned(),
            students_listing_endpoint: "alunos/listagem".to_owned(),
            student_lessons_endpoint: "licoes/index".to_owned(),
        };
        let credential_dir = tempfile::tempdir().expect("tempdir");
        let facade: ApplicationFacade = build_application_with(
            &config,
            Ok(FileCredentialStore::with_dir(
                &credential_dir.path().to_string_lossy(),
            )),
            http_client_builder(),
        )
        .expect("facade should be built");

        (facade, credential_dir)
    }

    #[test]
    fn a_login_redirect_is_read_as_success_instead_of_being_followed() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("POST"))
                .and(path("/autenticar"))
                .respond_with(ResponseTemplate::new(303).insert_header("Location", "/painel"))
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            let (facade, _credential_dir) = facade_talking_to(&mock_server);

            let result = facade.login("user@example.com".to_owned(), "hunter2".to_owned());

            assert_eq!(result, LoginOutcome::Successful);
        });
    }

    #[test]
    fn the_session_cookie_from_login_is_sent_on_later_requests() {
        use wiremock::matchers::{header, method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("POST"))
                .and(path("/autenticar"))
                .respond_with(
                    ResponseTemplate::new(303).insert_header("Set-Cookie", "session=abc; Path=/"),
                )
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .and(header("cookie", "session=abc"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            Mock::given(method("GET"))
                .and(path("/alunos/listagem"))
                .and(header("cookie", "session=abc"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(
                            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                        )
                        .insert_header("Content-Type", "application/json"),
                )
                .mount(&mock_server)
                .await;
            let (facade, _credential_dir) = facade_talking_to(&mock_server);
            assert_eq!(
                facade.login("user@example.com".to_owned(), "hunter2".to_owned()),
                LoginOutcome::Successful
            );

            let result = facade.retrieve_all_available_students();

            assert_eq!(
                result,
                RetrieveAllAvailableStudentsOutcome::Success {
                    students: Vec::new()
                }
            );
        });
    }

    #[test]
    fn repeated_reads_of_the_students_listing_hit_the_site_once() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;

            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;

            Mock::given(method("GET"))
                .and(path("/alunos/listagem"))
                .respond_with(
                    ResponseTemplate::new(200)
                        .set_body_string(
                            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                        )
                        .insert_header("Content-Type", "application/json"),
                )
                .mount(&mock_server)
                .await;

            let (facade, _credential_dir) = facade_talking_to(&mock_server);

            let _ = facade.retrieve_all_available_students();
            let _ = facade.retrieve_all_available_students();

            let listing_requests: usize = mock_server
                .received_requests()
                .await
                .expect("requests should have been recorded")
                .iter()
                .filter(|request| request.url.path() == "/alunos/listagem")
                .count();
            assert_eq!(listing_requests, 1);
        });
    }

    fn any_config() -> Config {
        Config {
            base_url: "http://127.0.0.1:1".to_owned(),
            auth_endpoint: "autenticar".to_owned(),
            dashboard_endpoint: "painel".to_owned(),
            students_listing_endpoint: "alunos/listagem".to_owned(),
            student_lessons_endpoint: "licoes/index".to_owned(),
        }
    }

    #[test]
    fn without_a_data_directory_the_application_cannot_be_built() {
        let result =
            build_application_with(&any_config(), Err(NoDataDirectory), http_client_builder());

        assert_eq!(
            result.err(),
            Some("no platform data directory is available to store credentials".to_owned())
        );
    }

    #[test]
    fn when_the_http_client_cannot_be_built_the_application_reports_why() {
        let credential_dir = tempfile::tempdir().expect("tempdir");
        let impossible_tls_range = reqwest::blocking::Client::builder()
            .min_tls_version(reqwest::tls::Version::TLS_1_3)
            .max_tls_version(reqwest::tls::Version::TLS_1_2);

        let result = build_application_with(
            &any_config(),
            Ok(FileCredentialStore::under(credential_dir.path())),
            impossible_tls_range,
        );

        let message = result.err().expect("building should fail");
        assert!(
            message.contains(": "),
            "the message should include the underlying cause, got: {message}"
        );
    }

    #[test]
    fn a_response_slower_than_the_request_timeout_fails_as_a_timeout() {
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, MockServer, ResponseTemplate};

        smol::block_on(async {
            let mock_server: MockServer = MockServer::start().await;
            Mock::given(method("GET"))
                .and(path("/painel"))
                .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(1500)))
                .mount(&mock_server)
                .await;
            let client: reqwest::blocking::Client =
                http_client_builder_with_timeout(Duration::from_millis(200))
                    .build()
                    .expect("HTTP client should be built");

            let error: reqwest::Error = client
                .get(format!("{}/painel", mock_server.uri()))
                .send()
                .expect_err("the response should have been abandoned");

            assert!(error.is_timeout(), "expected a timeout, got: {error:?}");
        });
    }

    #[test]
    fn the_request_timeout_outlasts_the_slowest_sam_response_observed() {
        const SLOWEST_OBSERVED_RESPONSE: Duration = Duration::from_millis(87_500);

        assert!(REQUEST_TIMEOUT > SLOWEST_OBSERVED_RESPONSE);
    }
}
