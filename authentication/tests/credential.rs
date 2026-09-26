use authentication::domain::entities::{Credential, Email, Password};

#[test]
fn debug_output_never_shows_the_password() {
    let credential: Credential = Credential::new(
        Email::new("someone@example.com".to_owned()),
        Password::new("hunter2-secret".to_owned()),
    );

    let printed: String = format!("{credential:?}");

    assert!(printed.contains("someone@example.com"), "got: {printed}");
    assert!(!printed.contains("hunter2-secret"), "got: {printed}");
}
