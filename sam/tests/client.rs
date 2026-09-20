use sam::client::{MsaLesson, MtdLesson, SamClient, SamClientImpl, StudentLessonsPage};
use sam::http::SamOperations;
use test_support::sam_site::sam_client_for;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_client(mock_server: &MockServer) -> Result<SamClientImpl, reqwest::Error> {
    sam_client_for(&mock_server.uri())
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

fn student_lessons_page(msa_table_html: &str, mtd_table_html: &str) -> String {
    format!("<html><body>{msa_table_html}{mtd_table_html}</body></html>")
}

fn html_response(body: &str) -> ResponseTemplate {
    ResponseTemplate::new(200)
        .set_body_string(body)
        .insert_header("Content-Type", "text/html")
}

#[test]
fn given_student_lessons_page_should_parse_both_tables_from_a_single_fetch() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
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

        let client: SamClientImpl = build_client(&mock_server).expect("client should be built");

        let page: StudentLessonsPage = client
            .student_lessons("500132")
            .expect("Lessons retrieval should succeed");

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
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(html_response(&student_lessons_page(
                &msa_table_fragment(
                    "<tr id=\"msa_1\"><td>01/01/2024</td><td>1.0</td><td>1</td><td></td><td></td><td>obs</td><td></td></tr>",
                ),
                "<!-- no datatable3 on this page -->",
            )))
            .mount(&mock_server)
            .await;

        let client: SamClientImpl = build_client(&mock_server).expect("client should be built");

        let page: StudentLessonsPage = client
            .student_lessons("500132")
            .expect("Should not fail on missing table");

        assert_eq!(page.msa.len(), 1);
        assert_eq!(page.method, Vec::new());
        assert_eq!(
            page.msa[0].authorizer, None,
            "Absent authorizer stays absent"
        );
    });
}

#[test]
fn given_unexpected_status_for_lessons_should_fail() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let client: SamClientImpl = build_client(&mock_server).expect("client should be built");

        let result: Result<StudentLessonsPage, _> = client.student_lessons("500132");

        assert!(result.is_err(), "Expected an Err but got {result:#?}");
    });
}

#[test]
fn given_a_truncated_response_should_fail_to_decode() {
    use std::io::{Read, Write};
    use std::net::TcpListener;

    let listener: TcpListener = TcpListener::bind("127.0.0.1:0").expect("should bind");
    let addr: std::net::SocketAddr = listener.local_addr().expect("should have a local address");

    let server: std::thread::JoinHandle<()> = std::thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut buf: [u8; 1024] = [0; 1024];
            let _ = stream.read(&mut buf);
            let _ = stream.write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\nShort");
        }
    });

    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("HTTP client should be built");

    let sam_operations: SamOperations = SamOperations::new(
        http_client,
        &format!("http://{addr}"),
        "autenticar",
        "painel",
        "alunos/listagem",
        "licoes/index",
    );
    let client: SamClientImpl = SamClientImpl::new(sam_operations);

    let result: Result<StudentLessonsPage, _> = client.student_lessons("500132");

    assert!(
        result.is_err(),
        "Expected a decode failure from a truncated body but got {result:#?}"
    );

    server.join().expect("server thread should not panic");
}

#[test]
fn given_connection_refused_should_fail() {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("HTTP client should be built");

    let sam_operations: SamOperations = SamOperations::new(
        http_client,
        "http://127.0.0.1:1",
        "autenticar",
        "painel",
        "alunos/listagem",
        "licoes/index",
    );
    let client: SamClientImpl = SamClientImpl::new(sam_operations);

    let result: Result<StudentLessonsPage, _> = client.student_lessons("500132");

    assert!(
        result.is_err(),
        "Expected lessons retrieval to fail but got {result:#?}"
    );
}

#[test]
fn unexpected_response_error_message_includes_its_context() {
    let error: sam::client::SamClientError = sam::client::SamClientError::UnexpectedResponse {
        context: "missing table".to_string(),
    };

    assert!(error.to_string().contains("missing table"), "got: {error}");
}
