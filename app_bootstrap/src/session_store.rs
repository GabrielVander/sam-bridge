use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoredCredentials {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

impl std::fmt::Debug for StoredCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StoredCredentials")
            .field("base_url", &self.base_url)
            .field("username", &self.username)
            .field("password", &"***")
            .finish()
    }
}

impl Drop for StoredCredentials {
    fn drop(&mut self) {
        for b in unsafe { self.password.as_bytes_mut() } {
            *b = 0;
        }
    }
}

pub trait SessionStore: Send + Sync {
    fn save(&self, credentials: &StoredCredentials) -> Result<()>;
    fn load(&self) -> Option<StoredCredentials>;
    fn clear(&self);
}

pub struct FileSessionStore {
    session_path: PathBuf,
    key_path: PathBuf,
}

impl Default for FileSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSessionStore {
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
        Self::with_dir(data_dir)
    }

    pub fn with_dir(dir: PathBuf) -> Self {
        Self {
            session_path: dir.join("session.enc"),
            key_path: dir.join("key.bin"),
        }
    }

    fn load_or_create_key(&self) -> Result<[u8; 32]> {
        if let Ok(bytes) = std::fs::read(&self.key_path)
            && bytes.len() == 32
        {
            let mut key = [0u8; 32];
            key.copy_from_slice(&bytes);
            return Ok(key);
        }
        let mut key = [0u8; 32];
        getrandom::getrandom(&mut key).context("Failed to generate encryption key")?;
        let dir = self.key_path.parent().unwrap_or(Path::new("."));
        let _ = std::fs::create_dir_all(dir);
        let tmp = dir.join(".key.bin.tmp");
        std::fs::write(&tmp, key).context("Failed to write key file")?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        std::fs::rename(&tmp, &self.key_path).context("Failed to persist key file")?;
        #[cfg(unix)]
        {
            let _ = std::fs::set_permissions(
                &self.key_path,
                std::os::unix::fs::PermissionsExt::from_mode(0o600),
            );
        }
        Ok(key)
    }

    fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let key = self.load_or_create_key()?;
        let mut cocoon = cocoon::Cocoon::new(&key);
        cocoon
            .wrap(plaintext)
            .map_err(|e| anyhow::anyhow!("{e:?}"))
            .context("Failed to encrypt session file")
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

    fn atomic_write(&self, path: &Path, data: &[u8]) -> Result<()> {
        let dir = path.parent().unwrap_or(Path::new("."));
        let _ = std::fs::create_dir_all(dir);
        let tmp = dir.join(format!(
            ".{}.tmp",
            path.file_name().unwrap_or_default().to_string_lossy()
        ));
        std::fs::write(&tmp, data).context("Failed to write session file")?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(&tmp, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        std::fs::rename(&tmp, path).context("Failed to persist session file")?;
        #[cfg(unix)]
        {
            let _ =
                std::fs::set_permissions(path, std::os::unix::fs::PermissionsExt::from_mode(0o600));
        }
        Ok(())
    }
}

impl SessionStore for FileSessionStore {
    fn save(&self, credentials: &StoredCredentials) -> Result<()> {
        let json = serde_json::to_vec(credentials).expect("Serializing String fields cannot fail");
        let ciphertext = self.encrypt(&json)?;
        self.atomic_write(&self.session_path, &ciphertext)
    }

    fn load(&self) -> Option<StoredCredentials> {
        let ciphertext = std::fs::read(&self.session_path).ok()?;
        let plaintext = self.decrypt(&ciphertext).or_else(|| {
            eprintln!("Failed to decrypt session file (tampered or wrong key), ignoring");
            None
        })?;
        serde_json::from_slice::<StoredCredentials>(&plaintext)
            .map_err(|_| eprintln!("Corrupt decrypted session file, ignoring"))
            .ok()
    }

    fn clear(&self) {
        let _ = std::fs::remove_file(&self.session_path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> (FileSessionStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("tempdir");
        let store = FileSessionStore::with_dir(dir.path().to_path_buf());
        (store, dir)
    }

    fn credentials() -> StoredCredentials {
        StoredCredentials {
            base_url: "https://musical.congregacao.org.br".to_owned(),
            username: "test_user".to_owned(),
            password: "test_pass".to_owned(),
        }
    }

    #[test]
    fn default_impl_creates_data_directory() {
        let _store = FileSessionStore::default();
    }

    #[test]
    fn new_creates_data_directory() {
        let store = FileSessionStore::new();
        assert!(store.session_path.ends_with("sam_bridge/session.enc"));
    }

    #[test]
    fn round_trip_preserves_credentials() {
        let (store, _dir) = temp_store();
        let creds = credentials();

        store.save(&creds).expect("save should succeed");
        let loaded = store.load().expect("load should return Some");

        assert_eq!(loaded, creds);
    }

    #[test]
    fn saved_file_is_encrypted_not_plaintext() {
        let (store, _dir) = temp_store();
        store.save(&credentials()).expect("save");
        let raw = std::fs::read(&store.session_path).expect("read enc file");
        let raw_str = String::from_utf8_lossy(&raw);
        assert!(
            !raw_str.contains("test_pass"),
            "encrypted file must not contain plaintext password"
        );
    }

    #[test]
    fn missing_file_loads_as_none() {
        let (store, _dir) = temp_store();
        assert!(store.load().is_none());
    }

    #[test]
    fn clear_removes_the_file() {
        let (store, _dir) = temp_store();
        store.save(&credentials()).expect("save");

        store.clear();

        assert!(store.load().is_none());
        assert!(!store.session_path.exists());
    }

    #[test]
    fn corrupt_file_loads_as_none() {
        let (store, _dir) = temp_store();
        std::fs::write(&store.session_path, b"{invalid json").expect("write corrupt data");
        let _ = store.load_or_create_key();

        assert!(store.load().is_none());
    }

    #[test]
    fn tamper_detection_returns_none() {
        let (store, _dir) = temp_store();
        store.save(&credentials()).expect("save");
        let mut data = std::fs::read(&store.session_path).expect("read");
        if !data.is_empty() {
            data[0] ^= 0xFF;
            std::fs::write(&store.session_path, &data).expect("tamper");
        }
        assert!(store.load().is_none());
    }

    #[cfg(unix)]
    #[test]
    fn files_have_restricted_permissions() {
        use std::os::unix::fs::PermissionsExt;
        let (store, _dir) = temp_store();
        store.save(&credentials()).expect("save");
        let meta = std::fs::metadata(&store.session_path).expect("meta");
        assert_eq!(meta.permissions().mode() & 0o777, 0o600);
        let key_meta = std::fs::metadata(&store.key_path).expect("key meta");
        assert_eq!(key_meta.permissions().mode() & 0o777, 0o600);
    }

    #[test]
    fn load_or_create_key_reuses_existing() {
        let (store, _dir) = temp_store();
        store.save(&credentials()).expect("save");
        let key1 = std::fs::read(&store.key_path).expect("key1");
        store.save(&credentials()).expect("save again");
        let key2 = std::fs::read(&store.key_path).expect("key2");
        assert_eq!(key1, key2);
        assert_eq!(store.load().expect("load").username, "test_user");
    }

    #[test]
    fn truncated_key_is_regenerated() {
        let (store, _dir) = temp_store();
        std::fs::write(&store.key_path, b"short").expect("write short key");
        store.save(&credentials()).expect("save should regenerate key");
        let key = std::fs::read(&store.key_path).expect("key");
        assert_eq!(key.len(), 32);
        assert!(store.load().is_some());
    }

    #[test]
    fn decrypt_with_wrong_key_returns_none() {
        let dir1 = tempfile::tempdir().expect("dir1");
        let store1 = FileSessionStore::with_dir(dir1.path().to_path_buf());
        store1.save(&credentials()).expect("save1");
        let data = std::fs::read(&store1.session_path).expect("data");
        let dir2 = tempfile::tempdir().expect("dir2");
        let store2 = FileSessionStore::with_dir(dir2.path().to_path_buf());
        store2.save(&credentials()).expect("save2 to create different key");
        std::fs::write(&store2.session_path, &data).expect("copy data with wrong key");
        assert!(store2.load().is_none());
    }

    #[test]
    fn clear_is_idempotent() {
        let (store, _dir) = temp_store();
        store.clear();
        store.clear();
        assert!(store.load().is_none());
        store.save(&credentials()).expect("save");
        store.clear();
        store.clear();
        assert!(store.load().is_none());
    }
}
