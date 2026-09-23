#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Credential {
    pub email: Email,
    pub password: Password,
}

impl Credential {
    #[must_use]
    pub const fn new(email: Email, password: Password) -> Self {
        Self { email, password }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Email(pub String);

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Password(pub String);
