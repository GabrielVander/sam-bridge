use crate::shared::domain::entities::Credential;

pub trait LoadCredentialGateway: Send + Sync {
    fn load(&self) -> Option<Credential>;
}
