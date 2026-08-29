use async_trait::async_trait;

#[async_trait]
pub trait AuthGateway: Send + Sync {
    async fn login(&self, username: String, password: String) -> anyhow::Result<()>;
}
