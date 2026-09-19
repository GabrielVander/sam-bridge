/// Coarse, stable classification of why a gateway operation failed.
///
/// The UI picks user-facing copy from this; the accompanying `details` string
/// on the gateway error is for diagnostics only and must never contain
/// credentials.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Network,
    UnexpectedResponse,
    SessionExpired,
    Unknown,
}
