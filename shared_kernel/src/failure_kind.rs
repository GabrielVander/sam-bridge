#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FailureKind {
    Transient,
    Unexpected,
    SessionExpired,
    Unclassified,
}
