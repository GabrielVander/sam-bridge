use std::sync::Arc;

use sam::client::{
    SamClient, SamClientError, SamClientImpl, SamCredentials, SamStudent, StudentLessonsPage,
};
use sam::http::SamOperations;
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::{FailureKind, StudentGateway, StudentGatewayError};
use student::domain::entities::{Instrument, MusicianLevel, Student, StudentPosition};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;
use support::sam_operations_for;

fn build_gateway(mock_server: &MockServer) -> Result<StudentGatewaySamImpl, reqwest::Error> {
    build_gateway_for(&mock_server.uri())
}

fn build_gateway_for(base_url: &str) -> Result<StudentGatewaySamImpl, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(base_url)?;

    let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(StudentGatewaySamImpl::new(sam_client))
}

fn failure_of(result: Result<Vec<Student>, StudentGatewayError>) -> Option<(FailureKind, String)> {
    match result {
        Err(StudentGatewayError::UnableToPerformOperation { kind, details }) => {
            Some((kind, details))
        }
        Ok(_) => None,
    }
}

#[test]
fn given_accessible_dashboard_students_should_be_retrieved_and_mapped() {
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
                ResponseTemplate::new(200)
                    .set_body_string(
                        r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["99998","PEDRO ÁLVARES CABRAL","JARDIM PALMARES DO NORTE <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","VIOLINO","CANDIDATO(A)","99998","0"]]}"#,
                    )
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: Result<Vec<Student>, StudentGatewayError> = gateway.get_available_records();

        let students: Vec<Student> = result.expect("students retrieval should succeed");
        assert_eq!(students.len(), 1);
        assert_eq!(students[0].id, "99998");
        assert_eq!(students[0].name, "PEDRO ÁLVARES CABRAL");
        assert_eq!(
            students[0].location,
            "JARDIM PALMARES DO NORTE | BR-SP-ARARAQUARA-SÃO CARLOS"
        );
        assert_eq!(
            students[0].position,
            StudentPosition::Musician {
                level: MusicianLevel::Candidate,
                instrument: Some(Instrument::Violin),
                instrument_name: Some("VIOLINO".to_owned()),
            }
        );

        let received_requests: Vec<wiremock::Request> = mock_server
            .received_requests()
            .await
            .expect("requests should have been recorded");
        let dashboard_index: usize = received_requests
            .iter()
            .position(|r| r.url.path() == "/painel")
            .expect("dashboard should have been visited");
        let listing_index: usize = received_requests
            .iter()
            .position(|r| r.url.path() == "/alunos/listagem")
            .expect("listing should have been requested");
        assert!(
            dashboard_index < listing_index,
            "dashboard should be visited before the students listing"
        );
    });
}

#[test]
fn given_musicians_without_a_real_instrument_no_instrument_name_is_carried() {
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
                ResponseTemplate::new(200)
                    .set_body_string(
                        r#"{"draw":"1","recordsTotal":3,"recordsFiltered":3,"data":[["1","A","SOMEWHERE","MÚSICO","A DEFINIR","CANDIDATO(A)","1","0"],["2","B","SOMEWHERE","MÚSICO","","CANDIDATO(A)","2","0"],["3","C","SOMEWHERE","ORGANISTA","VIOLINO","RJM","3","0"]]}"#,
                    )
                    .insert_header("Content-Type", "application/json"),
            )
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let students: Vec<Student> = gateway
            .get_available_records()
            .expect("students retrieval should succeed");

        let names: Vec<Option<&str>> = students
            .iter()
            .map(|student| match &student.position {
                StudentPosition::Musician {
                    instrument_name, ..
                } => instrument_name.as_deref(),
                _ => None,
            })
            .collect();
        assert_eq!(names, vec![None, None, None]);
    });
}

#[test]
fn given_inaccessible_dashboard_students_retrieval_should_fail() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(307))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::SessionExpired);
        assert!(details.contains("Session expired"), "got: {details}");
    });
}

#[test]
fn given_unexpected_listing_status_students_retrieval_should_fail() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(details.contains("500"), "got: {details}");
    });
}

#[test]
fn given_a_listing_that_is_not_json_the_details_explain_what_could_not_be_decoded() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        Mock::given(method("GET"))
            .and(path("/alunos/listagem"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<html>maintenance</html>"))
            .mount(&mock_server)
            .await;

        let gateway: StudentGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) = failure_of(gateway.get_available_records())
            .expect("students retrieval should have failed");

        assert_eq!(kind, FailureKind::Unexpected);
        assert!(
            details.contains("Unable to decode student listing JSON response"),
            "got: {details}"
        );
        assert!(
            details.contains("expected value"),
            "the underlying parse error should be kept, got: {details}"
        );
    });
}

#[test]
fn given_an_unreachable_site_the_failure_is_a_network_error_naming_the_operation() {
    // Port 1 is reserved and nothing listens on it, so the connection is refused.
    let gateway: StudentGatewaySamImpl =
        build_gateway_for("http://127.0.0.1:1").expect("client should be built");

    let (kind, details) =
        failure_of(gateway.get_available_records()).expect("students retrieval should have failed");

    assert_eq!(kind, FailureKind::Transient);
    assert!(details.contains("dashboard"), "got: {details}");
}

struct FailingSamClient;

impl SamClient for FailingSamClient {
    fn login(&self, _credentials: &SamCredentials) -> Result<(), SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }

    fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }

    fn student_lessons(&self, _student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
        Err(SamClientError::InvalidCredentials)
    }
}

#[test]
fn a_client_reporting_invalid_credentials_for_a_data_fetch_fails_with_an_unknown_kind() {
    let gateway: StudentGatewaySamImpl = StudentGatewaySamImpl::new(Arc::new(FailingSamClient));

    let (kind, details) =
        failure_of(gateway.get_available_records()).expect("students retrieval should have failed");

    assert_eq!(kind, FailureKind::Unclassified);
    assert!(details.contains("Invalid credentials"), "got: {details}");
}
