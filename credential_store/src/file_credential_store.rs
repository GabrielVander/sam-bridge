use std::path::{Path, PathBuf};

use async_trait::async_trait;
use authentication::{
    application::gateways::{CredentialStore, CredentialStoreError},
    domain::entities::{Credential, Email, Password},
};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

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
    dir: PathBuf,
    credential_path: PathBuf,
    key_path: PathBuf,
}

impl FileCredentialStore {
    #[must_use]
    #[allow(clippy::new_without_default)]
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
        let dir = Path::new(dir).to_path_buf();
        Self {
            credential_path: dir.join("session.enc"),
            key_path: dir.join("key.bin"),
            dir,
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

        let _ = std::fs::create_dir_all(&self.dir);
        let tmp = self.dir.join(".key.bin.tmp");
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

    fn atomic_write(&self, data: &[u8]) -> anyhow::Result<()> {
        let _ = std::fs::create_dir_all(&self.dir);
        let tmp = self.dir.join(".session.enc.tmp");
        std::fs::write(&tmp, data)?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        std::fs::rename(&tmp, &self.credential_path)?;
        #[cfg(unix)]
        {
            let _ = std::fs::set_permissions(
                &self.credential_path,
                std::os::unix::fs::PermissionsExt::from_mode(0o600),
            );
        }
        Ok(())
    }

    fn save_sync(&self, credential: &StoredCredential) -> anyhow::Result<()> {
        let json: Zeroizing<Vec<u8>> = Zeroizing::new(serde_json::to_vec(credential)?);
        let ciphertext: Vec<u8> = self.encrypt(&json)?;
        self.atomic_write(&ciphertext)
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

#[cfg(test)]
mod tests {
    use super::StoredCredential;

    #[test]
    fn debug_formatting_redacts_the_password() {
        let stored = StoredCredential {
            email: "someone@example.com".to_owned(),
            password: "super-secret".to_owned(),
        };

        let formatted = format!("{stored:?}");

        assert!(formatted.contains("someone@example.com"));
        assert!(!formatted.contains("super-secret"));
    }
}
