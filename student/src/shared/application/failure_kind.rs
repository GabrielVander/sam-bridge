/// Coarse, stable classification of why a gateway operation failed.
///
/// The UI picks user-facing copy from this; the accompanying `details` string
/// on each gateway error is for diagnostics only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Network,
    UnexpectedResponse,
    SessionExpired,
    Unknown,
}
