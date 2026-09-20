use sam::client::{SamClient, SamClientImpl, SamCredentials};
use sam::http::SamOperations;
use wiremock::{Mock, MockServer, ResponseTemplate};

const EXPECTED_PATHS: [&str; 4] = [
    "/autenticar",
    "/painel",
    "/alunos/listagem",
    "/licoes/index/500132",
];

fn build_client(base_url: &str, endpoints: [&str; 4]) -> Result<SamClientImpl, reqwest::Error> {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let [authentication, dashboard, students_listing, student_lessons] = endpoints;

    Ok(SamClientImpl::new(SamOperations::new(
        http_client,
        base_url,
        authentication,
        dashboard,
        students_listing,
        student_lessons,
    )))
}

async fn sam_answering_every_request() -> MockServer {
    let mock_server: MockServer = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200))
        .mount(&mock_server)
        .await;

    mock_server
}

async fn paths_requested_by_every_operation(
    client: &SamClientImpl,
    mock_server: &MockServer,
) -> Option<Vec<String>> {
    let _ = client.login(&SamCredentials {
        login: "user@example.com".to_string(),
        password: "hunter2".to_string(),
    });
    let _ = client.students();
    let _ = client.student_lessons("500132");

    mock_server.received_requests().await.map(|requests| {
        requests
            .iter()
            .map(|request| request.url.path().to_string())
            .collect()
    })
}

#[test]
fn endpoints_given_without_a_leading_slash_are_requested_under_the_base_url() {
    smol::block_on(async {
        let mock_server: MockServer = sam_answering_every_request().await;
        let client: SamClientImpl = build_client(
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
        let client: SamClientImpl = build_client(
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
        let client: SamClientImpl = build_client(
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
