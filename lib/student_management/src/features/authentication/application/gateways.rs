use async_trait::async_trait;

#[async_trait]
pub trait AuthGateway {
    async fn login(&self, username: String, password: String) -> anyhow::Result<()>;
}
