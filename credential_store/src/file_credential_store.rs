use authentication::{
    application::gateways::{
        ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
        SaveCredentialGateway, SaveCredentialGatewayError,
    },
    domain::entities::{Credential, Email, Password},
};
use std::path::{Path, PathBuf};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("no platform data directory is available to store credentials")]
pub struct NoDataDirectory;

fn key_generation_failed(error: getrandom::Error) -> anyhow::Error {
    anyhow::anyhow!("Failed to generate encryption key: {error}")
}

#[allow(clippy::needless_pass_by_value)] // `map_err` hands the error over by value
fn encryption_failed(error: cocoon::Error) -> anyhow::Error {
    anyhow::anyhow!("Failed to encrypt credential file: {error:?}")
}

pub struct FileCredentialStore {
    dir: PathBuf,
    credential_path: PathBuf,
    key_path: PathBuf,
}

impl FileCredentialStore {
    pub fn new() -> Result<Self, NoDataDirectory> {
        Self::in_platform_data_dir(dirs::data_local_dir())
    }

    #[must_use]
    pub fn under(base: &Path) -> Self {
        let data_dir = base.join("sam_bridge");
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

    fn in_platform_data_dir(platform_data_dir: Option<PathBuf>) -> Result<Self, NoDataDirectory> {
        platform_data_dir
            .map(|base| Self::under(&base))
            .ok_or(NoDataDirectory)
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
        getrandom::fill(&mut key).map_err(key_generation_failed)?;

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
        cocoon.wrap(plaintext).map_err(encryption_failed)
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
impl SaveCredentialGateway for FileCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), SaveCredentialGatewayError> {
        let stored = StoredCredential {
            email: credential.email.0.clone(),
            password: credential.password.0.clone(),
        };

        self.save_sync(&stored)
            .map_err(|_| SaveCredentialGatewayError::UnableToPerformOperation)
    }
}

impl LoadCredentialGateway for FileCredentialStore {
    fn load(&self) -> Option<Credential> {
        self.load_sync().map(|stored| {
            Credential::new(
                Email(stored.email.clone()),
                Password(stored.password.clone()),
            )
        })
    }
}

impl ClearCredentialGateway for FileCredentialStore {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        self.clear_sync();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{FileCredentialStore, NoDataDirectory, StoredCredential};

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

    #[test]
    fn without_a_platform_data_dir_there_is_nowhere_to_store_credentials() {
        let result = FileCredentialStore::in_platform_data_dir(None);

        assert_eq!(result.err(), Some(NoDataDirectory));
    }

    #[test]
    fn the_missing_data_dir_error_explains_itself() {
        assert_eq!(
            NoDataDirectory.to_string(),
            "no platform data directory is available to store credentials"
        );
    }
}
