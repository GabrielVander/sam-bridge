#[derive(Debug, PartialEq)]
pub(crate) enum AuthOutcome {
    Authenticated,
    InvalidCredentials,
    Unexpected,
}

const INVALID_CREDENTIALS_MARKER: &str = "<p>* Oops... O usuário ou senha incorretos!</p>";

pub(crate) fn parse_authentication(status: reqwest::StatusCode, body: &str) -> AuthOutcome {
    if status == reqwest::StatusCode::OK && body.contains(INVALID_CREDENTIALS_MARKER) {
        return AuthOutcome::InvalidCredentials;
    }

    if status == reqwest::StatusCode::SEE_OTHER {
        return AuthOutcome::Authenticated;
    }

    AuthOutcome::Unexpected
}

#[cfg(test)]
mod authentication_tests {
    use super::{AuthOutcome, parse_authentication};
    use reqwest::StatusCode;

    #[test]
    fn given_see_other_response_authentication_should_succeed() {
        let outcome: AuthOutcome = parse_authentication(StatusCode::SEE_OTHER, "");

        assert_eq!(outcome, AuthOutcome::Authenticated);
    }

    #[test]
    fn given_see_other_response_with_body_authentication_should_still_succeed() {
        let outcome: AuthOutcome =
            parse_authentication(StatusCode::SEE_OTHER, "<html>redirecting...</html>");

        assert_eq!(outcome, AuthOutcome::Authenticated);
    }

    #[test]
    fn given_ok_response_with_invalid_credentials_marker_should_be_invalid_credentials() {
        let outcome: AuthOutcome = parse_authentication(
            StatusCode::OK,
            "<p>* Oops... O usuário ou senha incorretos!</p>",
        );

        assert_eq!(outcome, AuthOutcome::InvalidCredentials);
    }

    #[test]
    fn given_ok_response_without_invalid_credentials_marker_should_be_unexpected() {
        let outcome: AuthOutcome =
            parse_authentication(StatusCode::OK, "<html>welcome</html>");

        assert_eq!(outcome, AuthOutcome::Unexpected);
    }

    #[test]
    fn given_other_status_code_should_be_unexpected() {
        let outcome: AuthOutcome = parse_authentication(StatusCode::NOT_FOUND, "");

        assert_eq!(outcome, AuthOutcome::Unexpected);
    }
}
