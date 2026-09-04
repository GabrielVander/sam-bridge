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

pub struct Email(pub String);

pub struct Password(pub String);
