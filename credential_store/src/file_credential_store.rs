use std::path::{Path, PathBuf};

use async_trait::async_trait;
use authentication::{
    application::gateways::{CredentialStore, CredentialStoreError},
    domain::entities::{Credential, Email, Password},
};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

// `Zeroize`/`ZeroizeOnDrop` (rather than a hand-rolled `Drop` writing zeros
// into the buffer) matter here: a plain "write zeros then drop" loop is a
// dead store from the optimizer's point of view — nothing observable reads
// the buffer afterward — so LLVM is free to elide it in a release build.
// The `zeroize` crate uses a volatile write specifically to defeat that.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Zeroize, ZeroizeOnDrop)]
struct StoredCredential {
    email: String,
    password: String,
}

impl std::fmt::Debug for StoredCredential {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoredCredential")
            .field("email", &self.email)
            .field("password", &"***")
            .finish()
    }
}

pub struct FileCredentialStore {
    credential_path: PathBuf,
    key_path: PathBuf,
}

impl Default for FileCredentialStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FileCredentialStore {
    #[must_use]
    pub fn new() -> Self {
        let data_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("sam_bridge");
        let _ = std::fs::create_dir_all(&data_dir);
        #[cfg(unix)]
        {
            let _ = std::fs::set_permissions(
                &data_dir,
                std::os::unix::fs::PermissionsExt::from_mode(0o700),
            );
        }
        Self::with_dir(&data_dir.to_string_lossy())
    }

    #[must_use]
    pub fn with_dir(dir: &str) -> Self {
        let dir = Path::new(dir);
        Self {
            credential_path: dir.join("session.enc"),
            key_path: dir.join("key.bin"),
        }
    }

    fn load_or_create_key(&self) -> anyhow::Result<[u8; 32]> {
        if let Ok(bytes) = std::fs::read(&self.key_path)
            && bytes.len() == 32
        {
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            return Ok(key);
        }

        let mut key = [0u8; 32];
        getrandom::getrandom(&mut key)
            .map_err(|e| anyhow::anyhow!("Failed to generate encryption key: {e}"))?;

        let dir = self.key_path.parent().unwrap_or_else(|| Path::new("."));
        let _ = std::fs::create_dir_all(dir);
        let tmp = dir.join(".key.bin.tmp");
        std::fs::write(&tmp, key)?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        std::fs::rename(&tmp, &self.key_path)?;
        #[cfg(unix)]
        {
            let _ = std::fs::set_permissions(
                &self.key_path,
                std::os::unix::fs::PermissionsExt::from_mode(0o600),
            );
        }

        Ok(key)
    }

    fn encrypt(&self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key = self.load_or_create_key()?;
        let mut cocoon = cocoon::Cocoon::new(&key);
        cocoon
            .wrap(plaintext)
            .map_err(|e| anyhow::anyhow!("Failed to encrypt credential file: {e:?}"))
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        let key_bytes = std::fs::read(&self.key_path).ok()?;
        if key_bytes.len() != 32 {
            return None;
        }
        let mut key = [0u8; 32];
        key.copy_from_slice(&key_bytes);
        let cocoon = cocoon::Cocoon::new(&key);
        cocoon.unwrap(ciphertext).ok()
    }

    fn atomic_write(path: &Path, data: &[u8]) -> anyhow::Result<()> {
        let dir = path.parent().unwrap_or_else(|| Path::new("."));
        let _ = std::fs::create_dir_all(dir);
        let tmp = dir.join(format!(
            ".{}.tmp",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        std::fs::write(&tmp, data)?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        std::fs::rename(&tmp, path)?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        Ok(())
    }

    fn save_sync(&self, credential: &StoredCredential) -> anyhow::Result<()> {
        // The JSON encoding is itself a transient plaintext copy of the
        // credential (distinct from `StoredCredential`'s own zeroized
        // buffers), so it gets the same treatment.
        let json: Zeroizing<Vec<u8>> = Zeroizing::new(serde_json::to_vec(credential)?);
        let ciphertext: Vec<u8> = self.encrypt(&json)?;
        Self::atomic_write(&self.credential_path, &ciphertext)
    }

    fn load_sync(&self) -> Option<StoredCredential> {
        let ciphertext: Vec<u8> = std::fs::read(&self.credential_path).ok()?;
        let plaintext: Zeroizing<Vec<u8>> = Zeroizing::new(self.decrypt(&ciphertext)?);
        serde_json::from_slice::<StoredCredential>(&plaintext).ok()
    }

    fn clear_sync(&self) {
        let _ = std::fs::remove_file(&self.credential_path);
    }
}

#[async_trait]
impl CredentialStore for FileCredentialStore {
    async fn save(&self, credential: &Credential) -> Result<(), CredentialStoreError> {
        let stored = StoredCredential {
            email: credential.email.0.clone(),
            password: credential.password.0.clone(),
        };

        self.save_sync(&stored)
            .map_err(|_| CredentialStoreError::UnableToPerformOperation)
    }

    async fn load(&self) -> Option<Credential> {
        self.load_sync().map(|stored| {
            Credential::new(
                Email(stored.email.clone()),
                Password(stored.password.clone()),
            )
        })
    }

    async fn clear(&self) -> Result<(), CredentialStoreError> {
        self.clear_sync();
        Ok(())
    }
}
