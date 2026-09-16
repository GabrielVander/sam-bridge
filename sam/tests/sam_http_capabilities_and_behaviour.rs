use std::collections::{BTreeMap, BTreeSet};

const MAX_STUDENTS_PER_INSTRUMENT: usize = 15;

struct SamSiteConfig {
    base_url: String,
    username: String,
    password: String,
}

fn configured_sam_site() -> Option<SamSiteConfig> {
    let username: String = std::env::var("SAM_USERNAME").ok()?;
    let password: String = std::env::var("SAM_PASSWORD").ok()?;
    let base_url: String = std::env::var("SAM_BASE_URL")
        .unwrap_or_else(|_| "https://musical.congregacao.org.br".to_owned());

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

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response_result: reqwest::Result<reqwest::blocking::Response> =
        client.get(build_invalid_sam_base_url()).send();

    println!("{response_result:#?}");

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

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response: reqwest::blocking::Response = client
        .get(build_sam_authentication_url(&site))
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert_eq!(response_content.lines().next(), Some("<!DOCTYPE html>"));
}

#[test]
fn login_with_invalid_credentials_returns_error_in_html() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let form = [("login", "someuser"), ("password", "somepassword")];

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url(&site))
        .form(&form)
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("<p>* Oops... O usuário ou senha incorretos!</p>"));
}

#[test]
fn login_with_valid_credentials_returns_session_id_cookie() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response: reqwest::blocking::Response = client
        .post(build_sam_authentication_url(&site))
        .form(&[
            ("login", site.username.as_str()),
            ("password", site.password.as_str()),
        ])
        .send()
        .unwrap();
    println!("{response:#?}");

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
    println!("{session_id_cookie:#?}");

    assert!(session_id_cookie.is_some());
}

#[test]
fn dashboard_is_unacessable_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response: reqwest::blocking::Response =
        client.get(build_sam_dashboard_url(&site)).send().unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert!(
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(build_sam_base_url(&site))
    );
}

#[test]
fn dashboard_is_acessable_if_previously_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");
    println!("{session_cookie:#?}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_dashboard_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("<span>Painel de Controle</span>"));
}

#[test]
fn students_listing_is_unacessable_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert!(
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(build_sam_base_url(&site))
    );
}

#[test]
fn students_listing_fails_even_if_previously_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");
    println!("{session_cookie:#?}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{response:#?}");

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

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");
    println!("{session_cookie:#?}");

    {
        client
            .get(build_sam_dashboard_url(&site))
            .header(reqwest::header::COOKIE, &session_cookie)
            .send()
            .unwrap();
    };

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("recordsTotal"));
    assert!(response_content.contains("data"));
}

#[test]
fn student_lessons_fail_if_not_logged_in() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let invalid_student_id: String = build_invalid_student_id();

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &invalid_student_id))
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::TEMPORARY_REDIRECT);
    assert!(
        response
            .headers()
            .get("location")
            .unwrap()
            .to_str()
            .unwrap()
            .starts_with(build_sam_base_url(&site))
    );
}

#[test]
fn student_lessons_returns_nothing_if_logged_in_but_invalid_student_id() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let invalid_student_id: String = build_invalid_student_id();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");
    println!("{session_cookie:#?}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &invalid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert_eq!(response_content, "");
}

#[test]
fn student_lessons_succeeds_if_logged_in_and_valid_student_id() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let valid_student_id: String = build_valid_student_id();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");
    println!("{session_cookie:#?}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &valid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();
    println!("{response:#?}");

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("Lições Aprovadas"));
}

#[test]
fn student_lessons_contains_msa_lessons() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let valid_student_id: String = build_valid_student_id();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &valid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("<tr id=\"msa_42763\">\n            <td>19/09/2023</td>\n            <td>4.4 - 4.5</td>\n            <td>38 - 39</td>\n            <td>13 - 14</td>\n            <td>Sol</td>\n            <td>Estudar o praticar solfejo </td>\n            <td>THIAGO SOUZA SANTOS</td>\n            <td>\n                                    <button type=\"button\" class=\"btn btn-danger btn-sm\" data-toggle=\"tooltip\" title=\"Excluir\"\n                            onclick=\"delete_lancamento_msa(42763)\">\n                        <i class=\"fa fa-trash\"></i> Apagar\n                    </button>\n                            </td>\n        </tr>"));
}

#[test]
fn student_lessons_contains_instrument_lessons() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let valid_student_id: String = "545039".to_string();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");

    let response: reqwest::blocking::Response = client
        .get(build_sam_student_lessons_url(&site, &valid_student_id))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();

    let response_content: String = response.text().unwrap();
    println!("{response_content:#?}");

    assert!(response_content.contains("<tr id=\"mtd_214020\">\n            <td>00</td>\n            <td>00</td>\n            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>\n            <td>04/12/2023</td>\n            <td>MURILO FAGNER CARDOSO</td>\n            <td>04/12/2023 21:17:17</td>\n            <td>Postura do violino </td>\n            <td>\n                                <button type=\"button\" class=\"btn btn-danger btn-sm\" data-toggle=\"tooltip\" title=\"Excluir\"\n                    onclick=\"delete_lancamento_mtd(214020)\">\n                    <i class=\"fa fa-trash\"></i> Apagar\n                </button>\n                            </td>\n        </tr>"));
}

/// Exploratory: pure HTTP + raw JSON against the real site, nothing from
/// this workspace's own crates (no `sam::client`, no parsing helpers). Not
/// asserting exact vocabulary (we don't control SAM's data entry), just
/// surfacing what raw `role`/`level`/`instrument` strings actually occur
/// today. Prints only the distinct raw values, never individual student
/// names or ids.
#[test]
fn discovers_role_level_and_instrument_vocabulary_from_the_real_students_listing() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");

    // The listing endpoint 500s unless the dashboard was visited first in
    // this session (see `students_listing_fails_even_if_previously_logged_in`
    // above).
    client
        .get(build_sam_dashboard_url(&site))
        .header(reqwest::header::COOKIE, &session_cookie)
        .send()
        .unwrap();

    let response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .unwrap();

    assert_eq!(response.status(), reqwest::StatusCode::OK);

    let response_content: String = response.text().unwrap();
    let parsed: serde_json::Value =
        serde_json::from_str(&response_content).expect("listing response should be valid JSON");
    let rows: &Vec<serde_json::Value> = parsed["data"]
        .as_array()
        .expect("listing response should have a data array");

    assert!(
        !rows.is_empty(),
        "expected at least one student record in the real roster"
    );

    let distinct_roles: BTreeSet<String> = rows.iter().map(|r| column(r, 3)).collect();
    let distinct_levels: BTreeSet<String> = rows.iter().map(|r| column(r, 5)).collect();
    let distinct_instruments: BTreeSet<String> = rows.iter().map(|r| column(r, 4)).collect();

    println!("total students: {}", rows.len());
    println!(
        "distinct roles ({}): {distinct_roles:#?}",
        distinct_roles.len()
    );
    println!(
        "distinct levels ({}): {distinct_levels:#?}",
        distinct_levels.len()
    );
    println!(
        "distinct instruments ({}): {distinct_instruments:#?}",
        distinct_instruments.len()
    );

    for role in &distinct_roles {
        let levels_for_role: BTreeSet<String> = rows
            .iter()
            .filter(|r| &column(r, 3) == role)
            .map(|r| column(r, 5))
            .collect();
        println!("levels for role {role:?}: {levels_for_role:#?}");
    }

    let mut instrument_counts: BTreeMap<String, usize> = BTreeMap::new();
    for row in rows {
        if column(row, 3) == "MÚSICO" {
            *instrument_counts.entry(column(row, 4)).or_insert(0) += 1;
        }
    }
    println!("instrument counts for role MÚSICO: {instrument_counts:#?}");
}

/// Exploratory: pure HTTP + `scraper` (a third-party HTML parser, not this
/// workspace's own parsing code), nothing from `sam::client`/`sam::parsing`.
/// Samples a handful of students per instrument (not all ~700 - this hits
/// the real production site) and reads their real method-lesson rows, to see
/// what "Método" values SAM actually records per instrument. An instrument
/// can have students on more than one method book at once, so this collects
/// every distinct value seen, not just one.
#[test]
fn discovers_method_names_actually_used_per_instrument() {
    let site: SamSiteConfig = require_sam_site();
    if site.base_url.is_empty() {
        return;
    }

    let client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let authorized_session_id: String = {
        client
            .post(build_sam_authentication_url(&site))
            .form(&[
                ("login", site.username.as_str()),
                ("password", site.password.as_str()),
            ])
            .send()
            .unwrap()
            .cookies()
            .find(|i| i.name() == "PHPSESSID")
            .unwrap()
            .value()
            .to_string()
    };
    let session_cookie: String = format!("PHPSESSID={authorized_session_id}");

    client
        .get(build_sam_dashboard_url(&site))
        .header(reqwest::header::COOKIE, &session_cookie)
        .send()
        .unwrap();

    let listing_response: reqwest::blocking::Response = client
        .get(build_sam_students_listing_url(&site))
        .header(reqwest::header::COOKIE, session_cookie.clone())
        .send()
        .unwrap();
    let listing_body: String = listing_response.text().unwrap();
    let listing: serde_json::Value =
        serde_json::from_str(&listing_body).expect("listing response should be valid JSON");
    let rows: &Vec<serde_json::Value> = listing["data"]
        .as_array()
        .expect("listing response should have a data array");

    let mut sampled_student_ids_by_instrument: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for row in rows {
        if column(row, 3) != "MÚSICO" {
            continue;
        }
        let instrument: String = column(row, 4);
        if instrument == "A DEFINIR" {
            continue;
        }
        let ids: &mut Vec<String> = sampled_student_ids_by_instrument
            .entry(instrument)
            .or_default();
        if ids.len() < MAX_STUDENTS_PER_INSTRUMENT {
            ids.push(column(row, 0));
        }
    }

    // Table id and column position observed directly on the live lesson
    // page: the "Método" column is the 3rd `<td>` in each `datatable3` row.
    let mtd_row_selector: scraper::Selector =
        scraper::Selector::parse("table#datatable3 tbody tr").unwrap();
    let cell_selector: scraper::Selector = scraper::Selector::parse("td").unwrap();

    let mut methods_by_instrument: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for (instrument, student_ids) in &sampled_student_ids_by_instrument {
        let mut methods: BTreeSet<String> = BTreeSet::new();

        for student_id in student_ids {
            let response: reqwest::blocking::Response = client
                .get(build_sam_student_lessons_url(&site, student_id))
                .header(reqwest::header::COOKIE, session_cookie.clone())
                .send()
                .unwrap();
            let body: String = response.text().unwrap();

            let document: scraper::Html = scraper::Html::parse_document(&body);
            for row in document.select(&mtd_row_selector) {
                if let Some(method_cell) = row.select(&cell_selector).nth(2) {
                    let method_name: String = method_cell
                        .text()
                        .collect::<Vec<_>>()
                        .join(" ")
                        .trim()
                        .to_owned();
                    if !method_name.is_empty() {
                        methods.insert(method_name);
                    }
                }
            }
        }

        methods_by_instrument.insert(instrument.clone(), methods);
    }

    println!("methods actually used per instrument (sampled): {methods_by_instrument:#?}");

    assert!(
        !methods_by_instrument.is_empty(),
        "expected at least one sampled instrument"
    );
}

/// Raw `DataTables` row column order per the live response:
/// [id, name, location, role, instrument, level, ...].
fn column(row: &serde_json::Value, index: usize) -> String {
    row.get(index)
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned()
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
        build_sam_base_url(site).trim_end_matches('/'),
        student_id
    )
}

const fn build_invalid_sam_base_url() -> &'static str {
    "https://musical.musical.invalid_url.org.br/"
}

const fn build_sam_base_url(site: &SamSiteConfig) -> &str {
    site.base_url.as_str()
}

fn build_invalid_student_id() -> String {
    "someStudent".to_string()
}

fn build_valid_student_id() -> String {
    "500132".to_string()
}
