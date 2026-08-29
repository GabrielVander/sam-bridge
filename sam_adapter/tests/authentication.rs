use std::sync::Arc;

use authentication::{
    application::gateways::{AuthorizationResult, CredentialGateway},
    domain::entities::{Credential, Email, Password},
};
use sam::client::{SamClient, SamCredentials, SamStudent, StudentLessonsPage};
use sam_adapter::adapters::gateways::CredentialGatewaySamImpl;

#[test]
fn success_flow() {
    let credential: Credential = Credential::new(
        Email("some@email.com".to_string()),
        Password("secretP@ssw0rd".to_string()),
    );

    let sam_client: Arc<FakeSamClient> = Arc::new(FakeSamClient::new(Ok(())));
    let gateway: CredentialGatewaySamImpl = CredentialGatewaySamImpl::new(sam_client.clone());

    let result: Result<AuthorizationResult, String> =
        smol::block_on(async { gateway.authorize(&credential).await });

    assert_eq!(result, Ok(AuthorizationResult::Authorized))
}

#[test]
fn unauthorized_flow() {
    let credential: Credential = Credential::new(
        Email("some@email.com".to_string()),
        Password("secretP@ssw0rd".to_string()),
    );

    let sam_client: Arc<FakeSamClient> =
        Arc::new(FakeSamClient::new(Err("Invalid credentials".to_string())));
    let gateway: CredentialGatewaySamImpl = CredentialGatewaySamImpl::new(sam_client.clone());

    let result: Result<AuthorizationResult, String> =
        smol::block_on(async { gateway.authorize(&credential).await });

    assert_eq!(result, Ok(AuthorizationResult::Unauthorized))
}

#[test]
fn failure() {
    let error_msg: &str = "Some error";
    let credential: Credential = Credential::new(
        Email("some@email.com".to_string()),
        Password("secretP@ssw0rd".to_string()),
    );

    let sam_client: Arc<FakeSamClient> = Arc::new(FakeSamClient::new(Err(error_msg.to_string())));
    let gateway: CredentialGatewaySamImpl = CredentialGatewaySamImpl::new(sam_client.clone());

    let result: Result<AuthorizationResult, String> =
        smol::block_on(async { gateway.authorize(&credential).await });

    assert_eq!(result, Err(error_msg.into()))
}

struct FakeSamClient {
    login_result: Result<(), String>,
}

impl FakeSamClient {
    fn new(login_result: Result<(), String>) -> Self {
        Self { login_result }
    }
}

impl SamClient for FakeSamClient {
    fn login(&self, _credentials: &SamCredentials) -> Result<(), String> {
        self.login_result.clone()
    }

    fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
        todo!()
    }

    fn student_lessons(&self, _student_id: &str) -> anyhow::Result<StudentLessonsPage> {
        todo!()
    }
}
