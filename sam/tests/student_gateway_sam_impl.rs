use std::sync::Arc;

use sam::client::SamClientImpl;
use sam::http::SamOperations;
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::StudentGateway;
use student::domain::entities::{MusicianLevel, Student, StudentPosition};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_gateway(mock_server: &MockServer) -> Result<StudentGatewaySamImpl, reqwest::Error> {
    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()?;

    let sam_operations: SamOperations = SamOperations::new(
        client,
        &mock_server.uri(),
        "autenticar",
        "painel",
        "alunos/listagem",
    );

    let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(StudentGatewaySamImpl::new(sam_client))
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

        let result: anyhow::Result<Vec<Student>> = gateway.get_available_records().await;

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
                level: MusicianLevel::Candidate
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

        let result: anyhow::Result<Vec<Student>> = gateway.get_available_records().await;

        assert!(
            result.is_err(),
            "expected students retrieval to fail without an accessible dashboard"
        );
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

        let result: anyhow::Result<Vec<Student>> = gateway.get_available_records().await;

        assert!(
            result.is_err(),
            "expected students retrieval to fail on an unexpected listing status"
        );
    });
}
