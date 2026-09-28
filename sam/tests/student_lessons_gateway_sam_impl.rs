use std::sync::Arc;

use chrono::NaiveDate;
use sam::client::{MsaLesson, SamClient, SamClientImpl, StudentLessonsPage};
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::StudentLessonsGatewaySamImpl;
use student::application::gateways::{
    FailureKind, StudentLessonsGateway, StudentLessonsGatewayError,
};
use student::domain::entities::{Clef, Lesson, Range, StudentId, StudentLessons};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[path = "support/helpers.rs"]
mod support;
use support::{FakeSamClient, UNREACHABLE_SITE, sam_operations_for};

#[test]
fn given_lessons_page_should_map_both_categories_to_domain_lessons() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(student_lessons_page())
                    .insert_header("Content-Type", "text/html"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentLessonsGatewaySamImpl =
            build_gateway(&mock_server).expect("gateway should be built");

        let result: StudentLessons = gateway
            .get_all_for_student_with_id(&StudentId::new("500132".to_owned()))
            .expect("Lessons retrieval should succeed");

        assert_eq!(
            result,
            StudentLessons {
                msa: vec![Lesson {
                    id: Some("559783".to_owned()),
                    date: Some(NaiveDate::from_ymd_opt(2025, 9, 9).expect("valid date")),
                    phase: Some(Range {
                        from: "4.5".to_owned(),
                        to: "4.5".to_owned()
                    }),
                    page: Some(Range {
                        from: "38".to_owned(),
                        to: "38".to_owned()
                    }),
                    lesson: Some(Range {
                        from: "7".to_owned(),
                        to: "8".to_owned()
                    }),
                    clef: Some(Clef::G),
                    description: Some("Passou lições 7 e 8, estudar próximas lições.".to_owned()),
                    instructor: Some("MARCOS ROGÉRIO COSME".to_owned()),
                    method: None,
                }],
                method: vec![Lesson {
                    id: Some("214020".to_owned()),
                    date: Some(NaiveDate::from_ymd_opt(2023, 12, 4).expect("valid date")),
                    phase: None,
                    page: Some(Range {
                        from: "00".to_owned(),
                        to: "00".to_owned()
                    }),
                    lesson: Some(Range {
                        from: "00".to_owned(),
                        to: "00".to_owned()
                    }),
                    clef: None,
                    description: Some("Postura do violino".to_owned()),
                    instructor: Some("MURILO FAGNER CARDOSO".to_owned()),
                    method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                }],
            }
        );

        let requested_paths: Vec<String> = mock_server
            .received_requests()
            .await
            .expect("requests should have been recorded")
            .iter()
            .map(|request| request.url.path().to_owned())
            .collect();
        assert_eq!(
            requested_paths,
            ["/licoes/index/500132"],
            "both lesson kinds come from one fetch, without a dashboard warm-up"
        );
    });
}

#[test]
fn given_page_with_no_lessons_should_return_empty_bundle() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/999999"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html><body></body></html>")
                    .insert_header("Content-Type", "text/html"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentLessonsGatewaySamImpl =
            build_gateway(&mock_server).expect("gateway should be built");

        let result: StudentLessons = gateway
            .get_all_for_student_with_id(&StudentId::new("999999".to_owned()))
            .expect("Lessons retrieval should succeed");

        assert_eq!(result, StudentLessons::default());
    });
}

#[test]
fn given_an_unexpected_status_the_failure_names_it() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let gateway: StudentLessonsGatewaySamImpl =
            build_gateway(&mock_server).expect("gateway should be built");

        let (kind, details) =
            failure_of(gateway.get_all_for_student_with_id(&StudentId::new("500132".to_owned())))
                .expect("lessons retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(details.contains("500"), "got: {details}");
    });
}

#[test]
fn given_sam_redirects_away_from_the_lessons_page_the_session_has_expired() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(
                ResponseTemplate::new(307).insert_header("location", mock_server.uri().as_str()),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentLessonsGatewaySamImpl =
            build_gateway(&mock_server).expect("gateway should be built");

        let (kind, _) =
            failure_of(gateway.get_all_for_student_with_id(&StudentId::new("500132".to_owned())))
                .expect("lessons retrieval should have failed");

        assert_eq!(kind, FailureKind::SessionExpired);
    });
}

#[test]
fn given_an_unreachable_site_the_failure_is_a_network_error_naming_the_operation() {
    let gateway: StudentLessonsGatewaySamImpl =
        build_gateway_for(UNREACHABLE_SITE).expect("gateway should be built");

    let (kind, details) =
        failure_of(gateway.get_all_for_student_with_id(&StudentId::new("500132".to_owned())))
            .expect("lessons retrieval should have failed");

    assert_eq!(kind, FailureKind::Transient);
    assert!(details.contains("student_lessons"), "got: {details}");
}

#[test]
fn given_a_truncated_response_the_failure_is_a_network_error_about_decoding() {
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

    let gateway: StudentLessonsGatewaySamImpl =
        build_gateway_for(&format!("http://{addr}")).expect("gateway should be built");

    let (kind, details) =
        failure_of(gateway.get_all_for_student_with_id(&StudentId::new("500132".to_owned())))
            .expect("lessons retrieval should have failed");

    assert_eq!(kind, FailureKind::Transient);
    assert!(details.contains("decode"), "got: {details}");

    server.join().expect("server thread should not panic");
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
            lessons_served(body),
            Some(StudentLessons::default()),
            "page {body:?}"
        );
    }
}

#[test]
fn a_page_without_the_method_table_still_lists_the_msa_lessons() {
    let lessons: Option<StudentLessons> = lessons_served(&lessons_page(
        &msa_table(r#"<tr id="msa_1"><td>01/01/2024</td></tr>"#),
        "<!-- no datatable3 on this page -->",
    ));

    assert_eq!(
        lessons,
        Some(StudentLessons {
            msa: vec![Lesson {
                id: Some("1".to_owned()),
                date: NaiveDate::from_ymd_opt(2024, 1, 1),
                ..Lesson::default()
            }],
            method: Vec::new(),
        })
    );
}

#[test]
fn every_msa_row_is_read_with_all_its_fields() {
    let lessons: Option<Vec<Lesson>> = msa_lessons_listed(
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
    );

    assert_eq!(
        lessons,
        Some(vec![
            Lesson {
                id: Some("538784".to_owned()),
                date: NaiveDate::from_ymd_opt(2025, 8, 19),
                phase: Some(Range::new("3.4".to_owned(), "4.1".to_owned())),
                page: Some(Range::new("30".to_owned(), "34".to_owned())),
                description: Some(
                    "Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32.".to_owned()
                ),
                instructor: Some("ELIAS BRANDE".to_owned()),
                ..Lesson::default()
            },
            Lesson {
                id: Some("559783".to_owned()),
                date: NaiveDate::from_ymd_opt(2025, 9, 9),
                phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
                page: Some(Range::new("38".to_owned(), "38".to_owned())),
                lesson: Some(Range::new("7".to_owned(), "8".to_owned())),
                clef: Some(Clef::G),
                description: Some("Passou lições 7 e 8, estudar próximas lições.".to_owned()),
                instructor: Some("MARCOS ROGÉRIO COSME".to_owned()),
                method: None,
            },
        ])
    );
}

#[test]
fn an_msa_row_without_any_field_is_still_read() {
    let lessons: Option<Vec<Lesson>> = msa_lessons_listed(
        r#"<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr><tr id="msa_1"></tr>"#,
    );

    assert_eq!(
        lessons,
        Some(vec![
            Lesson::default(),
            Lesson {
                id: Some("1".to_owned()),
                ..Lesson::default()
            }
        ]),
        "rows without an authorizer or any other field must not be dropped"
    );
}

#[test]
fn a_cell_with_inline_markup_is_read_as_its_text() {
    let lessons: Option<Vec<Lesson>> = msa_lessons_listed(
        "<tr><td></td><td></td><td></td><td></td><td></td><td>  Revisão: <b>Ligaduras</b>. Estudar.  </td><td></td></tr>",
    );

    assert_eq!(
        lessons,
        Some(vec![Lesson {
            description: Some("Revisão:  Ligaduras . Estudar.".to_owned()),
            ..Lesson::default()
        }])
    );
}

#[test]
fn an_empty_msa_table_has_no_lessons() {
    assert_eq!(msa_lessons_listed(""), Some(Vec::new()));
}

#[test]
fn every_method_row_is_read_with_all_its_fields() {
    let lessons: Option<Vec<Lesson>> = method_lessons_listed(
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
    );

    assert_eq!(
        lessons,
        Some(vec![
            Lesson {
                id: Some("738654".to_owned()),
                date: NaiveDate::from_ymd_opt(2026, 3, 24),
                page: Some(Range::single("00".to_owned())),
                lesson: Some(Range::single("00".to_owned())),
                description: Some("Revisão para RJM".to_owned()),
                instructor: Some("MURILO FAGNER CARDOSO".to_owned()),
                method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                ..Lesson::default()
            },
            Lesson {
                id: Some("190204".to_owned()),
                date: NaiveDate::from_ymd_opt(2023, 10, 23),
                page: Some(Range::single("11".to_owned())),
                lesson: Some(Range::single("7".to_owned())),
                instructor: Some("MURILO FAGNER CARDOSO".to_owned()),
                method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned()),
                ..Lesson::default()
            },
        ])
    );
}

#[test]
fn a_method_row_without_any_field_is_still_read() {
    assert_eq!(
        method_lessons_listed(
            "<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr>"
        ),
        Some(vec![Lesson::default()])
    );
}

#[test]
fn an_empty_method_table_has_no_lessons() {
    assert_eq!(method_lessons_listed(""), Some(Vec::new()));
}

fn msa_lessons_listed(rows_html: &str) -> Option<Vec<Lesson>> {
    lessons_served(&lessons_page(&msa_table(rows_html), "")).map(|lessons| lessons.msa)
}

fn method_lessons_listed(rows_html: &str) -> Option<Vec<Lesson>> {
    lessons_served(&lessons_page("", &method_table(rows_html))).map(|lessons| lessons.method)
}

fn lessons_served(body: &str) -> Option<StudentLessons> {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string(body)
                    .insert_header("Content-Type", "text/html"),
            )
            .mount(&mock_server)
            .await;

        build_gateway(&mock_server)
            .ok()?
            .get_all_for_student_with_id(&StudentId::new("500132".to_owned()))
            .ok()
    })
}

fn lessons_page(msa_table_html: &str, method_table_html: &str) -> String {
    format!("<html><body>{msa_table_html}{method_table_html}</body></html>")
}

fn msa_table(rows_html: &str) -> String {
    format!(
        r#"<div id="msa"><table id="datatable1" class="table table-striped dataTable no-footer" role="grid">
    <thead><tr><th>Data da Lição</th><th>Fases</th><th>Paginas</th><th>Lições</th><th>Claves</th><th>Observações</th><th>Autorizante</th><th>Ações</th></tr></thead>
    <tbody>{rows_html}</tbody>
</table></div>"#
    )
}

fn method_table(rows_html: &str) -> String {
    format!(
        r#"<table id="datatable3" class="table table-striped table-bordered table-hover dataTable no-footer" role="grid">
    <thead><tr><th>Páginas</th><th>Lição</th><th>Método</th><th>Data da Lição</th><th>Autorizante</th><th>Data de Cadastro</th><th>Observações</th><th>Ações</th></tr></thead>
    <tbody>{rows_html}</tbody>
</table>"#
    )
}

fn build_gateway(mock_server: &MockServer) -> Result<StudentLessonsGatewaySamImpl, reqwest::Error> {
    build_gateway_for(&mock_server.uri())
}

fn build_gateway_for(base_url: &str) -> Result<StudentLessonsGatewaySamImpl, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(base_url)?;

    let client: Arc<dyn SamClient> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(StudentLessonsGatewaySamImpl::new(client))
}

fn failure_of(
    result: Result<StudentLessons, StudentLessonsGatewayError>,
) -> Option<(FailureKind, String)> {
    match result {
        Err(StudentLessonsGatewayError::UnableToPerformOperation { kind, details }) => {
            Some((kind, details))
        }
        Ok(_) => None,
    }
}

fn student_lessons_page() -> String {
    r#"<html><body>
<div id="msa"><table id="datatable1"><tbody>
<tr id="msa_559783" role="row" class="even">
    <td>09/09/2025</td>
    <td>4.5 - 4.5</td>
    <td>38 - 38</td>
    <td>7 - 8</td>
    <td>Sol</td>
    <td>Passou lições 7 e 8, estudar próximas lições.</td>
    <td>MARCOS ROGÉRIO COSME</td>
</tr>
</tbody></table></div>
<table id="datatable3"><tbody>
<tr id="mtd_214020" role="row" class="even">
    <td>00</td>
    <td>00</td>
    <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
    <td>04/12/2023</td>
    <td>MURILO FAGNER CARDOSO</td>
    <td>04/12/2023 21:17:17</td>
    <td>Postura do violino </td>
</tr>
</tbody></table>
</body></html>"#
        .to_string()
}

#[test]
fn a_date_is_read_day_first() {
    let lesson: Lesson = msa_lesson_with_date("09/09/2025").unwrap();

    assert_eq!(lesson.date, NaiveDate::from_ymd_opt(2025, 9, 9));
}

#[test]
fn an_unreadable_date_is_left_out() {
    let lesson: Lesson = msa_lesson_with_date("not-a-date").unwrap();

    assert_eq!(lesson.date, None);
}

#[test]
fn a_range_is_read_from_its_two_ends() {
    let lesson: Lesson = msa_lesson_with_phases("7 - 8").unwrap();

    assert_eq!(
        lesson.phase,
        Some(Range::new("7".to_owned(), "8".to_owned()))
    );
}

#[test]
fn a_single_value_is_a_range_that_starts_and_ends_there() {
    let lesson: Lesson = msa_lesson_with_phases("00").unwrap();

    assert_eq!(
        lesson.phase,
        Some(Range::new("00".to_owned(), "00".to_owned()))
    );
}

#[test]
fn a_blank_range_is_left_out() {
    for blank in ["", "   "] {
        assert_eq!(msa_lesson_with_phases(blank).unwrap().phase, None);
    }
}

#[test]
fn every_clef_sam_writes_is_recognized() {
    for (raw, clef) in [
        ("Sol", Clef::G),
        ("Dó", Clef::C),
        ("Do", Clef::C),
        ("Fá", Clef::F),
        ("Fa", Clef::F),
    ] {
        assert_eq!(
            msa_lesson_with_clef(raw).unwrap().clef,
            Some(clef),
            "clef {raw:?}"
        );
    }
}

#[test]
fn an_unknown_clef_is_left_out() {
    assert_eq!(msa_lesson_with_clef("Xyz").unwrap().clef, None);
}

fn msa_lesson_read_from(row: MsaLesson) -> Option<Lesson> {
    let gateway: StudentLessonsGatewaySamImpl = StudentLessonsGatewaySamImpl::new(Arc::new(
        FakeSamClient::showing_lessons(StudentLessonsPage {
            msa: vec![row],
            method: Vec::new(),
        }),
    ));

    gateway
        .get_all_for_student_with_id(&StudentId::new("500132".to_owned()))
        .ok()?
        .msa
        .pop()
}

fn msa_lesson_with_date(date: &str) -> Option<Lesson> {
    msa_lesson_read_from(MsaLesson {
        date: Some(date.to_owned()),
        ..MsaLesson::default()
    })
}

fn msa_lesson_with_phases(phases: &str) -> Option<Lesson> {
    msa_lesson_read_from(MsaLesson {
        phases: Some(phases.to_owned()),
        ..MsaLesson::default()
    })
}

fn msa_lesson_with_clef(clefs: &str) -> Option<Lesson> {
    msa_lesson_read_from(MsaLesson {
        clefs: Some(clefs.to_owned()),
        ..MsaLesson::default()
    })
}
