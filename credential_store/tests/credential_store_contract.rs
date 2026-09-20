//! The file-backed store must behave like every other `CredentialStore`.

use authentication::application::gateways::{CredentialStore, CredentialStoreError};
use authentication::domain::entities::Credential;
use credential_store::FileCredentialStore;

/// A file store in a temporary directory that lives as long as the store.
struct TempFileStore {
    store: FileCredentialStore,
    _dir: tempfile::TempDir,
}

fn temp_file_store() -> std::io::Result<TempFileStore> {
    let dir = tempfile::tempdir()?;
    let store = FileCredentialStore::with_dir(&dir.path().to_string_lossy());
    Ok(TempFileStore { store, _dir: dir })
}

impl CredentialStore for TempFileStore {
    fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError> {
        self.store.save(credential)
    }

    fn load(&self) -> Option<Credential> {
        self.store.load()
    }

    fn clear(&self) -> Result<(), CredentialStoreError> {
        self.store.clear()
    }
}

test_support::credential_store_contract!(temp_file_store().expect("tempdir"));
