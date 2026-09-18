use std::sync::Arc;

use sam::client::SamClientImpl;
use sam::http::SamOperations;
use sam::lessons::adapters::gateways::MusicianProfileGatewaySamImpl;
use student::application::gateways::{MusicianProfileGateway, MusicianProfileGatewayError};
use student::domain::entities::{Instrument, MusicianLevel, MusicianProfile};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_gateway(
    mock_server: &MockServer,
) -> Result<MusicianProfileGatewaySamImpl, reqwest::Error> {
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
        "licoes/index",
    );

    let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));

    Ok(MusicianProfileGatewaySamImpl::new(sam_client))
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

        let profile: MusicianProfile = gateway.get_by_id("1").await.expect("should succeed");

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

        let profile: MusicianProfile = gateway.get_by_id("1").await.expect("should succeed");

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

        let profile: MusicianProfile = gateway.get_by_id("1").await.expect("should succeed");

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
            gateway.get_by_id("does-not-exist").await;

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

        let result: Result<MusicianProfile, MusicianProfileGatewayError> =
            gateway.get_by_id("1").await;

        assert_eq!(result, Err(MusicianProfileGatewayError::NotAMusician));
    });
}

#[test]
fn client_failure_is_reported_as_unable_to_perform_operation() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/painel"))
            .respond_with(ResponseTemplate::new(307))
            .mount(&mock_server)
            .await;

        let gateway: MusicianProfileGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: Result<MusicianProfile, MusicianProfileGatewayError> =
            gateway.get_by_id("1").await;

        assert_eq!(
            result,
            Err(MusicianProfileGatewayError::UnableToPerformOperation)
        );
    });
}
