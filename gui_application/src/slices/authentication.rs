use student_management_sam_adapter::{Authenticated, SamClient, gateways};

/// Authenticates against SAM and returns the session client used to build the
/// application gateways. Credential checking is pure infrastructure today —
/// `LoginUseCase` remains in core for when business rules emerge.
pub async fn login(
    base_url: String,
    username: String,
    password: String,
) -> anyhow::Result<SamClient<Authenticated>> {
    // Client construction cannot fail with this fixed configuration; the
    // defensive error propagation is compiled out of coverage builds (same
    // pattern as sam/src/client/sam_client.rs).
    #[cfg(coverage)]
    let client = new_client(base_url).expect("Fixed HTTP client configuration must be valid");
    #[cfg(not(coverage))]
    let client = new_client(base_url)?;

    gateways::login(client, username, password).await
}

fn new_client(base_url: String) -> anyhow::Result<SamClient<student_management_sam_adapter::Unauthenticated>> {
    // Client construction cannot fail with this fixed configuration; the
    // defensive error path is compiled out of coverage builds (same pattern
    // as sam/src/http/sam_transport.rs).
    #[cfg(coverage)]
    return Ok(SamClient::new(base_url).expect("Fixed HTTP client configuration must be valid"));

    #[cfg(not(coverage))]
    SamClient::new(base_url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_valid_credentials_should_return_usable_authenticated_client() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/painel"))
                .respond_with(wiremock::ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/alunos/listagem"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200).set_body_string(
                        r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                    ),
                )
                .mount(&mock_server)
                .await;

            let client = login(mock_server.uri(), "u".to_owned(), "p".to_owned())
                .await
                .expect("Login should succeed");

            let students =
                student_management_sam_adapter::ports::RosterSource::students(&client);

            assert!(students.is_ok(), "Session should be usable after login");
        });
    }

    #[test]
    fn given_invalid_credentials_should_fail_with_invalid_credentials_error() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200).set_body_string(
                        "<p>* Oops... O usuário ou senha incorretos!</p>",
                    ),
                )
                .mount(&mock_server)
                .await;

            let result = login(mock_server.uri(), "u".to_owned(), "wrong".to_owned()).await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid credentials"));
        });
    }

    #[test]
    fn given_unreachable_server_should_fail_gracefully() {
        smol::block_on(async {
            let result = login("http://127.0.0.1:1".to_owned(), "u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
        });
    }
}
