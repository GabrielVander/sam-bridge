use std::sync::Arc;

use chrono::NaiveDate;
use sam::client::{SamClient, SamClientImpl};
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::StudentLessonsGatewaySamImpl;
use student::application::gateways::StudentLessonsGateway;
use student::domain::entities::{Clef, Lesson, Range, StudentLessons};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_gateway(mock_server: &MockServer) -> Result<StudentLessonsGatewaySamImpl, reqwest::Error> {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()?;

    let sam_operations: SamOperations = SamOperations::new(
        http_client,
        &mock_server.uri(),
        "autenticar",
        "painel",
        "alunos/listagem",
        "licoes/index",
    );

    let client: Arc<dyn SamClient + Send + Sync> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(StudentLessonsGatewaySamImpl::new(client))
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
            .get_all_for_student_with_id("500132")
            .expect("Lessons retrieval should succeed");

        assert_eq!(
            result,
            StudentLessons {
                approved: vec![Lesson {
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
            .get_all_for_student_with_id("999999")
            .expect("Lessons retrieval should succeed");

        assert_eq!(result, StudentLessons::default());
    });
}

#[test]
fn given_request_failure_should_propagate_error() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let gateway: StudentLessonsGatewaySamImpl =
            build_gateway(&mock_server).expect("gateway should be built");

        let result = gateway.get_all_for_student_with_id("500132");

        assert!(result.is_err(), "Expected an Err but got {result:#?}");
    });
}
