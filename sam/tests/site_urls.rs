use std::sync::Arc;

use authentication::application::gateways::AuthorizeCredentialGateway;
use authentication::domain::entities::{Credential, Email, Password};
use sam::authentication::adapters::gateways::AuthorizationGatewaySamImpl;
use sam::client::{SamClient, SamClientImpl};
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::StudentLessonsGatewaySamImpl;
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::{StudentGateway, StudentLessonsGateway};
use student::domain::entities::StudentId;
use wiremock::{Mock, MockServer, ResponseTemplate};

const EXPECTED_PATHS: [&str; 4] = [
    "/autenticar",
    "/painel",
    "/alunos/listagem",
    "/licoes/index/500132",
];

#[test]
fn endpoints_given_without_a_leading_slash_are_requested_under_the_base_url() {
    smol::block_on(async {
        let mock_server: MockServer = sam_answering_every_request().await;
        let client: Arc<dyn SamClient> = build_client(
            &mock_server.uri(),
            ["autenticar", "painel", "alunos/listagem", "licoes/index"],
        )
        .expect("HTTP client should be built");

        let paths: Vec<String> = paths_requested_by_every_operation(&client, &mock_server)
            .await
            .expect("requests should have been recorded");

        assert_eq!(paths, EXPECTED_PATHS);
    });
}

#[test]
fn a_leading_slash_on_an_endpoint_does_not_double_the_slash_after_the_base_url() {
    smol::block_on(async {
        let mock_server: MockServer = sam_answering_every_request().await;
        let client: Arc<dyn SamClient> = build_client(
            &mock_server.uri(),
            [
                "/autenticar",
                "/painel",
                "/alunos/listagem",
                "/licoes/index",
            ],
        )
        .expect("HTTP client should be built");

        let paths: Vec<String> = paths_requested_by_every_operation(&client, &mock_server)
            .await
            .expect("requests should have been recorded");

        assert_eq!(paths, EXPECTED_PATHS);
    });
}

#[test]
fn a_trailing_slash_on_the_base_url_does_not_double_the_slash_before_an_endpoint() {
    smol::block_on(async {
        let mock_server: MockServer = sam_answering_every_request().await;
        let client: Arc<dyn SamClient> = build_client(
            &format!("{}/", mock_server.uri()),
            ["/autenticar", "painel", "/alunos/listagem", "licoes/index"],
        )
        .expect("HTTP client should be built");

        let paths: Vec<String> = paths_requested_by_every_operation(&client, &mock_server)
            .await
            .expect("requests should have been recorded");

        assert_eq!(paths, EXPECTED_PATHS);
    });
}

async fn sam_answering_every_request() -> MockServer {
    let mock_server: MockServer = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    mock_server
}

fn build_client(
    base_url: &str,
    endpoints: [&str; 4],
) -> Result<Arc<dyn SamClient>, reqwest::Error> {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let [authentication, dashboard, students_listing, student_lessons] = endpoints;

    Ok(Arc::new(SamClientImpl::new(SamOperations::new(
        http_client,
        base_url,
        authentication,
        dashboard,
        students_listing,
        student_lessons,
    ))))
}

async fn paths_requested_by_every_operation(
    client: &Arc<dyn SamClient>,
    mock_server: &MockServer,
) -> Option<Vec<String>> {
    let _ = AuthorizationGatewaySamImpl::new(client.clone()).authorize(&Credential::new(
        Email::new("user@example.com".to_owned()),
        Password::new("hunter2".to_owned()),
    ));
    let _ = StudentGatewaySamImpl::new(client.clone()).get_available_records();
    let _ = StudentLessonsGatewaySamImpl::new(client.clone())
        .get_all_for_student_with_id(&StudentId::new("500132".to_owned()));

    mock_server.received_requests().await.map(|requests| {
        requests
            .iter()
            .map(|request| request.url.path().to_string())
            .collect()
    })
}
