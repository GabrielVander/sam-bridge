use std::sync::Arc;

use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError, FailureKind,
};
use authentication::domain::entities::{Credential, Email, Password};
use sam::authentication::adapters::gateways::CredentialGatewaySamImpl;
use sam::client::SamClientImpl;
use sam::http::SamOperations;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn build_gateway(mock_server: &MockServer) -> Result<CredentialGatewaySamImpl, reqwest::Error> {
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

    Ok(CredentialGatewaySamImpl::new(sam_client))
}

fn failure_of(
    result: Result<AuthorizationResult, CredentialGatewayError>,
) -> Option<(FailureKind, String)> {
    match result {
        Err(CredentialGatewayError::UnableToPerformOperation { kind, details }) => {
            Some((kind, details))
        }
        Ok(_) => None,
    }
}

fn credential() -> Credential {
    Credential::new(
        Email("someone@example.com".to_owned()),
        Password("hunter2".to_owned()),
    )
}

#[test]
fn given_a_303_response_authorization_succeeds() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/autenticar"))
            .respond_with(ResponseTemplate::new(303))
            .mount(&mock_server)
            .await;

        let gateway: CredentialGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: AuthorizationResult = gateway.authorize(&credential()).expect("should succeed");

        assert_eq!(result, AuthorizationResult::Authorized);
    });
}

#[test]
fn given_the_invalid_credentials_marker_authorization_is_unauthorized() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/autenticar"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_string("<p>* Oops... O usuário ou senha incorretos!</p>"),
            )
            .mount(&mock_server)
            .await;

        let gateway: CredentialGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let result: AuthorizationResult = gateway.authorize(&credential()).expect("should succeed");

        assert_eq!(result, AuthorizationResult::Unauthorized);
    });
}

#[test]
fn given_an_unexpected_response_authorization_fails() {
    smol::block_on(async {
        let mock_server: MockServer = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/autenticar"))
            .respond_with(ResponseTemplate::new(200).set_body_string("<html></html>"))
            .mount(&mock_server)
            .await;

        let gateway: CredentialGatewaySamImpl =
            build_gateway(&mock_server).expect("client should be built");

        let (kind, details) =
            failure_of(gateway.authorize(&credential())).expect("authorization should have failed");

        assert_eq!(kind, FailureKind::UnexpectedResponse);
        assert!(
            details.contains("Unexpected authentication response"),
            "got: {details}"
        );
    });
}

#[test]
fn given_a_connection_failure_authorization_fails() {
    smol::block_on(async {
        let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .build()
            .expect("client should be built");

        let sam_operations: SamOperations = SamOperations::new(
            client,
            "http://127.0.0.1:1",
            "autenticar",
            "painel",
            "alunos/listagem",
            "licoes/index",
        );
        let sam_client: Arc<SamClientImpl> = Arc::new(SamClientImpl::new(sam_operations));
        let gateway: CredentialGatewaySamImpl = CredentialGatewaySamImpl::new(sam_client);

        let (kind, details) =
            failure_of(gateway.authorize(&credential())).expect("authorization should have failed");

        assert_eq!(kind, FailureKind::Network);
        assert!(details.contains("authentication"), "got: {details}");
        assert!(
            !details.contains("hunter2"),
            "the password must never appear in diagnostics, got: {details}"
        );
        assert!(
            !details.contains("someone@example.com"),
            "the email must never appear in diagnostics, got: {details}"
        );
    });
}
