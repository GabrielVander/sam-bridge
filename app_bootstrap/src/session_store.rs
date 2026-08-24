use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StoredCredentials {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

pub trait SessionStore: Send + Sync {
    fn save(&self, credentials: &StoredCredentials) -> Result<()>;
    fn load(&self) -> Option<StoredCredentials>;
    fn clear(&self);
}

pub struct FileSessionStore {
    path: PathBuf,
}

impl Default for FileSessionStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSessionStore {
    pub fn new() -> Self {
        let data_dir = dirs::data_local_dir().unwrap_or_default().join("sam_bridge");
        std::fs::create_dir_all(&data_dir).ok();
        Self {
            path: data_dir.join("session.json"),
        }
    }

    #[cfg(test)]
    pub(crate) fn with_path(path: PathBuf) -> Self {
        Self { path }
    }
}

impl SessionStore for FileSessionStore {
    fn save(&self, credentials: &StoredCredentials) -> Result<()> {
        let json = serde_json::to_string_pretty(credentials)
            .expect("Serializing String fields cannot fail");
        std::fs::write(&self.path, json).context("Failed to write session file")
    }

    fn load(&self) -> Option<StoredCredentials> {
        let content = std::fs::read_to_string(&self.path).ok()?;
        serde_json::from_str(&content)
            .map_err(|e| {
                eprintln!("Corrupt session file, ignoring: {e}");
                e
            })
            .ok()
    }

    fn clear(&self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> (FileSessionStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("session.json");
        (FileSessionStore::with_path(path), dir)
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
        // Exercises FileSessionStore::new() → dirs::data_local_dir() → create_dir_all().
        let store = FileSessionStore::new();
        // Verify the path points inside a sam_bridge subdirectory.
        assert!(store.path.ends_with("sam_bridge/session.json"));
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
    }

    #[test]
    fn corrupt_file_loads_as_none() {
        let (store, _dir) = temp_store();
        std::fs::write(&store.path, "{invalid json").expect("write corrupt data");

        assert!(store.load().is_none());
    }
}
