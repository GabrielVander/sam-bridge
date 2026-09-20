//! Wiring for tests that talk to a stand-in SAM site, usually a wiremock server.
//!
//! It mirrors how the application configures its HTTP client: redirects are
//! not followed and cookies are kept, because SAM tracks the login session in a
//! cookie.

use sam::client::SamClientImpl;
use sam::http::SamOperations;

/// SAM operations pointed at `base_url`, with the endpoint names the
/// application uses.
///
/// # Errors
///
/// Returns the error of the underlying HTTP client if it cannot be built.
pub fn sam_operations_for(base_url: &str) -> Result<SamOperations, reqwest::Error> {
    let http_client: reqwest::blocking::Client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()?;

    Ok(SamOperations::new(
        http_client,
        base_url,
        "autenticar",
        "painel",
        "alunos/listagem",
        "licoes/index",
    ))
}

/// A SAM client talking to `base_url`.
///
/// # Errors
///
/// Returns the error of the underlying HTTP client if it cannot be built.
pub fn sam_client_for(base_url: &str) -> Result<SamClientImpl, reqwest::Error> {
    sam_operations_for(base_url).map(SamClientImpl::new)
}
