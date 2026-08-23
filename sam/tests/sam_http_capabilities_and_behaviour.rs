//! Exploration tests exercising the real SAM site's HTTP behavior.
//!
//! These document observed site behavior (redirects, session cookies,
//! endpoint quirks) and double as opt-in smoke tests. They run only when
//! `SAM_USERNAME` and `SAM_PASSWORD` are set; otherwise each test reports
//! a skip. `SAM_BASE_URL` optionally overrides the target site.

struct SamSiteConfig {
    base_url: String,
    username: String,
    password: String,
}

fn configured_sam_site() -> Option<SamSiteConfig> {
    let username: String = std::env::var("SAM_USERNAME").ok()?;
    let password: String = std::env::var("SAM_PASSWORD").ok()?;
    let base_url: String =
        std::env::var("SAM_BASE_URL").unwrap_or_else(|_| "https://musical.congregacao.org.br".to_owned());

    Some(SamSiteConfig {
        base_url,
        username,
        password,
    })
}

fn require_sam_site() -> SamSiteConfig {
    let Some(site): Option<SamSiteConfig> = configured_sam_site() else {
        eprintln!(
            "skipping: set SAM_USERNAME and SAM_PASSWORD (optionally SAM_BASE_URL) to run against the real SAM site"
        );
        return SamSiteConfig {
            base_url: String::new(),
            username: String::new(),
            password: String::new(),
        };
    };

    site
}

#[test]
fn invalid_url() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let response_result: reqwest::Result<reqwest::blocking::Response> =
        client.get(build_invalid_sam_base_url()).send();

    println!("{:#?}", response_result);

    assert!(response_result.is_err());
    assert_eq!(
        response_result.unwrap_err().to_string(),
        format!(
            "error sending request for url ({})",
            build_invalid_sam_base_url()
        )
    );
}

#[test]
fn login_page_returns_ui_html() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_authentication_url(&site)).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert_eq!(response_content.lines().next(), Some("<!DOCTYPE html>"));
}

#[test]
fn login_with_invalid_credentials_returns_error_in_html() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let form = [("login", "someuser"), ("password", "somepassword")];

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url(&site))
        .form(&form)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("<p>* Oops... O usuário ou senha incorretos!</p>"));
}

#[test]
fn login_with_valid_credentials_returns_session_id_cookie() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url(&site))
        .form(&[("login", site.username.as_str()), ("password", site.password.as_str())])
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

#[test]
fn dashboard_is_unacessable_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_dashboard_url(&site)).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url(&site)
    )
}

#[test]
fn dashboard_is_acessable_if_previously_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client, &site);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_dashboard_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert!(response_content.contains("<span>Painel de Controle</span>"))
}

#[test]
fn students_listing_is_unacessable_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let response: reqwest::blocking::Response =
        client.get(build_sam_students_listing_url(&site)).send().unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url(&site)
    )
}

#[test]
fn students_listing_fails_even_if_previously_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client, &site);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(
        response.status(),
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    );
}

#[test]
fn students_listing_succeeds_if_previously_logged_in_and_has_visited_dashboard() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let authorized_session_id: String = get_authenticated_session_id(&client, &site);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    visit_dashboard(&client, &site, &session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
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

#[test]
fn student_lessons_fail_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let invalid_student_id: String = build_invalid_student_id();

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &invalid_student_id))
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert_eq!(
        response.headers().get("location").unwrap(),
        build_sam_base_url(&site)
    );
}

#[test]
fn student_lessons_returns_nothing_if_logged_in_but_invalid_student_id() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let invalid_student_id: String = build_invalid_student_id();

    let authorized_session_id: String = get_authenticated_session_id(&client, &site);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &invalid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{:#?}", response);

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{:#?}", response_content);

    assert_eq!(response_content, "");
}

#[test]
fn student_lessons_succeeds_if_logged_in_and_valid_student_id() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = build_http_client();

    let valid_student_id: String = build_valid_student_id();

    let authorized_session_id: String = get_authenticated_session_id(&client, &site);
    let session_cookie: String = format!("PHPSESSID={}", authorized_session_id);
    println!("{:#?}", session_cookie);

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &valid_student_id))
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

fn build_sam_authentication_url(site: &SamSiteConfig) -> String {
    format!("{}/autenticar", build_sam_base_url(site))
}

fn build_sam_dashboard_url(site: &SamSiteConfig) -> String {
    format!("{}/painel", build_sam_base_url(site))
}

fn build_sam_students_listing_url(site: &SamSiteConfig) -> String {
    format!("{}/alunos/listagem", build_sam_base_url(site))
}

fn build_sam_student_lessons_url(site: &SamSiteConfig, student_id: &String) -> String {
    format!(
        "{}/licoes/index/{}",
        build_sam_base_url(site).trim_end_matches("/"),
        student_id
    )
}

fn build_invalid_sam_base_url() -> &'static str {
    "https://musical.musical.invalid_url.org.br/"
}

fn build_sam_base_url(site: &SamSiteConfig) -> &str {
    site.base_url.as_str()
}

fn build_invalid_student_id() -> String {
    "someStudent".to_string()
}

fn build_valid_student_id() -> String {
    "500132".to_string()
}

fn get_authenticated_session_id(
    client: &reqwest::blocking::Client,
    site: &SamSiteConfig,
) -> String {
    client
        .post(build_sam_authentication_url(site))
        .form(&[("login", site.username.as_str()), ("password", site.password.as_str())])
        .send()
        .unwrap()
        .cookies()
        .find(|i| i.name() == "PHPSESSID")
        .unwrap()
        .value()
        .to_string()
}

fn visit_dashboard(client: &reqwest::blocking::Client, site: &SamSiteConfig, session_cookie: &String) {
    client
        .get(build_sam_dashboard_url(site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
}
