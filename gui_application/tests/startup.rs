use gui_application::api::build_main_application;

#[test]
fn the_production_application_stores_credentials_in_the_given_data_directory() {
    let data_dir = tempfile::tempdir().unwrap();

    let application = build_main_application(data_dir.path().to_string_lossy().into_owned());

    assert!(application.is_ok());
    assert!(data_dir.path().join("sam_bridge").is_dir());
}
