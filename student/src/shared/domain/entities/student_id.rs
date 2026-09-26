/// How SAM identifies a student, shared by the roster and their lessons.
#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct StudentId(String);

impl StudentId {
    #[must_use]
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
