use std::sync::Arc;

use chrono::NaiveDate;
use sam::client::{MsaLesson, SamClient, SamClientImpl, StudentLessonsPage};
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::StudentLessonsGatewaySamImpl;
use student::application::gateways::{
    FailureKind, StudentLessonsGateway, StudentLessonsGatewayError,
};
use student::domain::entities::{Clef, Lesson, Range, StudentLessons};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;
use support::{FakeSamClient, sam_operations_for};

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

        let (kind, details) = failure_of(gateway.get_all_for_student_with_id("500132"))
            .expect("lessons retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(details.contains("500"), "got: {details}");
    });
}

#[test]
fn given_an_unreachable_site_the_failure_is_a_network_error_naming_the_operation() {
    // Port 1 is reserved and nothing listens on it, so the connection is refused.
    let gateway: StudentLessonsGatewaySamImpl =
        build_gateway_for("http://127.0.0.1:1").expect("gateway should be built");

    let (kind, details) = failure_of(gateway.get_all_for_student_with_id("500132"))
        .expect("lessons retrieval should have failed");

    assert_eq!(kind, FailureKind::Transient);
    assert!(details.contains("student_lessons"), "got: {details}");
}

fn build_gateway(mock_server: &MockServer) -> Result<StudentLessonsGatewaySamImpl, reqwest::Error> {
    build_gateway_for(&mock_server.uri())
}

fn build_gateway_for(base_url: &str) -> Result<StudentLessonsGatewaySamImpl, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(base_url)?;

    let client: Arc<dyn SamClient + Send + Sync> = Arc::new(SamClientImpl::new(sam_operations));

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
        .get_all_for_student_with_id("500132")
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
