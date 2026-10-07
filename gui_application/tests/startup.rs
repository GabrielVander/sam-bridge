use gui_application::api::build_main_application;

#[test]
fn the_production_application_stores_credentials_in_the_platform_data_directory() {
    let platform_data_dir = tempfile::tempdir().unwrap();
    // SAFETY: this binary holds a single test, so no other thread reads the environment.
    unsafe { std::env::set_var("XDG_DATA_HOME", platform_data_dir.path()) };

    let application = build_main_application();

    assert!(application.is_ok());
    assert!(platform_data_dir.path().join("sam_bridge").is_dir());
}
