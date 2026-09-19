use std::error::Error;

use authentication::application::gateways::FailureKind as AuthenticationFailureKind;
use student::application::gateways::FailureKind;

use crate::client::SamClientError;

const MAX_CHARS: usize = 500;
const KEPT_CHARS_WHEN_TRUNCATED: usize = 499;

/// Renders an error and its `source()` chain, outermost first, as a single
/// line meant for bug reports: `outer: middle: inner`.
///
/// The result is capped so an unexpectedly large cause (e.g. an embedded page
/// fragment) cannot flood the UI.
#[must_use]
pub fn error_chain(error: &dyn Error) -> String {
    let mut rendered: String = error.to_string();
    let mut current: Option<&(dyn Error + 'static)> = error.source();

    while let Some(cause) = current {
        rendered.push_str(": ");
        rendered.push_str(&cause.to_string());
        current = cause.source();
    }

    truncate(rendered)
}

pub(crate) const fn failure_kind(error: &SamClientError) -> FailureKind {
    match error {
        SamClientError::RequestError { .. } => FailureKind::Network,
        SamClientError::UnexpectedResponse { .. } => FailureKind::UnexpectedResponse,
        SamClientError::SessionExpired => FailureKind::SessionExpired,
        SamClientError::InvalidCredentials => FailureKind::Unknown,
    }
}

pub(crate) const fn authentication_failure_kind(
    error: &SamClientError,
) -> AuthenticationFailureKind {
    match error {
        SamClientError::RequestError { .. } => AuthenticationFailureKind::Network,
        SamClientError::UnexpectedResponse { .. } => AuthenticationFailureKind::UnexpectedResponse,
        SamClientError::SessionExpired => AuthenticationFailureKind::SessionExpired,
        SamClientError::InvalidCredentials => AuthenticationFailureKind::Unknown,
    }
}

fn truncate(text: String) -> String {
    if text.chars().count() <= MAX_CHARS {
        return text;
    }

    let mut truncated: String = text.chars().take(KEPT_CHARS_WHEN_TRUNCATED).collect();
    truncated.push('…');
    truncated
}
