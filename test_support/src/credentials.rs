//! Fakes for the `authentication` ports.

use std::sync::{Mutex, PoisonError};

use authentication::application::gateways::{
    AuthorizationResult, CredentialGateway, CredentialGatewayError, CredentialStore,
    CredentialStoreError,
};
use authentication::domain::entities::{Credential, Email, Password};

/// A credential store that keeps its credential in memory.
///
/// It remembers, replaces and forgets exactly like the real store does, which
/// the contract suite checks, so tests can read back what a use case saved.
#[derive(Default)]
pub struct InMemoryCredentialStore {
    saved: Mutex<Option<(String, String)>>,
}

impl InMemoryCredentialStore {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            saved: Mutex::new(None),
        }
    }

    /// A store that already remembers a credential, as after an earlier login.
    #[must_use]
    pub const fn holding(email: String, password: String) -> Self {
        Self {
            saved: Mutex::new(Some((email, password))),
        }
    }
}

impl CredentialStore for InMemoryCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError> {
        *self.saved.lock().unwrap_or_else(PoisonError::into_inner) =
            Some((credential.email.0.clone(), credential.password.0.clone()));
        Ok(())
    }

    fn load(&self) -> Option<Credential> {
        self.saved
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
            .map(|(email, password)| Credential::new(Email(email), Password(password)))
    }

    fn clear(&self) -> Result<(), CredentialStoreError> {
        *self.saved.lock().unwrap_or_else(PoisonError::into_inner) = None;
        Ok(())
    }
}

/// A credential store whose storage is broken: nothing can be saved.
pub struct FailingCredentialStore;

impl CredentialStore for FailingCredentialStore {
    fn save(&self, _: &Credential) -> Result<(), CredentialStoreError> {
        Err(CredentialStoreError::UnableToPerformOperation)
    }

    fn load(&self) -> Option<Credential> {
        None
    }

    fn clear(&self) -> Result<(), CredentialStoreError> {
        Ok(())
    }
}

/// A gateway that answers every authorization with a fixed result.
pub struct FakeCredentialGateway {
    result: Result<AuthorizationResult, CredentialGatewayError>,
}

impl FakeCredentialGateway {
    #[must_use]
    pub const fn new(result: Result<AuthorizationResult, CredentialGatewayError>) -> Self {
        Self { result }
    }

    #[must_use]
    pub const fn authorizing() -> Self {
        Self::new(Ok(AuthorizationResult::Authorized))
    }

    #[must_use]
    pub const fn rejecting() -> Self {
        Self::new(Ok(AuthorizationResult::Unauthorized))
    }
}

impl CredentialGateway for FakeCredentialGateway {
    fn authorize(&self, _: &Credential) -> Result<AuthorizationResult, CredentialGatewayError> {
        self.result.clone()
    }
}

/// A gateway that authorizes everything and remembers what it was asked about.
#[derive(Default)]
pub struct RecordingCredentialGateway {
    received: Mutex<Option<(String, String)>>,
}

impl RecordingCredentialGateway {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            received: Mutex::new(None),
        }
    }

    /// The email and password of the last credential it was asked to authorize.
    #[must_use]
    pub fn last_received(&self) -> Option<(String, String)> {
        self.received
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl CredentialGateway for RecordingCredentialGateway {
    fn authorize(
        &self,
        credential: &Credential,
    ) -> Result<AuthorizationResult, CredentialGatewayError> {
        *self.received.lock().unwrap_or_else(PoisonError::into_inner) =
            Some((credential.email.0.clone(), credential.password.0.clone()));
        Ok(AuthorizationResult::Authorized)
    }
}
