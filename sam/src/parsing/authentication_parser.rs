use crate::http::SamResponse;

#[derive(Debug, Clone)]
pub struct AuthenticationParser;

impl AuthenticationParser {
    const INVALID_CREDENTIALS_MARKER: &str = "<p>* Oops... O usuário ou senha incorretos!</p>";

    pub fn parse_response(response: &SamResponse) -> AuthResponse {
        if response.status == 200 && response.body.contains(Self::INVALID_CREDENTIALS_MARKER) {
            return AuthResponse::InvalidCredentials;
        }

        if response.status == 303 {
            return AuthResponse::Authenticated;
        }

        AuthResponse::Unexpected
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum AuthResponse {
    Authenticated,
    InvalidCredentials,
    Unexpected,
}
