use authentication::application::gateways::CredentialStore;
use authentication::domain::entities::{Credential, Email, Password};
use credential_store::FileCredentialStore;

fn temp_store() -> std::io::Result<(FileCredentialStore, tempfile::TempDir)> {
    let dir = tempfile::tempdir()?;
    let store = FileCredentialStore::with_dir(&dir.path().to_string_lossy());
    Ok((store, dir))
}

fn credential() -> Credential {
    Credential::new(
        Email("test_user@example.com".to_owned()),
        Password("test_pass".to_owned()),
    )
}

#[test]
fn round_trip_preserves_credential() {
    let (store, _dir) = temp_store().expect("tempdir");

    smol::block_on(async {
        store
            .save(&credential())
            .await
            .expect("save should succeed");
        let loaded = store.load().await.expect("load should return Some");

        assert_eq!(loaded.email.0, "test_user@example.com");
        assert_eq!(loaded.password.0, "test_pass");
    });
}

#[test]
fn saved_file_is_encrypted_not_plaintext() {
    let (store, dir) = temp_store().expect("tempdir");

    smol::block_on(async {
        store.save(&credential()).await.expect("save");
    });

    let raw = std::fs::read(dir.path().join("session.enc")).expect("read enc file");
    let raw_str = String::from_utf8_lossy(&raw);
    assert!(
        !raw_str.contains("test_pass"),
        "encrypted file must not contain the plaintext password"
    );
}

#[test]
fn missing_file_loads_as_none() {
    let (store, _dir) = temp_store().expect("tempdir");

    smol::block_on(async {
        assert!(store.load().await.is_none());
    });
}

#[test]
fn clear_removes_the_credential() {
    let (store, dir) = temp_store().expect("tempdir");

    smol::block_on(async {
        store.save(&credential()).await.expect("save");

        store.clear().await.expect("clear should succeed");

        assert!(store.load().await.is_none());
    });
    assert!(!dir.path().join("session.enc").exists());
}

#[test]
fn tamper_detection_returns_none() {
    let (store, dir) = temp_store().expect("tempdir");
    let credential_path = dir.path().join("session.enc");

    smol::block_on(async {
        store.save(&credential()).await.expect("save");
    });

    let mut data = std::fs::read(&credential_path).expect("read");
    if !data.is_empty() {
        data[0] ^= 0xFF;
        std::fs::write(&credential_path, &data).expect("tamper");
    }

    smol::block_on(async {
        assert!(store.load().await.is_none());
    });
}

#[test]
fn a_second_store_pointed_at_the_same_dir_reads_what_the_first_wrote() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store1 = FileCredentialStore::with_dir(&dir.path().to_string_lossy());
    let store2 = FileCredentialStore::with_dir(&dir.path().to_string_lossy());

    smol::block_on(async {
        store1.save(&credential()).await.expect("save via store1");

        let loaded = store2.load().await.expect("load via store2");

        assert_eq!(loaded.email.0, "test_user@example.com");
        assert_eq!(loaded.password.0, "test_pass");
    });
}

#[test]
fn decrypt_with_wrong_key_returns_none() {
    let dir1 = tempfile::tempdir().expect("dir1");
    let store1 = FileCredentialStore::with_dir(&dir1.path().to_string_lossy());
    let dir2 = tempfile::tempdir().expect("dir2");
    let store2 = FileCredentialStore::with_dir(&dir2.path().to_string_lossy());

    smol::block_on(async {
        store1.save(&credential()).await.expect("save1");
        store2
            .save(&credential())
            .await
            .expect("save2 to create a different key");
    });

    let data = std::fs::read(dir1.path().join("session.enc")).expect("data");
    std::fs::write(dir2.path().join("session.enc"), &data).expect("copy data with wrong key");

    smol::block_on(async {
        assert!(store2.load().await.is_none());
    });
}

#[cfg(unix)]
#[test]
fn files_have_restricted_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let (store, dir) = temp_store().expect("tempdir");

    smol::block_on(async {
        store.save(&credential()).await.expect("save");
    });

    let meta = std::fs::metadata(dir.path().join("session.enc")).expect("meta");
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    let key_meta = std::fs::metadata(dir.path().join("key.bin")).expect("key meta");
    assert_eq!(key_meta.permissions().mode() & 0o777, 0o600);
}
