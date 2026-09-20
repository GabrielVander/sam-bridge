use std::sync::Arc;

use sam::client::SamClientImpl;
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::MusicianProfileGatewaySamImpl;
use student::application::gateways::{
    FailureKind, MusicianProfileGateway, MusicianProfileGatewayError,
};
use student::domain::entities::{Instrument, MusicianLevel, MusicianProfile};
use test_support::sam_site::sam_operations_for;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_gateway(
    mock_server: &MockServer,
) -> Result<MusicianProfileGatewaySamImpl, reqwest::Error> {
    build_gateway_for(&mock_server.uri())
}

fn build_gateway_for(base_url: &str) -> Result<MusicianProfileGatewaySamImpl, reqwest::Error> {
    let sam_operations: SamOperations = sam_operations_for(base_url)?;

    let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(MusicianProfileGatewaySamImpl::new(sam_client))
}

fn failure_of(
    result: Result<MusicianProfile, MusicianProfileGatewayError>,
) -> Option<(FailureKind, String)> {
    match result {
        Err(MusicianProfileGatewayError::UnableToPerformOperation { kind, details }) => {
            Some((kind, details))
        }
        Ok(_) | Err(_) => None,
    }
}

async fn mount_listing(mock_server: &MockServer, row_json: &str) {
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
                    r#"{{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[{row_json}]}}"#
                ))
                .insert_header("Content-Type", "application/json"),
        )
        .mount(mock_server)
        .await;
}

#[test]
fn returns_the_musicians_level_and_instrument() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_listing(
            &mock_server,
            r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","MÚSICO","VIOLINO","RJM","1","0"]"#,
        )
        .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let profile: MusicianProfile = gateway.get_by_id("1").expect("should succeed");

        assert_eq!(profile.level, MusicianLevel::YouthService);
        assert_eq!(profile.instrument, Some(Instrument::Violin));
    });
}

#[test]
fn student_without_an_assigned_instrument_has_none() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_listing(
            &mock_server,
            r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","MÚSICO","A DEFINIR","CANDIDATO(A)","1","0"]"#,
        )
        .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let profile: MusicianProfile = gateway.get_by_id("1").expect("should succeed");

        assert_eq!(profile.instrument, None);
    });
}

#[test]
fn student_with_a_blank_instrument_column_has_none() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_listing(
            &mock_server,
            r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","MÚSICO","  ","CANDIDATO(A)","1","0"]"#,
        )
        .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let profile: MusicianProfile = gateway.get_by_id("1").expect("should succeed");

        assert_eq!(profile.instrument, None);
    });
}

#[test]
fn unknown_id_is_not_found() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_listing(
            &mock_server,
            r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","MÚSICO","VIOLINO","RJM","1","0"]"#,
        )
        .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: Result<MusicianProfile, MusicianProfileGatewayError> =
            gateway.get_by_id("does-not-exist");

        assert_eq!(result, Err(MusicianProfileGatewayError::NotFound));
    });
}

#[test]
fn non_musician_is_reported_as_not_a_musician() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;
        mount_listing(
            &mock_server,
            r#"["1","PEDRO ÁLVARES CABRAL","SOMEWHERE","ORGANISTA","A DEFINIR","RJM","1","0"]"#,
        )
        .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: Result<MusicianProfile, MusicianProfileGatewayError> = gateway.get_by_id("1");

        assert_eq!(result, Err(MusicianProfileGatewayError::NotAMusician));
    });
}

#[test]
fn an_expired_session_is_reported_with_its_kind_and_details() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(307))
            .mount(&mock_server)
            .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) =
            failure_of(gateway.get_by_id("1")).expect("profile retrieval should have failed");

        assert_eq!(kind, FailureKind::SessionExpired);
        assert!(details.contains("Session expired"), "got: {details}");
    });
}

#[test]
fn an_unreachable_site_is_a_network_error_naming_the_operation() {
    // Port 1 is reserved and nothing listens on it, so the connection is refused.
    let gateway: MusicianProfileGatewaySamImpl =
        build_gateway_for("http://127.0.0.1:1").expect("client should be built");

    let (kind, details) =
        failure_of(gateway.get_by_id("1")).expect("profile retrieval should have failed");

    assert_eq!(kind, FailureKind::Network);
    assert!(details.contains("dashboard"), "got: {details}");
}
