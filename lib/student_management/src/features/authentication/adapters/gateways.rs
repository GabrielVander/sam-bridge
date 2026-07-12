use async_trait::async_trait;

use crate::{
    features::authentication::domain::gateways::AuthGateway,
    shared::infra::sam_site_adapter::SamSiteAdapter,
};

#[async_trait]
impl AuthGateway for SamSiteAdapter {
    async fn login(&self, username: String, password: String) -> Result<(), String> {
        self.login(&username, &password)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
