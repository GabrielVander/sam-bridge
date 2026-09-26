use std::sync::{Mutex, MutexGuard, PoisonError};

use authentication::{
    application::gateways::{
        ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
        SaveCredentialGateway, SaveCredentialGatewayError,
    },
    domain::entities::{Credential, Email, Password},
};

#[derive(Default)]
pub struct InMemoryCredentialStore {
    saved: Mutex<Option<(String, String)>>,
}

impl InMemoryCredentialStore {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    fn saved(&self) -> MutexGuard<'_, Option<(String, String)>> {
        self.saved.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn store_credential(&self, credential: &Credential) {
        *self.saved() = Some((credential.email.0.clone(), credential.password.0.clone()));
    }

    fn retrieve_credential(&self) -> Option<Credential> {
        self.saved()
            .clone()
            .map(|(email, password)| Credential::new(Email(email), Password(password)))
    }

    fn clear_credential(&self) {
        *self.saved() = None;
    }
}

impl SaveCredentialGateway for InMemoryCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), SaveCredentialGatewayError> {
        self.store_credential(credential);
        Ok(())
    }
}

impl LoadCredentialGateway for InMemoryCredentialStore {
    fn load(&self) -> Option<Credential> {
        self.retrieve_credential()
    }
}

impl ClearCredentialGateway for InMemoryCredentialStore {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        self.clear_credential();
        Ok(())
    }
}
