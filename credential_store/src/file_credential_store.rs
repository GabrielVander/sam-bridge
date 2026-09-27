use authentication::{
    application::gateways::{
        ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
        SaveCredentialGateway, SaveCredentialGatewayError,
    },
    domain::entities::{Credential, Email, Password},
};
use std::path::{Path, PathBuf};
use zeroize::{Zeroize, ZeroizeOnDrop, Zeroizing};

#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[error("no platform data directory is available to store credentials")]
pub struct NoDataDirectory;

pub struct FileCredentialStore {
    dir: PathBuf,
    credential_path: PathBuf,
    key_path: PathBuf,
}

impl FileCredentialStore {
    pub fn new() -> Result<Self, NoDataDirectory> {
        Self::in_platform_data_dir(dirs::data_local_dir())
    }

    pub fn in_platform_data_dir(
        platform_data_dir: Option<PathBuf>,
    ) -> Result<Self, NoDataDirectory> {
        platform_data_dir
            .map(|base| Self::under(&base))
            .ok_or(NoDataDirectory)
    }

    #[must_use]
    pub fn under(base: &Path) -> Self {
        let data_dir: PathBuf = base.join("sam_bridge");

        let _ = std::fs::create_dir_all(&data_dir);
        restrict_permissions(&data_dir, 0o700);

        Self::with_dir(&data_dir.to_string_lossy())
    }

    #[must_use]
    pub fn with_dir(dir: &str) -> Self {
        let dir: PathBuf = Path::new(dir).to_path_buf();

        Self {
            credential_path: dir.join("session.enc"),
            key_path: dir.join("key.bin"),
            dir,
        }
    }
}

impl SaveCredentialGateway for FileCredentialStore {
    fn save(&self, credential: &Credential) -> Result<(), SaveCredentialGatewayError> {
        let stored: StoredCredential = StoredCredential {
            email: credential.email().as_str().to_owned(),
            password: credential.password().as_str().to_owned(),
        };

        self.save_sync(&stored)
            .map_err(|_| SaveCredentialGatewayError::UnableToPerformOperation)
    }
}

impl LoadCredentialGateway for FileCredentialStore {
    fn load(&self) -> Option<Credential> {
        self.load_sync().map(|stored| {
            Credential::new(
                Email::new(stored.email.clone()),
                Password::new(stored.password.clone()),
            )
        })
    }
}

impl ClearCredentialGateway for FileCredentialStore {
    fn clear(&self) -> Result<(), ClearCredentialGatewayError> {
        self.clear_sync().map_err(
            |error| ClearCredentialGatewayError::UnableToPerformOperation {
                details: format!("Unable to remove the credential file: {error}"),
            },
        )
    }
}

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize, Zeroize, ZeroizeOnDrop)]
struct StoredCredential {
    email: String,
    password: String,
}

impl FileCredentialStore {
    fn save_sync(&self, credential: &StoredCredential) -> anyhow::Result<()> {
        let json: Zeroizing<Vec<u8>> = Zeroizing::new(serde_json::to_vec(credential)?);
        let ciphertext: Vec<u8> = self.encrypt(&json)?;

        self.write_private_file(&self.credential_path, ".session.enc.tmp", &ciphertext)
    }

    fn encrypt(&self, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let key: [u8; 32] = self.load_or_create_key()?;

        let mut cocoon = cocoon::Cocoon::new(&key);
        cocoon.wrap(plaintext).map_err(encryption_failed)
    }

    fn load_or_create_key(&self) -> anyhow::Result<[u8; 32]> {
        if let Some(key) = self.read_key() {
            return Ok(key);
        }

        let mut key: [u8; 32] = [0u8; 32];
        getrandom::fill(&mut key).map_err(key_generation_failed)?;

        self.write_private_file(&self.key_path, ".key.bin.tmp", &key)?;

        Ok(key)
    }

    fn read_key(&self) -> Option<[u8; 32]> {
        std::fs::read(&self.key_path).ok()?.try_into().ok()
    }

    fn write_private_file(&self, path: &Path, tmp_name: &str, data: &[u8]) -> anyhow::Result<()> {
        let _ = std::fs::create_dir_all(&self.dir);

        let tmp: PathBuf = self.dir.join(tmp_name);

        std::fs::write(&tmp, data)?;
        restrict_permissions(&tmp, 0o600);

        std::fs::rename(&tmp, path)?;
        restrict_permissions(path, 0o600);

        Ok(())
    }

    fn load_sync(&self) -> Option<StoredCredential> {
        let ciphertext: Vec<u8> = std::fs::read(&self.credential_path).ok()?;
        let plaintext: Zeroizing<Vec<u8>> = Zeroizing::new(self.decrypt(&ciphertext)?);

        serde_json::from_slice::<StoredCredential>(&plaintext).ok()
    }

    fn decrypt(&self, ciphertext: &[u8]) -> Option<Vec<u8>> {
        let key: [u8; 32] = self.read_key()?;

        let cocoon = cocoon::Cocoon::new(&key);
        cocoon.unwrap(ciphertext).ok()
    }

    fn clear_sync(&self) -> std::io::Result<()> {
        match std::fs::remove_file(&self.credential_path) {
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            result => result,
        }
    }
}

#[cfg(unix)]
fn restrict_permissions(path: &Path, mode: u32) {
    let _ = std::fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(mode));
}

#[cfg(not(unix))]
const fn restrict_permissions(_path: &Path, _mode: u32) {}

fn key_generation_failed(error: getrandom::Error) -> anyhow::Error {
    anyhow::anyhow!("Failed to generate encryption key: {error}")
}

#[allow(clippy::needless_pass_by_value)] // `map_err` hands the error over by value
fn encryption_failed(error: cocoon::Error) -> anyhow::Error {
    anyhow::anyhow!("Failed to encrypt credential file: {error:?}")
}
