use authentication::application::gateways::{
    ClearCredentialGateway, ClearCredentialGatewayError, LoadCredentialGateway,
    SaveCredentialGateway,
};
use authentication::domain::entities::{Credential, Email, Password};
use credential_store::{FileCredentialStore, NoDataDirectory};

#[test]
fn a_new_store_holds_no_credential() {
    let (store, _dir) = temp_store().expect("tempdir");

    assert!(store.load().is_none());
}

#[test]
fn saving_again_replaces_the_previous_credential() {
    let (store, _dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("first save");

    let second: Credential = Credential::new(
        Email("someone_else@example.com".to_owned()),
        Password("different_pass".to_owned()),
    );

    store.save(&second).expect("second save");

    let loaded: Credential = store.load().expect("load should return Some");

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

    let raw: Vec<u8> = std::fs::read(dir.path().join("session.enc")).expect("read enc file");
    let raw_str: std::borrow::Cow<str> = String::from_utf8_lossy(&raw);

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

    let mut data: Vec<u8> = std::fs::read(&credential_path).expect("read");

    if !data.is_empty() {
        data[0] ^= 0xFF;
        std::fs::write(&credential_path, &data).expect("tamper");
    }

    assert!(store.load().is_none());
}

#[test]
fn a_second_store_pointed_at_the_same_dir_reads_what_the_first_wrote() {
    let dir: tempfile::TempDir = tempfile::tempdir().expect("tempdir");
    let store1: FileCredentialStore = FileCredentialStore::with_dir(&dir.path().to_string_lossy());
    let store2: FileCredentialStore = FileCredentialStore::with_dir(&dir.path().to_string_lossy());

    store1.save(&credential()).expect("save via store1");

    let loaded: Credential = store2.load().expect("load via store2");

    assert_eq!(loaded.email.0, "test_user@example.com");
    assert_eq!(loaded.password.0, "test_pass");
}

#[test]
fn decrypt_with_wrong_key_returns_none() {
    let dir1: tempfile::TempDir = tempfile::tempdir().expect("dir1");
    let store1: FileCredentialStore = FileCredentialStore::with_dir(&dir1.path().to_string_lossy());
    let dir2: tempfile::TempDir = tempfile::tempdir().expect("dir2");
    let store2: FileCredentialStore = FileCredentialStore::with_dir(&dir2.path().to_string_lossy());

    store1.save(&credential()).expect("save1");
    store2
        .save(&credential())
        .expect("save2 to create a different key");

    let data: Vec<u8> = std::fs::read(dir1.path().join("session.enc")).expect("data");

    std::fs::write(dir2.path().join("session.enc"), &data).expect("copy data with wrong key");

    assert!(store2.load().is_none());
}

#[test]
fn saving_twice_reuses_the_existing_encryption_key() {
    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("first save");
    let key_after_first_save: Vec<u8> =
        std::fs::read(dir.path().join("key.bin")).expect("key after first save");

    store.save(&credential()).expect("second save");
    let key_after_second_save: Vec<u8> =
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
    use authentication::application::gateways::SaveCredentialGatewayError;

    let (store, dir) = temp_store().expect("tempdir");

    store
        .save(&credential())
        .expect("first save creates the key");

    std::fs::remove_file(dir.path().join("session.enc")).expect("remove the credential file");
    std::fs::create_dir(dir.path().join("session.enc"))
        .expect("occupy the credential path with a directory");

    let result: Result<(), SaveCredentialGatewayError> = store.save(&credential());

    assert!(
        result.is_err(),
        "a credential file path occupied by a directory must fail rather than silently succeed"
    );
}

#[test]
fn clear_fails_when_the_credential_file_cannot_be_removed() {
    let (store, dir) = temp_store().expect("tempdir");

    std::fs::create_dir(dir.path().join("session.enc"))
        .expect("occupy the credential path with a directory");

    let result: Result<(), ClearCredentialGatewayError> = store.clear();

    let Err(ClearCredentialGatewayError::UnableToPerformOperation { details }) = result else {
        panic!("a credential that could not be removed must not be reported as cleared");
    };

    let os_reason: String = std::fs::remove_file(dir.path().join("session.enc"))
        .expect_err("the directory still occupies the credential path")
        .to_string();

    assert!(
        details.contains(&os_reason),
        "details should say why the credential file could not be removed, got: {details}"
    );
}

#[cfg(unix)]
#[test]
fn save_fails_when_the_directory_is_not_writable_during_key_creation() {
    use std::os::unix::fs::PermissionsExt;

    use authentication::application::gateways::SaveCredentialGatewayError;

    let (store, dir) = temp_store().expect("tempdir");

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o500))
        .expect("make the directory read-only");

    let result: Result<(), SaveCredentialGatewayError> = store.save(&credential());

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

    use authentication::application::gateways::SaveCredentialGatewayError;

    let (store, dir) = temp_store().expect("tempdir");

    store
        .save(&credential())
        .expect("first save creates the key while the directory is still writable");

    std::fs::set_permissions(dir.path(), std::fs::Permissions::from_mode(0o500))
        .expect("make the directory read-only");

    let result: Result<(), SaveCredentialGatewayError> = store.save(&credential());

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

    let base: tempfile::TempDir = tempfile::tempdir().expect("tempdir");

    let _store: FileCredentialStore = credential_store::FileCredentialStore::under(base.path());

    let created_dir: std::path::PathBuf = base.path().join("sam_bridge");
    assert!(created_dir.is_dir(), "the data directory should be created");

    let mode: u32 = std::fs::metadata(&created_dir)
        .expect("metadata")
        .permissions()
        .mode()
        & 0o777;
    assert_eq!(mode, 0o700);
}

#[test]
fn a_store_under_a_base_directory_keeps_its_credentials_there() {
    let base: tempfile::TempDir = tempfile::tempdir().expect("tempdir");
    let store: FileCredentialStore = credential_store::FileCredentialStore::under(base.path());

    store.save(&credential()).expect("save");

    assert!(base.path().join("sam_bridge").join("session.enc").is_file());

    let loaded: Credential = store.load().expect("load should return Some");
    assert_eq!(loaded.email.0, "test_user@example.com");
}

#[cfg(unix)]
#[test]
fn files_have_restricted_permissions() {
    use std::os::unix::fs::PermissionsExt;

    let (store, dir) = temp_store().expect("tempdir");

    store.save(&credential()).expect("save");

    let meta: std::fs::Metadata = std::fs::metadata(dir.path().join("session.enc")).expect("meta");
    assert_eq!(meta.permissions().mode() & 0o777, 0o600);

    let key_meta: std::fs::Metadata =
        std::fs::metadata(dir.path().join("key.bin")).expect("key meta");
    assert_eq!(key_meta.permissions().mode() & 0o777, 0o600);
}

#[test]
fn without_a_platform_data_dir_there_is_nowhere_to_store_credentials() {
    let result: Result<FileCredentialStore, NoDataDirectory> =
        FileCredentialStore::in_platform_data_dir(None);

    assert_eq!(result.err(), Some(NoDataDirectory));
}

#[test]
fn a_store_in_the_platform_data_dir_keeps_its_credentials_there() {
    let base: tempfile::TempDir = tempfile::tempdir().expect("tempdir");
    let store: FileCredentialStore =
        FileCredentialStore::in_platform_data_dir(Some(base.path().to_path_buf()))
            .expect("a platform data dir is available");

    store.save(&credential()).expect("save should succeed");

    assert!(base.path().join("sam_bridge").join("session.enc").exists());
}

#[test]
fn the_missing_data_dir_error_explains_itself() {
    assert_eq!(
        NoDataDirectory.to_string(),
        "no platform data directory is available to store credentials"
    );
}

fn temp_store() -> std::io::Result<(FileCredentialStore, tempfile::TempDir)> {
    let dir: tempfile::TempDir = tempfile::tempdir()?;
    let store: FileCredentialStore = FileCredentialStore::with_dir(&dir.path().to_string_lossy());

    Ok((store, dir))
}

fn credential() -> Credential {
    Credential::new(
        Email("test_user@example.com".to_owned()),
        Password("test_pass".to_owned()),
    )
}
