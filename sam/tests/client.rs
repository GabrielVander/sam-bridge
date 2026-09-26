use sam::client::{
    MsaLesson, MtdLesson, SamClient, SamClientError, SamClientImpl, SamStudent, StudentLessonsPage,
};
use sam::http::SamOperations;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[path = "support/helpers.rs"]
mod support;
use support::{UNREACHABLE_SITE, sam_client_for};

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
        UNREACHABLE_SITE,
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

#[test]
fn every_msa_row_is_read_with_all_its_fields() {
    let lessons: Vec<MsaLesson> = msa_lessons_listed(
        r#"<tr id="msa_538784" role="row" class="odd">
            <td>19/08/2025</td>
            <td>3.4 - 4.1</td>
            <td>30 - 34</td>
            <td></td>
            <td></td>
            <td>Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32. </td>
            <td>ELIAS BRANDE</td>
            <td><button onclick="delete_lancamento_msa(538784)">Apagar</button></td>
        </tr><tr id="msa_559783" role="row" class="even">
            <td>09/09/2025</td>
            <td>4.5 - 4.5</td>
            <td>38 - 38</td>
            <td>7 - 8</td>
            <td>Sol</td>
            <td>Passou lições 7 e 8, estudar próximas lições.</td>
            <td>MARCOS ROGÉRIO COSME</td>
            <td><button onclick="delete_lancamento_msa(559783)">Apagar</button></td>
        </tr>"#,
    )
    .unwrap();

    assert_eq!(
        lessons,
        vec![
            MsaLesson {
                id: Some("538784".to_owned()),
                date: Some("19/08/2025".to_owned()),
                phases: Some("3.4 - 4.1".to_owned()),
                pages: Some("30 - 34".to_owned()),
                lessons: None,
                clefs: None,
                description: Some(
                    "Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32.".to_owned()
                ),
                authorizer: Some("ELIAS BRANDE".to_owned()),
            },
            MsaLesson {
                id: Some("559783".to_owned()),
                date: Some("09/09/2025".to_owned()),
                phases: Some("4.5 - 4.5".to_owned()),
                pages: Some("38 - 38".to_owned()),
                lessons: Some("7 - 8".to_owned()),
                clefs: Some("Sol".to_owned()),
                description: Some("Passou lições 7 e 8, estudar próximas lições.".to_owned()),
                authorizer: Some("MARCOS ROGÉRIO COSME".to_owned()),
            },
        ]
    );
}

#[test]
fn an_msa_row_without_any_field_is_still_read() {
    let lessons: Vec<MsaLesson> = msa_lessons_listed(
        r#"<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr><tr id="msa_1"></tr>"#,
    ).unwrap();

    assert_eq!(
        lessons,
        vec![
            MsaLesson::default(),
            MsaLesson {
                id: Some("1".to_owned()),
                ..MsaLesson::default()
            }
        ],
        "rows without an authorizer or any other field must not be dropped"
    );
}

#[test]
fn a_cell_with_inline_markup_is_read_as_its_text() {
    let lessons: Vec<MsaLesson> = msa_lessons_listed(
        "<tr><td></td><td></td><td></td><td></td><td></td><td>  Revisão: <b>Ligaduras</b>. Estudar.  </td><td></td></tr>",
    ).unwrap();

    assert_eq!(
        lessons,
        vec![MsaLesson {
            description: Some("Revisão:  Ligaduras . Estudar.".to_owned()),
            ..MsaLesson::default()
        }]
    );
}

#[test]
fn an_empty_msa_table_has_no_lessons() {
    assert_eq!(msa_lessons_listed("").unwrap(), Vec::new());
}

#[test]
fn a_page_without_lesson_tables_has_no_lessons() {
    for body in [
        "",
        "   ",
        "<html><body><h1>Informação não encontrada</h1></body></html>",
        r#"<html><body><div id="msa">Nenhum registro encontrado</div></body></html>"#,
        r#"<html><body><div id="msa"><table id="datatable1"><thead><tr><th>Data da Lição</th></tr></thead></table></div><table id="datatable3"><thead><tr><th>Páginas</th></tr></thead></table></body></html>"#,
    ] {
        assert_eq!(
            lessons_page_served(body),
            Some(StudentLessonsPage::default()),
            "page {body:?}"
        );
    }
}

#[test]
fn every_method_row_is_read_with_all_its_fields() {
    let lessons: Vec<MtdLesson> = method_lessons_listed(
        r#"<tr id="mtd_738654" role="row" class="odd">
            <td>00</td>
            <td>00</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>24/03/2026</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>30/03/2026 18:38:29</td>
            <td>Revisão para RJM </td>
            <td><button onclick="delete_lancamento_mtd(738654)">Apagar</button></td>
        </tr><tr id="mtd_190204" role="row" class="even">
            <td>11</td>
            <td>7</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>23/10/2023</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>23/10/2023 20:35:24</td>
            <td></td>
            <td><button onclick="delete_lancamento_mtd(190204)">Apagar</button></td>
        </tr>"#,
    )
    .unwrap();

    assert_eq!(
        lessons,
        vec![
            MtdLesson {
                id: Some("738654".to_owned()),
                pages: Some("00".to_owned()),
                lesson: Some("00".to_owned()),
                method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                date: Some("24/03/2026".to_owned()),
                authorizer: Some("MURILO FAGNER CARDOSO".to_owned()),
                registration_date: Some("30/03/2026 18:38:29".to_owned()),
                observations: Some("Revisão para RJM".to_owned()),
            },
            MtdLesson {
                id: Some("190204".to_owned()),
                pages: Some("11".to_owned()),
                lesson: Some("7".to_owned()),
                method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                date: Some("23/10/2023".to_owned()),
                authorizer: Some("MURILO FAGNER CARDOSO".to_owned()),
                registration_date: Some("23/10/2023 20:35:24".to_owned()),
                observations: None,
            },
        ]
    );
}

#[test]
fn a_method_row_without_any_field_is_still_read() {
    let lessons: Vec<MtdLesson> = method_lessons_listed(
        "<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr>",
    )
    .unwrap();

    assert_eq!(lessons, vec![MtdLesson::default()]);
}

#[test]
fn an_empty_method_table_has_no_lessons() {
    assert_eq!(method_lessons_listed("").unwrap(), Vec::new());
}

#[test]
fn every_student_row_is_read_with_all_its_columns() {
    let result: Result<Vec<SamStudent>, SamClientError> = students_listing_served(
        200,
        r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["99999","CARLOS ALBERTO DE NOBREGA","JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","A DEFINIR","CANDIDATO(A)","99999","0"]]}"#,
    ).unwrap();

    assert_eq!(
        result.expect("the listing should be read"),
        vec![listed_student(
            "99999",
            "CARLOS ALBERTO DE NOBREGA",
            "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS",
            "MÚSICO",
            "A DEFINIR",
            "CANDIDATO(A)",
        )]
    );
}

#[test]
fn an_empty_listing_has_no_students() {
    let result: Result<Vec<SamStudent>, SamClientError> = students_listing_served(
        200,
        r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
    )
    .unwrap();

    assert_eq!(result.expect("the listing should be read"), Vec::new());
}

#[test]
fn unknown_fields_and_extra_columns_are_ignored() {
    let result: Result<Vec<SamStudent>, SamClientError> = students_listing_served(
        200,
        r#"{"other":"field","data":[["1","N","L","R","I","LV","EXTRA-1","EXTRA-2"]]}"#,
    )
    .unwrap();

    assert_eq!(
        result.expect("the listing should be read"),
        vec![listed_student("1", "N", "L", "R", "I", "LV")]
    );
}

#[test]
fn missing_trailing_columns_are_read_as_empty() {
    let result: Result<Vec<SamStudent>, SamClientError> =
        students_listing_served(200, r#"{"data":[["1","NAME"]]}"#).unwrap();

    assert_eq!(
        result.expect("the listing should be read"),
        vec![listed_student("1", "NAME", "", "", "", "")]
    );
}

#[test]
fn a_listing_with_an_unexpected_status_fails_naming_it() {
    let result: Result<Vec<SamStudent>, SamClientError> = students_listing_served(
        500,
        r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
    )
    .unwrap();

    let error: SamClientError = result.expect_err("the listing should not be read");
    assert!(
        error
            .to_string()
            .contains("Unexpected status for student listing response: 500"),
        "got: {error}"
    );
}

#[test]
fn a_listing_that_is_not_the_expected_json_fails_to_decode() {
    for body in [
        "",
        "{}",
        r#"{"something": "else"}"#,
        r#"{"data": [["1", 2, "3", "4", "5", "6"]]}"#,
    ] {
        let error: SamClientError = students_listing_served(200, body)
            .unwrap()
            .expect_err("the listing should not be read");

        assert!(
            error
                .to_string()
                .contains("Unable to decode student listing JSON response"),
            "body {body:?} got: {error}"
        );
    }
}

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

fn lessons_page_served(body: &str) -> Option<StudentLessonsPage> {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(html_response(body))
            .mount(&mock_server)
            .await;

        build_client(&mock_server)
            .ok()?
            .student_lessons("500132")
            .ok()
    })
}

fn msa_lessons_listed(rows_html: &str) -> Option<Vec<MsaLesson>> {
    lessons_page_served(&student_lessons_page(&msa_table_fragment(rows_html), ""))
        .map(|page| page.msa)
}

fn method_lessons_listed(rows_html: &str) -> Option<Vec<MtdLesson>> {
    lessons_page_served(&student_lessons_page("", &mtd_table_fragment(rows_html)))
        .map(|page| page.method)
}

fn students_listing_served(
    status: u16,
    body: &str,
) -> Option<Result<Vec<SamStudent>, SamClientError>> {
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
                ResponseTemplate::new(status)
                    .set_body_string(body)
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        build_client(&mock_server)
            .ok()
            .map(|client| client.students())
    })
}

fn listed_student(
    id: &str,
    name: &str,
    location: &str,
    role: &str,
    instrument: &str,
    level: &str,
) -> SamStudent {
    SamStudent {
        id: id.to_owned(),
        name: name.to_owned(),
        location: location.to_owned(),
        role: role.to_owned(),
        instrument: instrument.to_owned(),
        level: level.to_owned(),
    }
}
