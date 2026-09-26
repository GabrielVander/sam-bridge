use std::time::Duration;

use credential_store::{FileCredentialStore, NoDataDirectory};
use gui_application::api::ApplicationFacade;
use gui_application::api::authentication::LoginOutcomeDto;
use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use gui_application::api::roster::RetrieveAllAvailableStudentsOutcomeDto;
use gui_application::composition::{
    Config, REQUEST_TIMEOUT, build_application_with, http_client_builder,
};
use pretty_assertions::assert_eq;
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const EMPTY_LISTING: &str = r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#;

fn config_for(base_url: &str) -> Config {
    Config {
        base_url: base_url.to_owned(),
        auth_endpoint: "autenticar".to_owned(),
        dashboard_endpoint: "painel".to_owned(),
        students_listing_endpoint: "alunos/listagem".to_owned(),
        student_lessons_endpoint: "licoes/index".to_owned(),
    }
}

/// A facade talking to `mock_server`, with its credentials kept in a temporary directory
/// that lives as long as the returned guard.
fn facade_talking_to(
    mock_server: &MockServer,
    timeout: Duration,
) -> Option<(ApplicationFacade, tempfile::TempDir)> {
    let credential_dir = tempfile::tempdir().ok()?;
    let facade = build_application_with(
        &config_for(&mock_server.uri()),
        Ok(FileCredentialStore::under(credential_dir.path())),
        http_client_builder(timeout),
    )
    .ok()?;

    Some((facade, credential_dir))
}

fn json_response(body: &str) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_string(body)
        .insert_header("Content-Type", "application/json")
}

#[test]
fn a_login_redirect_is_read_as_success_instead_of_being_followed() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/autenticar"))
            .respond_with(ResponseTemplate::new(303).insert_header("Location", "/painel"))
            .mount(&mock_server)
            .await;
        let (facade, _credential_dir) = facade_talking_to(&mock_server, REQUEST_TIMEOUT).unwrap();

        let result = facade.login("user@example.com".to_owned(), "hunter2".to_owned());

        assert_eq!(result, LoginOutcomeDto::Successful);
    });
}

#[test]
fn the_session_cookie_from_login_is_sent_on_later_requests() {
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
            .respond_with(json_response(EMPTY_LISTING))
            .mount(&mock_server)
            .await;
        let (facade, _credential_dir) = facade_talking_to(&mock_server, REQUEST_TIMEOUT).unwrap();
        assert_eq!(
            facade.login("user@example.com".to_owned(), "hunter2".to_owned()),
            LoginOutcomeDto::Successful
        );

        let result = facade.retrieve_all_available_students();

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcomeDto::Success {
                students: Vec::new()
            }
        );
    });
}

#[test]
fn repeated_reads_of_the_students_listing_hit_the_site_once() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;
        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(json_response(EMPTY_LISTING))
            .mount(&mock_server)
            .await;
        let (facade, _credential_dir) = facade_talking_to(&mock_server, REQUEST_TIMEOUT).unwrap();

        let _ = facade.retrieve_all_available_students();
        let _ = facade.retrieve_all_available_students();

        let listing_requests: usize = mock_server
            .received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|request| request.url.path() == "/alunos/listagem")
            .count();
        assert_eq!(listing_requests, 1);
    });
}

#[test]
fn a_sam_slower_than_the_request_timeout_fails_as_a_network_error() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/autenticar"))
            .respond_with(ResponseTemplate::new(303).set_delay(Duration::from_millis(1500)))
            .mount(&mock_server)
            .await;
        let (facade, _credential_dir) =
            facade_talking_to(&mock_server, Duration::from_millis(200)).unwrap();

        let result = facade.login("user@example.com".to_owned(), "hunter2".to_owned());

        let LoginOutcomeDto::Failure { report } = result else {
            panic!("the login should have been abandoned, got {result:?}");
        };
        assert_eq!(report.kind, ErrorKindDto::Network);
        assert!(
            report.details.contains("timed out"),
            "got: {}",
            report.details
        );
    });
}

#[test]
fn the_request_timeout_outlasts_the_slowest_sam_response_observed() {
    const SLOWEST_OBSERVED_RESPONSE: Duration = Duration::from_millis(87_500);

    assert!(REQUEST_TIMEOUT > SLOWEST_OBSERVED_RESPONSE);
}

#[test]
fn without_a_data_directory_the_application_cannot_be_built() {
    let result = build_application_with(
        &config_for("http://127.0.0.1:1"),
        Err(NoDataDirectory),
        http_client_builder(REQUEST_TIMEOUT),
    );

    assert_eq!(
        result.err().map(ErrorReportDto::from),
        Some(ErrorReportDto {
            kind: ErrorKindDto::LocalStorage,
            details: "Unable to set up the credential storage: \
                      no platform data directory is available to store credentials"
                .to_owned(),
        })
    );
}

#[test]
fn when_the_http_client_cannot_be_built_the_application_reports_why() {
    let credential_dir = tempfile::tempdir().unwrap();
    let impossible_tls_range = reqwest::blocking::Client::builder()
        .min_tls_version(reqwest::tls::Version::TLS_1_3)
        .max_tls_version(reqwest::tls::Version::TLS_1_2);

    let result = build_application_with(
        &config_for("http://127.0.0.1:1"),
        Ok(FileCredentialStore::under(credential_dir.path())),
        impossible_tls_range,
    );

    let report = result.err().map(ErrorReportDto::from).unwrap();
    assert_eq!(report.kind, ErrorKindDto::Unknown);
    assert!(
        report
            .details
            .starts_with("Unable to build the HTTP client: "),
        "the details should include the underlying cause, got: {}",
        report.details
    );
}
