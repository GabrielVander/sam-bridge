use authentication::application::gateways::{
    ClearCredentialGateway, LoadCredentialGateway, SaveCredentialGateway,
};
use authentication::domain::entities::{Credential, Email, Password};
use credential_store::FileCredentialStore;

#[test]
fn a_new_store_holds_no_credential() {
    let (store, _dir) = temp_store().expect("tempdir");

    assert!(store.load().is_none());
}

#[test]
fn saving_again_replaces_the_previous_credential() {
    let (store, _dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("first save");
    let second = Credential::new(
        Email("someone_else@example.com".to_owned()),
        Password("different_pass".to_owned()),
    );
    store.save(&second).expect("second save");

    let loaded = store.load().expect("load should return Some");
    assert_eq!(loaded.email.0, "someone_else@example.com");
    assert_eq!(loaded.password.0, "different_pass");
}

#[test]
fn clearing_an_empty_store_is_not_an_error() {
    let (store, _dir) = temp_store().expect("tempdir");

    store
        .clear()
        .expect("clearing an empty store should succeed");

    assert!(store.load().is_none());
}

#[test]
fn saved_file_is_encrypted_not_plaintext() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    let raw = std::fs::read(dir.path().join("session.enc")).expect("read enc file");
    let raw_str = String::from_utf8_lossy(&raw);
    assert!(
        !raw_str.contains("test_pass"),
        "encrypted file must not contain the plaintext password"
    );
}

#[test]
fn clear_removes_the_credential() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    store.clear().expect("clear should succeed");

    assert!(store.load().is_none());
    assert!(!dir.path().join("session.enc").exists());
}

#[test]
fn tamper_detection_returns_none() {
    let (store, dir) = temp_store().expect("tempdir");
    let credential_path = dir.path().join("session.enc");

    store.save(&credential()).expect("save");

    let mut data = std::fs::read(&credential_path).expect("read");
    if !data.is_empty() {
        data[0] ^= 0xFF;
        std::fs::write(&credential_path, &data).expect("tamper");
    }

    assert!(store.load().is_none());
}

#[test]
fn a_second_store_pointed_at_the_same_dir_reads_what_the_first_wrote() {
    let dir = tempfile::tempdir().expect("tempdir");
    let store1 = FileCredentialStore::with_dir(&dir.path().to_string_lossy());
    let store2 = FileCredentialStore::with_dir(&dir.path().to_string_lossy());

    store1.save(&credential()).expect("save via store1");

    let loaded = store2.load().expect("load via store2");

    assert_eq!(loaded.email.0, "test_user@example.com");
    assert_eq!(loaded.password.0, "test_pass");
}

#[test]
fn decrypt_with_wrong_key_returns_none() {
    let dir1 = tempfile::tempdir().expect("dir1");
    let store1 = FileCredentialStore::with_dir(&dir1.path().to_string_lossy());
    let dir2 = tempfile::tempdir().expect("dir2");
    let store2 = FileCredentialStore::with_dir(&dir2.path().to_string_lossy());

    store1.save(&credential()).expect("save1");
    store2
        .save(&credential())
        .expect("save2 to create a different key");

    let data = std::fs::read(dir1.path().join("session.enc")).expect("data");
    std::fs::write(dir2.path().join("session.enc"), &data).expect("copy data with wrong key");

    assert!(store2.load().is_none());
}

#[test]
fn saving_twice_reuses_the_existing_encryption_key() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("first save");
    let key_after_first_save =
        std::fs::read(dir.path().join("key.bin")).expect("key after first save");

    store.save(&credential()).expect("second save");
    let key_after_second_save =
        std::fs::read(dir.path().join("key.bin")).expect("key after second save");

    assert_eq!(
        key_after_first_save, key_after_second_save,
        "the encryption key must be reused across saves, not regenerated"
    );
}

#[test]
fn a_corrupted_key_file_causes_load_to_return_none() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    std::fs::write(dir.path().join("key.bin"), [0u8; 10]).expect("corrupt the key file");

    assert!(store.load().is_none());
}

#[test]
fn a_missing_key_file_causes_load_to_return_none() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    std::fs::remove_file(dir.path().join("key.bin")).expect("remove the key file");

    assert!(store.load().is_none());
}

#[cfg(unix)]
#[test]
fn save_fails_when_the_key_file_path_is_a_directory() {
    let (store, dir) = temp_store().expect("tempdir");
    std::fs::create_dir(dir.path().join("key.bin")).expect("occupy the key path with a directory");

    let result = store.save(&credential());

    assert!(
        result.is_err(),
        "a key file path occupied by a directory must fail rather than silently succeed"
    );
}

#[cfg(unix)]
#[test]
fn save_fails_when_the_credential_file_path_is_a_directory() {
    let (store, dir) = temp_store().expect("tempdir");
    store
        .save(&credential())
        .expect("first save creates the key");
    std::fs::remove_file(dir.path().join("session.enc")).expect("remove the credential file");
    std::fs::create_dir(dir.path().join("session.enc"))
        .expect("occupy the credential path with a directory");

    let result = store.save(&credential());

    assert!(
        result.is_err(),
        "a credential file path occupied by a directory must fail rather than silently succeed"
    );
}

#[cfg(unix)]
#[test]
fn save_fails_when_the_directory_is_not_writable_during_key_creation() {
    use std::os::unix::fs::PermissionsExt;

    let (store, dir) = temp_store().expect("tempdir");
    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o500))
        .expect("make the directory read-only");

    let result = store.save(&credential());

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))
        .expect("restore permissions so the tempdir can be cleaned up");

    assert!(
        result.is_err(),
        "creating the encryption key in a read-only directory must fail rather than silently succeed"
    );
}

#[cfg(unix)]
#[test]
fn save_fails_when_the_directory_is_not_writable_for_the_credential_file() {
    use std::os::unix::fs::PermissionsExt;

    let (store, dir) = temp_store().expect("tempdir");
    store
        .save(&credential())
        .expect("first save creates the key while the directory is still writable");

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o500))
        .expect("make the directory read-only");

    let result = store.save(&credential());

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o700))
        .expect("restore permissions so the tempdir can be cleaned up");

    assert!(
        result.is_err(),
        "writing the credential file into a read-only directory must fail rather than silently succeed"
    );
}

#[cfg(unix)]
#[test]
fn under_creates_the_data_directory_with_restricted_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let base = tempfile::tempdir().expect("tempdir");

    let _store = credential_store::FileCredentialStore::under(base.path());

    let created_dir = base.path().join("sam_bridge");
    assert!(created_dir.is_dir(), "the data directory should be created");

    let mode = std::fs::metadata(&created_dir)
        .expect("metadata")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o700);
}

#[test]
fn a_store_under_a_base_directory_keeps_its_credentials_there() {
    let base = tempfile::tempdir().expect("tempdir");
    let store = credential_store::FileCredentialStore::under(base.path());

    store.save(&credential()).expect("save");

    assert!(base.path().join("sam_bridge").join("session.enc").is_file());
    let loaded = store.load().expect("load should return Some");
    assert_eq!(loaded.email.0, "test_user@example.com");
}

#[cfg(unix)]
#[test]
fn files_have_restricted_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    let meta = std::fs::metadata(dir.path().join("session.enc")).expect("meta");
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);
    let key_meta = std::fs::metadata(dir.path().join("key.bin")).expect("key meta");
    assert_eq!(key_meta.permissions().mode() & 0o777, 0o600);
}

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
