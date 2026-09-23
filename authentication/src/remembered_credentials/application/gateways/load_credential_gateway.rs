use crate::shared::domain::entities::Credential;

pub trait LoadCredentialGateway {
    fn load(&self) -> Option<Credential>;
}
