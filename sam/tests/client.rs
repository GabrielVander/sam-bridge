
use sam::test_support::{ScriptedResponse, spawn_scripted_http_server};

use anyhow::{Error, Result};
use sam::client::{
    LessonsReader, MsaLesson, MtdLesson, RosterReader, SamClient, SamCredentials, SamStudent,
    StudentLessonsPage,
};

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

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client
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

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client
            .login(&credentials)
            .expect("Login should succeed");

        let result: Result<Vec<SamStudent>> = client.students();
        assert!(
            result.is_ok(),
            "Expected students retrieval to succeed but got {:#?}",
            result
        );

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

        let client: SamClient =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let mut client = client;
        let result: Result<()> = client.login(&credentials);
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

        let client: SamClient =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let mut client = client;
        let result: Result<()> = client.login(&credentials);
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

        let client: SamClient =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let mut client = client;
        let result: Result<()> = client.login(&credentials);
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

        let client: SamClient =
            SamClient::new(mock_server.uri()).expect("Client should be created");

        let mut client = client;
        let result: Result<()> = client.login(&credentials);
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

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client.login(&credentials).unwrap();

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
                format!("{:?}", err).starts_with(&format!(
                    "Unexpected status for student listing response: {status_code}"
                )),
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

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client.login(&credentials).unwrap();

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

    let mut client: SamClient =
        SamClient::new(format!("http://127.0.0.1:{unused_port}"))
            .expect("Client should be created");

    let result: Result<()> = client.login(&build_valid_credentials());

    assert!(
        result.is_err(),
        "Expected login to fail gracefully but got {:#?}",
        result
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .starts_with("Login request failed"),
        "Expected a login request failure"
    );
}

#[test]
fn given_server_becoming_unreachable_students_should_fail_with_session_error() {
    let server_addr: std::net::SocketAddr = spawn_scripted_http_server(vec![
        ScriptedResponse::Http {
            status: 303,
            body: "",
        },
        ScriptedResponse::CloseConnection,
    ]);

    let mut client: SamClient = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created");
    client
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
        ScriptedResponse::Http {
            status: 303,
            body: "",
        },
        ScriptedResponse::Http {
            status: 200,
            body: "<html>dashboard</html>",
        },
        ScriptedResponse::CloseConnection,
    ]);

    let mut client: SamClient = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created");
    client
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
        ScriptedResponse::Http {
            status: 303,
            body: "",
        },
        ScriptedResponse::Http {
            status: 200,
            body: "<html>dashboard</html>",
        },
        ScriptedResponse::TruncatedHttp {
            status: 200,
            declared_body_len: 1000,
            actual_body: r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"dat"#,
        },
    ]);

    let mut client: SamClient = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created");
    client
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

#[test]
fn given_student_lessons_page_should_parse_both_tables_from_a_single_fetch() {
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
            .and(wiremock::matchers::path("/licoes/index/500132"))
            .respond_with(html_response(&student_lessons_page(
                &msa_table_fragment(
                    r#"<tr id="msa_559783" role="row" class="even">
                        <td>09/09/2025</td>
                        <td>4.5 - 4.5</td>
                        <td>38 - 38</td>
                        <td>7 - 8</td>
                        <td>Sol</td>
                        <td>Passou lições 7 e 8, estudar próximas lições.</td>
                        <td>MARCOS ROGÉRIO COSME</td>
                    </tr>"#,
                ),
                &mtd_table_fragment(
                    r#"<tr id="mtd_214020" role="row" class="even">
                        <td>00</td>
                        <td>00</td>
                        <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
                        <td>04/12/2023</td>
                        <td>MURILO FAGNER CARDOSO</td>
                        <td>04/12/2023 21:17:17</td>
                        <td>Postura do violino </td>
                    </tr>"#,
                ),
            )))
            .mount(&mock_server)
            .await;

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client
            .login(&credentials)
            .expect("Login should succeed");

        let page: StudentLessonsPage =
            client.student_lessons("500132").expect("Lessons retrieval should succeed");

        assert_eq!(
            page.msa,
            vec![MsaLesson {
                id: Some("559783".to_string()),
                date: Some("09/09/2025".to_string()),
                phases: Some("4.5 - 4.5".to_string()),
                pages: Some("38 - 38".to_string()),
                lessons: Some("7 - 8".to_string()),
                clefs: Some("Sol".to_string()),
                description: Some("Passou lições 7 e 8, estudar próximas lições.".to_string()),
                authorizer: Some("MARCOS ROGÉRIO COSME".to_string()),
            }]
        );
        assert_eq!(
            page.method,
            vec![MtdLesson {
                id: Some("214020".to_string()),
                pages: Some("00".to_string()),
                lesson: Some("00".to_string()),
                method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_string()),
                date: Some("04/12/2023".to_string()),
                authorizer: Some("MURILO FAGNER CARDOSO".to_string()),
                registration_date: Some("04/12/2023 21:17:17".to_string()),
                observations: Some("Postura do violino".to_string()),
            }]
        );

        let received_requests: Vec<wiremock::Request> = mock_server
            .received_requests()
            .await
            .expect("All requests should have been recorded");

        assert_eq!(
            received_requests
                .iter()
                .filter(|request| request.url.path() == "/licoes/index/500132")
                .count(),
            1,
            "Both lesson kinds must come from exactly one fetch"
        );
        assert!(
            received_requests
                .iter()
                .all(|request| request.url.path() != "/painel"),
            "Student lessons should not require a dashboard warm-up"
        );
    });
}

#[test]
fn given_page_without_method_table_should_return_empty_method_list() {
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
            .and(wiremock::matchers::path("/licoes/index/500132"))
            .respond_with(html_response(&student_lessons_page(
                &msa_table_fragment(
                    "<tr id=\"msa_1\"><td>01/01/2024</td><td>1.0</td><td>1</td><td></td><td></td><td>obs</td><td></td></tr>",
                ),
                "<!-- no datatable3 on this page -->",
            )))
            .mount(&mock_server)
            .await;

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client
            .login(&credentials)
            .expect("Login should succeed");

        let page: StudentLessonsPage =
            client.student_lessons("500132").expect("Should not fail on missing table");

        assert_eq!(page.msa.len(), 1);
        assert!(page.method.is_empty());
        assert_eq!(page.msa[0].authorizer, None, "Absent authorizer stays absent");
    });
}

#[test]
fn given_unexpected_status_for_lessons_should_fail_with_status_error() {
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
            .and(wiremock::matchers::path("/licoes/index/500132"))
            .respond_with(wiremock::ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let mut client: SamClient = SamClient::new(mock_server.uri())
            .expect("Client should be created");
        client
            .login(&credentials)
            .expect("Login should succeed");

        let result: Result<StudentLessonsPage> = client.student_lessons("500132");

        assert!(result.is_err(), "Expected an Err but got {:#?}", result);
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("Unexpected status for student lessons response: 500"),
            "Expected an unexpected-status error"
        );
    });
}

#[test]
fn given_lessons_connection_dropped_should_fail() {
    let server_addr: std::net::SocketAddr = spawn_scripted_http_server(vec![
        ScriptedResponse::Http {
            status: 303,
            body: "",
        },
        ScriptedResponse::CloseConnection,
    ]);

    let mut client: SamClient = SamClient::new(format!("http://{server_addr}"))
        .expect("Client should be created");
    client
        .login(&build_valid_credentials())
        .expect("Login should succeed");

    let result: Result<StudentLessonsPage> = client.student_lessons("500132");

    assert!(
        result.is_err(),
        "Expected lessons retrieval to fail but got {:#?}",
        result
    );
    assert!(
        result
            .unwrap_err()
            .to_string()
            .starts_with("Student lessons request failed"),
        "Expected a student lessons request failure"
    );
}

fn student_lessons_page(msa_table_html: &str, mtd_table_html: &str) -> String {
    format!("<html><body>{msa_table_html}{mtd_table_html}</body></html>")
}

fn msa_table_fragment(rows_html: &str) -> String {
    format!(
        r#"<div id="msa"><table id="datatable1" class="table table-striped dataTable no-footer" role="grid">
    <thead><tr><th>Data da Lição</th><th>Fases</th><th>Paginas</th><th>Lições</th><th>Claves</th><th>Observações</th><th>Autorizante</th><th>Ações</th></tr></thead>
    <tbody>{rows_html}</tbody>
</table></div>"#
    )
}

fn mtd_table_fragment(rows_html: &str) -> String {
    format!(
        r#"<table id="datatable3" class="table table-striped table-bordered table-hover dataTable no-footer" role="grid">
    <thead><tr><th>Páginas</th><th>Lição</th><th>Método</th><th>Data da Lição</th><th>Autorizante</th><th>Data de Cadastro</th><th>Observações</th><th>Ações</th></tr></thead>
    <tbody>{rows_html}</tbody>
</table>"#
    )
}

fn html_response(body: &str) -> wiremock::ResponseTemplate {
    wiremock::ResponseTemplate::new(200)
        .set_body_string(body)
        .insert_header("Content-Type", "text/html")
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

#[test]
fn readers_work_without_authentication_for_error_paths() {
    let client = SamClient::new("http://127.0.0.1:1").expect("Client should be created");

    assert!(RosterReader::students(&client).is_err());
    assert!(LessonsReader::student_lessons(&client, "1").is_err());
}
