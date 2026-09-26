#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Credential {
    email: Email,
    password: Password,
}

impl Credential {
    #[must_use]
    pub const fn new(email: Email, password: Password) -> Self {
        Self { email, password }
    }

    #[must_use]
    pub const fn email(&self) -> &Email {
        &self.email
    }

    #[must_use]
    pub const fn password(&self) -> &Password {
        &self.password
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Email(String);

impl Email {
    #[must_use]
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Password(String);

impl Password {
    #[must_use]
    pub const fn new(value: String) -> Self {
        Self(value)
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Debug for Password {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("Password(***)")
    }
}
