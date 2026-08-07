use std::collections::HashMap;

#[ignore = "exploration"]
#[test]
fn login_page_returns_ui_html() {
    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_authentication_url()).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert_eq!(response_content.lines().next(), Some("<!DOCTYPE html>"));
}

#[ignore = "exploration"]
#[test]
fn login_with_invalid_credentials_returns_error_in_html() {
    let client: reqwest::blocking::Client = build_http_client();

    let (user, password) = build_invalid_credentials();

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("login", user);
    form.insert("password", password);

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url())
        .form(&form)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("<p>* Oops... O usuário ou senha incorretos!</p>"));
}

#[ignore = "exploration"]
#[test]
fn login_with_valid_credentials_returns_session_id_cookie() {
    let client: reqwest::blocking::Client = build_http_client();

    let (user, password) = build_valid_credentials();

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("login", user);
    form.insert("password", password);

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url())
        .form(&form)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
    assert!(
        response
            .headers()
            .get(reqwest::header::SET_COOKIE)
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with("PHPSESSID=")
    );

    let session_id_cookie: Option<reqwest::cookie::Cookie> =
        response.cookies().find(|i| i.name() == "PHPSESSID");
    println!("{:#?}", session_id_cookie);

    assert!(session_id_cookie.is_some());
}

#[ignore = "exploration"]
#[test]
fn dashboard_is_unacessable_if_not_logged_in() {
    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_dashboard_url()).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url()
    )
}

#[ignore = "exploration"]
#[test]
fn dashboard_is_acessable_if_previously_logged_in() {
    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_dashboard_url())
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("<span>Painel de Controle</span>"))
}

#[ignore = "exploration"]
#[test]
fn students_listing_is_unacessable_if_not_logged_in() {
    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_students_listing_url()).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url()
    )
}

#[ignore = "exploration"]
#[test]
fn students_listing_fails_even_if_previously_logged_in() {
    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url())
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(
        response.status(),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[ignore = "exploration"]
#[test]
fn students_listing_succeeds_if_previously_logged_in_and_has_visited_dashboard() {
    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    visit_dashboard(&client, &session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url())
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("recordsTotal"));
    assert!(response_content.contains("data"));
}

#[ignore = "exploration"]
#[test]
fn student_lessons_fail_if_not_logged_in() {
    let client: reqwest::blocking::Client = build_http_client();

    let invalid_student_id: String = build_invalid_student_id();

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&invalid_student_id))
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url()
    );
}

#[ignore = "exploration"]
#[test]
fn student_lessons_returns_nothing_if_logged_in_but_invalid_student_id() {
    let client: reqwest::blocking::Client = build_http_client();

    let invalid_student_id: String = build_invalid_student_id();

    let authorized_session_id: String = get_authenticated_session_id(&client);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&invalid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert_eq!(response_content, "");
}

#[ignore = "exploration"]
#[test]
fn student_lessons_succeeds_if_logged_in_and_valid_student_id() {
    let client: reqwest::blocking::Client = build_http_client();

    let valid_student_id: String = build_valid_student_id();

    let authorized_session_id: String = get_authenticated_session_id(&client);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&valid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("Lições Aprovadas"));
}

fn build_http_client() -> reqwest::blocking::Client {
    reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap()
}

fn build_sam_authentication_url() -> String {
    format!("{}/autenticar", build_sam_base_url())
}

fn build_sam_dashboard_url() -> String {
    format!("{}/painel", build_sam_base_url())
}

fn build_sam_students_listing_url() -> String {
    format!("{}/alunos/listagem", build_sam_base_url())
}

fn build_sam_student_lessons_url(student_id: &String) -> String {
    format!(
        "{}/licoes/index/{}",
        build_sam_base_url().trim_end_matches("/"),
        student_id
    )
}
fn build_sam_base_url() -> &'static str {
    "https://musical.congregacao.org.br/"
}
fn build_invalid_credentials() -> (&'static str, &'static str) {
    ("someuser", "somepassword")
}

fn build_valid_credentials() -> (&'static str, &'static str) {
    // Should put some valid credentials here
    ("<VALID USER>", "<VALID PASSWORD>")
}

fn build_invalid_student_id() -> String {
    "someStudent".to_string()
}

fn build_valid_student_id() -> String {
    "500132".to_string()
}

fn get_authenticated_session_id(client: &reqwest::blocking::Client) -> String {
    let (user, password) = build_valid_credentials();

    let mut form: HashMap<&str, &str> = HashMap::new();
    form.insert("login", user);
    form.insert("password", password);

    client
        .post(build_sam_authentication_url())
        .form(&form)
        .send()
        .unwrap()
        .cookies()
        .find(|i| i.name() == "PHPSESSID")
        .unwrap()
        .value()
        .to_string()
}

fn visit_dashboard(client: &reqwest::blocking::Client, session_cookie: &String) {
    client
        .get(build_sam_dashboard_url())
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
}
