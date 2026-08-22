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
            (999, ""),
            (
                998,
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

enum ScriptedResponse {
    Http {
        status: u16,
        body: &'static str,
    },
    TruncatedHttp {
        status: u16,
        declared_body_len: usize,
        actual_body: &'static str,
    },
    CloseConnection,
}

/// A minimal scripted HTTP server for simulating raw connection failures
/// (dropped connections and truncated bodies) that mock servers cannot express.
fn spawn_scripted_http_server(script: Vec<ScriptedResponse>) -> std::net::SocketAddr {
    use std::io::{Read, Write};
    use std::net::{TcpListener, TcpStream};

    fn find_header_end(request: &[u8]) -> Option<usize> {
        request.windows(4).position(|window| window == b"\r\n\r\n")
    }

    fn write_response(
        stream: &mut TcpStream,
        status: u16,
        declared_body_len: usize,
        body: &str,
    ) {
        let head: String = format!(
            "HTTP/1.1 {status} OK\r\nContent-Length: {declared_body_len}\r\nConnection: close\r\n\r\n"
        );

        let _ = stream.write_all(head.as_bytes());
        let _ = stream.write_all(body.as_bytes());
        let _ = stream.flush();
    }

    fn handle_connection(mut stream: TcpStream, action: ScriptedResponse) {
        const HEADER_TERMINATOR_LEN: usize = 4;

        let mut buffer: [u8; 4096] = [0; 4096];
        let mut received: Vec<u8> = Vec::new();
        let header_end: usize = loop {
            let read_bytes: usize = match stream.read(&mut buffer) {
                Ok(0) => return,
                Ok(read_bytes) => read_bytes,
                Err(_) => return,
            };
            received.extend_from_slice(&buffer[..read_bytes]);

            match find_header_end(&received) {
                Some(header_end) => break header_end,
                None if received.len() == buffer.len() => return,
                None => continue,
            }
        };

        let headers: String = String::from_utf8_lossy(&received[..header_end]).to_lowercase();
        let declared_body_len: usize = headers
            .split("\r\n")
            .find_map(|line| line.strip_prefix("content-length:"))
            .and_then(|value| value.trim().parse().ok())
            .unwrap_or(0);
        let already_received: usize = received.len() - (header_end + HEADER_TERMINATOR_LEN);
        let mut missing_body_bytes: usize = declared_body_len.saturating_sub(already_received);

        while missing_body_bytes > 0 {
            let read_bytes: usize = match stream.read(&mut buffer) {
                Ok(0) => break,
                Ok(read_bytes) => read_bytes,
                Err(_) => return,
            };
            missing_body_bytes -= missing_body_bytes.min(read_bytes);
        }

        match action {
            ScriptedResponse::CloseConnection => {}
            ScriptedResponse::Http { status, body } => {
                write_response(&mut stream, status, body.len(), body)
            }
            ScriptedResponse::TruncatedHttp {
                status,
                declared_body_len,
                actual_body,
            } => write_response(&mut stream, status, declared_body_len, actual_body),
        }
    }

    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("A port to be bound");
    let server_addr: std::net::SocketAddr = listener.local_addr().unwrap();

    std::thread::spawn(move || {
        for action in script {
            let (stream, _) = match listener.accept() {
                Ok(connection) => connection,
                Err(_) => return,
            };
            handle_connection(stream, action);
        }
    });

    server_addr
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
