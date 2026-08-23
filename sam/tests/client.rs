mod support;

use support::{ScriptedResponse, spawn_scripted_http_server};

use anyhow::{Error, Result};
use sam::client::{Authenticated, SamClient, SamCredentials, SamStudent, Unauthenticated};

#[test]
fn given_inaccessible_dashboard_students_should_fail_with_session_error() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_valid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_valid_credentials_response(),
        )
        .await;

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/painel"))
            .respond_with(build_simple_response(503))
            .mount(&mock_server)
            .await;

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/alunos/listagem"))
            .respond_with(build_students_listing_response(
                r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client: SamClient<Authenticated> = SamClient::new(mock_server.uri())
            .expect("Client should be created")
            .login(&credentials)
            .expect("Login should succeed");

        let result: Result<Vec<SamStudent>> = client.students();

        assert!(
            result.is_err(),
            "Expected students retrieval to fail without an accessible dashboard, but got {:#?}",
            result
        );
        assert!(
            result.unwrap_err().to_string().contains("Session"),
            "Expected a session-related error"
        );
    });
}

#[test]
fn given_accessible_dashboard_students_should_visit_it_before_listing() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_valid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_valid_credentials_response(),
        )
        .await;

        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/painel"))
            .respond_with(build_simple_response(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/alunos/listagem"))
            .respond_with(build_students_listing_response(
                r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
            ))
            .mount(&mock_server)
            .await;

        let client: SamClient<Authenticated> = SamClient::new(mock_server.uri())
            .expect("Client should be created")
            .login(&credentials)
            .expect("Login should succeed");

        let result: Result<Vec<SamStudent>> = client.students();
        assert!(result.is_ok(), "Expected students retrieval to succeed but got {:#?}", result);

        mock_server.verify().await;

        let received_requests: Vec<wiremock::Request> = mock_server
            .received_requests()
            .await
            .expect("All requests should have been recorded");

        let dashboard_request_index: Option<usize> = received_requests
            .iter()
            .position(|request| request.url.path() == "/painel");
        let listing_request_index: Option<usize> = received_requests
            .iter()
            .position(|request| request.url.path() == "/alunos/listagem");

        assert!(
            dashboard_request_index.is_some(),
            "Dashboard should have been visited"
        );
        assert!(
            listing_request_index.is_some(),
            "Students listing should have been requested"
        );
        assert!(
            dashboard_request_index.unwrap() < listing_request_index.unwrap(),
            "Dashboard visit should precede the students listing request, but got dashboard at index {:?} and listing at index {:?}",
            dashboard_request_index,
            listing_request_index,
        );
    });
}

#[test]
fn given_unexpected_response_status_code_login_should_fail() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_invalid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_simple_response(404),
        )
        .await;

        let client: SamClient<Unauthenticated> =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let result: Result<SamClient<Authenticated>> = client.login(&credentials);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Http error. Received unexpected response"
        )
    });
}

#[test]
fn given_unexpected_response_body_login_should_fail() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_invalid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_simple_response(200),
        )
        .await;

        let client: SamClient<Unauthenticated> =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let result: Result<SamClient<Authenticated>> = client.login(&credentials);
        assert_eq!(
            result.unwrap_err().to_string(),
            "Http error. Received unexpected response"
        )
    });
}

#[test]
fn given_invalid_credentials_login_should_fail() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_invalid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_invalid_credentials_response(),
        )
        .await;

        let client: SamClient<Unauthenticated> =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let result: Result<SamClient<Authenticated>> = client.login(&credentials);
        assert_eq!(result.unwrap_err().to_string(), "Invalid credentials")
    });
}

#[test]
fn given_valid_credentials_login_should_succeed() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_valid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_valid_credentials_response(),
        )
        .await;

        let client: SamClient<Unauthenticated> =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let result: Result<SamClient<Authenticated>> = client.login(&credentials);
        assert!(result.is_ok());
    });
}

#[test]
fn given_unexpected_status_for_students_listing_students_should_fail() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_valid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_valid_credentials_response(),
        )
        .await;

        let client: SamClient<Authenticated> = SamClient::new(mock_server.uri())
            .expect("Client should be created")
            .login(&credentials)
            .unwrap();

        let error_test_cases: Vec<(u16, &str)> = vec![
            (500, ""),
            (
                503,
                r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[]}"#,
            ),
        ];

        for (status_code, response_body) in error_test_cases {
            mock_server.reset().await;

            given_dashboard_endpoint_responds_with(&mock_server, build_simple_response(200)).await;

            given_students_listing_endpoint_responds_with(
                &mock_server,
                wiremock::ResponseTemplate::new(status_code).set_body_string(response_body),
            )
            .await;

            let result: Result<Vec<SamStudent>> = client.students();

            assert!(result.is_err(), "Expected an Err but got {:#?}", result);

            let err: Error = result.unwrap_err();
            assert!(
                format!("{:?}", err)
                    .starts_with(&format!("Unexpected status for student listing response: {status_code}")),
                "Expected an unexpected-status error but got {:#?}",
                err
            );

            mock_server.verify().await;
        }
    });
}

#[test]
fn given_valid_students_listing_students_should_be_mapped() {
    smol::block_on(async {
        let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;
        let credentials: SamCredentials = build_valid_credentials();

        given_credentials_authentication_endpoint_responds_with(
            &mock_server,
            &credentials,
            build_valid_credentials_response(),
        )
        .await;

        let client: SamClient<Authenticated> = SamClient::new(mock_server.uri())
            .expect("Client should be created")
            .login(&credentials)
            .unwrap();

        given_dashboard_endpoint_responds_with(&mock_server, build_simple_response(200)).await;

        given_students_listing_endpoint_responds_with(
            &mock_server,
            build_students_listing_response(
                r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"other":"field","data":[["99998","PEDRO ÁLVARES CABRAL","JARDIM PALMARES DO NORTE <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","VIOLINO","CANDIDATO(A)","99998","0"]]}"#,
            ),
        )
        .await;

        let result: Result<Vec<SamStudent>> = client.students();

        assert_eq!(
            result.expect("Students retrieval should succeed"),
            vec![SamStudent {
                id: "99998".to_string(),
                name: "PEDRO ÁLVARES CABRAL".to_string(),
                location: "JARDIM PALMARES DO NORTE <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS".to_string(),
                role: "MÚSICO".to_string(),
                instrument: "VIOLINO".to_string(),
                level: "CANDIDATO(A)".to_string()
            }]
        );

        mock_server.verify().await;
    });
}

#[test]
fn given_unreachable_server_login_should_return_err_instead_of_panicking() {
    let unused_port: u16 = free_local_port();

    let client: SamClient<Unauthenticated> =
        SamClient::new(format!("http://127.0.0.1:{unused_port}")).expect("Client should be created");

    let result: Result<SamClient<Authenticated>> = client.login(&build_valid_credentials());

    assert!(
        result.is_err(),
        "Expected login to fail gracefully but got {:#?}",
        result
    );
    assert!(
        result.unwrap_err().to_string().starts_with("Login request failed"),
        "Expected a login request failure"
    );
}

#[test]
fn given_server_becoming_unreachable_students_should_fail_with_session_error() {
    let server_addr: std::net::SocketAddr = spawn_scripted_http_server(vec![
        ScriptedResponse::Http { status: 303, body: "" },
        ScriptedResponse::CloseConnection,
    ]);

    let client: SamClient<Authenticated> = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created")
        .login(&build_valid_credentials())
        .expect("Login should succeed");

    let result: Result<Vec<SamStudent>> = client.students();

    assert!(
        result.is_err(),
        "Expected students retrieval to fail but got {:#?}",
        result
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .starts_with("Dashboard request failed"),
        "Expected a dashboard request failure"
    );
}

#[test]
fn given_listing_connection_dropped_students_should_fail() {
    let server_addr: std::net::SocketAddr = spawn_scripted_http_server(vec![
        ScriptedResponse::Http { status: 303, body: "" },
        ScriptedResponse::Http { status: 200, body: "<html>dashboard</html>" },
        ScriptedResponse::CloseConnection,
    ]);

    let client: SamClient<Authenticated> = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created")
        .login(&build_valid_credentials())
        .expect("Login should succeed");

    let result: Result<Vec<SamStudent>> = client.students();

    assert!(
        result.is_err(),
        "Expected students retrieval to fail but got {:#?}",
        result
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .starts_with("Students HTTP request failed"),
        "Expected a students listing request failure"
    );
}

#[test]
fn given_truncated_listing_body_students_should_fail() {
    let server_addr: std::net::SocketAddr = spawn_scripted_http_server(vec![
        ScriptedResponse::Http { status: 303, body: "" },
        ScriptedResponse::Http { status: 200, body: "<html>dashboard</html>" },
        ScriptedResponse::TruncatedHttp {
            status: 200,
            declared_body_len: 1000,
            actual_body: r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"dat"#,
        },
    ]);

    let client: SamClient<Authenticated> = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created")
        .login(&build_valid_credentials())
        .expect("Login should succeed");

    let result: Result<Vec<SamStudent>> = client.students();

    assert!(
        result.is_err(),
        "Expected students retrieval to fail but got {:#?}",
        result
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .starts_with("Unable to read students listing response body"),
        "Expected a listing response decoding failure"
    );
}

fn free_local_port() -> u16 {
    use std::net::TcpListener;

    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("A port to be bound");
    let port: u16 = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn build_invalid_credentials() -> SamCredentials {
    SamCredentials {
        login: "InvalidLogin".to_string(),
        password: "InvalidPassword".to_string(),
    }
}

fn build_valid_credentials() -> SamCredentials {
    SamCredentials {
        login: "ValidLogin".to_string(),
        password: "ValidPassword".to_string(),
    }
}

fn build_invalid_credentials_response() -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(200)
        .set_body_string("<p>* Oops... O usuário ou senha incorretos!</p>")
}

fn build_valid_credentials_response() -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(303)
}

fn build_simple_response(status_code: u16) -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(status_code)
}

fn build_students_listing_response(json_body: &str) -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(200)
        .set_body_string(json_body)
        .insert_header("Content-Type", "application/json")
}

async fn given_students_listing_endpoint_responds_with(
    server: &wiremock::MockServer,
    response: wiremock::ResponseTemplate,
) {
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/alunos/listagem"))
        .respond_with(response)
        .mount(server)
        .await;
}

async fn given_dashboard_endpoint_responds_with(
    server: &wiremock::MockServer,
    response: wiremock::ResponseTemplate,
) {
    wiremock::Mock::given(wiremock::matchers::method("GET"))
        .and(wiremock::matchers::path("/painel"))
        .respond_with(response)
        .mount(server)
        .await;
}

async fn given_credentials_authentication_endpoint_responds_with(
    server: &wiremock::MockServer,
    credentials: &SamCredentials,
    response: wiremock::ResponseTemplate,
) {
    wiremock::Mock::given(wiremock::matchers::method("POST"))
        .and(wiremock::matchers::path("/autenticar"))
        .and(wiremock::matchers::body_string_contains(format!(
            "login={}",
            credentials.login
        )))
        .and(wiremock::matchers::body_string_contains(format!(
            "password={}",
            credentials.password
        )))
        .respond_with(response)
        .mount(server)
        .await;
}
