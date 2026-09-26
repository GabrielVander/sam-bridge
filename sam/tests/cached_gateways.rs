use std::sync::Arc;
use std::time::Duration;
use student::domain::entities::StudentId;

use sam::client::{CacheTtl, SamClient, SamClientCacheDecorator, SamClientImpl, SystemClock};
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::{
    MusicianProfileGatewaySamImpl, StudentLessonsGatewaySamImpl,
};
use sam::roster::adapters::gateways::StudentGatewaySamImpl;
use student::application::gateways::{
    MusicianProfileGateway, StudentGateway, StudentLessonsGateway,
};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

mod support;
use support::sam_operations_for;

#[test]
fn the_student_and_musician_profile_gateways_share_a_single_listing_fetch() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_dashboard_and_listing(&mock_server).await;

        let client = client_with_cache_decorator(&mock_server).expect("client should be built");
        let students = StudentGatewaySamImpl::new(client.clone());
        let profiles = MusicianProfileGatewaySamImpl::new(client);

        students
            .get_available_records()
            .expect("listing should succeed");
        profiles
            .get_by_id(&StudentId::new("1".to_owned()))
            .expect("profile should be found");
        students
            .get_available_records()
            .expect("listing should succeed");

        assert_eq!(requests_to(&mock_server, "/alunos/listagem").await, 1);
        assert_eq!(requests_to(&mock_server, "/painel").await, 1);
    });
}

#[test]
fn repeated_lessons_requests_for_the_same_student_hit_the_site_once() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/licoes/index/500132"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<html><body></body></html>")
                    .insert_header("Content-Type", "text/html"),
            )
            .mount(&mock_server)
            .await;

        let lessons = StudentLessonsGatewaySamImpl::new(
            client_with_cache_decorator(&mock_server).expect("client should be built"),
        );

        lessons
            .get_all_for_student_with_id(&StudentId::new("500132".to_owned()))
            .expect("lessons should be retrieved");
        lessons
            .get_all_for_student_with_id(&StudentId::new("500132".to_owned()))
            .expect("lessons should be retrieved");

        assert_eq!(requests_to(&mock_server, "/licoes/index/500132").await, 1);
    });
}

const LISTING_ROW: &str =
    r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","MÚSICO","VIOLINO","RJM","1","0"]"#;

fn client_with_cache_decorator(
    mock_server: &MockServer,
) -> Result<Arc<dyn SamClient>, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(&mock_server.uri())?;

    Ok(Arc::new(SamClientCacheDecorator::new(
        Arc::new(SamClientImpl::new(sam_operations)),
        Arc::new(SystemClock),
        CacheTtl {
            students: Duration::from_secs(300),
            lessons: Duration::from_secs(300),
        },
    )))
}

async fn mount_dashboard_and_listing(mock_server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/painel"))
        .respond_with(ResponseTemplate::new(200))
        .mount(mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/alunos/listagem"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_string(format!(
                    r#"{{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[{LISTING_ROW}]}}"#
                ))
                .insert_header("Content-Type", "application/json"),
        )
        .mount(mock_server)
        .await;
}

async fn requests_to(mock_server: &MockServer, request_path: &str) -> usize {
    mock_server
        .received_requests()
        .await
        .unwrap_or_default()
        .iter()
        .filter(|request| request.url.path() == request_path)
        .count()
}
