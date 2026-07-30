use sam_integration::api::infrastructure::SamClient;
use student_management::api::application::AuthGateway;

pub(crate) struct AuthSamGateway<'a> {
    sam_client: &'a SamClient,
}

impl<'a> AuthSamGateway<'a> {
    pub(crate) fn new(sam_client: &'a SamClient) -> Self {
        Self { sam_client }
    }
}

#[async_trait::async_trait]
impl<'a> AuthGateway for AuthSamGateway<'a> {
    async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        self.sam_client
            .login(&username, &password)
            .await
            .map(|_| ())
    }
}
