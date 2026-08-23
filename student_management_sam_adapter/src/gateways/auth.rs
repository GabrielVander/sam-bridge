use async_trait::async_trait;
use student_management::api::application::AuthGateway;

use crate::ports::AuthSource;

pub struct SamAuthGateway<S: AuthSource> {
    source: S,
}

impl<S: AuthSource> SamAuthGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> AuthGateway for SamAuthGateway<S>
where
    S: AuthSource + Clone + Send + Sync + 'static,
{
    async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        let source = self.source.clone();
        // sam is blocking: run on smol's thread pool.
        smol::unblock(move || source.login(&username, &password))
            .await
            .map(|_| ())
    }
}

pub async fn login(
    client: sam::client::SamClient<sam::client::Unauthenticated>,
    username: String,
    password: String,
) -> anyhow::Result<sam::client::SamClient<sam::client::Authenticated>> {
    smol::unblock(move || crate::ports::AuthSource::login(&client, &username, &password)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::{Authenticated, SamClient, SamCredentials, Unauthenticated};
    use student_management::api::application::AuthGateway;

    #[derive(Clone)]
    struct StubAuthSource {
        base_url: Option<String>,
        seen_credentials: std::sync::Arc<std::sync::Mutex<Vec<(String, String)>>>,
    }

    impl AuthSource for StubAuthSource {
        fn login(&self, username: &str, password: &str) -> anyhow::Result<SamClient<Authenticated>>
        {
            self.seen_credentials
                .lock()
                .expect("Lock should be acquired")
                .push((username.to_owned(), password.to_owned()));

            match &self.base_url {
                Some(uri) => Ok(SamClient::<Unauthenticated>::new(uri.clone())
                    .expect("Client should be created")
                    .login(&SamCredentials {
                        login: username.to_owned(),
                        password: password.to_owned(),
                    })
                    .expect("Stub login against the mock server should succeed")),
                None => Err(anyhow::anyhow!("Invalid credentials")),
            }
        }
    }

    #[test]
    fn given_valid_credentials_gateway_should_succeed_and_forward_them() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;
            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;

            let seen = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
            let gateway = SamAuthGateway::new(StubAuthSource {
                base_url: Some(mock_server.uri()),
                seen_credentials: seen.clone(),
            });

            gateway
                .login("user".to_owned(), "pass".to_owned())
                .await
                .expect("Login should succeed");

            assert_eq!(
                *seen.lock().expect("Lock should be acquired"),
                vec![("user".to_owned(), "pass".to_owned())]
            );
        });
    }

    #[test]
    fn given_invalid_credentials_gateway_should_propagate_error() {
        smol::block_on(async {
            let gateway = SamAuthGateway::new(StubAuthSource {
                base_url: None,
                seen_credentials: Default::default(),
            });

            let result = gateway.login("u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid credentials"));
        });
    }

    #[test]
    fn given_login_helper_should_return_authenticated_client() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;

            let client = SamClient::<Unauthenticated>::new(mock_server.uri())
                .expect("Client should be created");

            let authenticated =
                login(client, "u".to_owned(), "p".to_owned()).await.expect("Login should succeed");

            let students_probe = authenticated.clone();
            let _ = students_probe; // session carried by cookie store

            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/painel"))
                .respond_with(wiremock::ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/alunos/listagem"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200)
                        .set_body_string(r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#),
                )
                .mount(&mock_server)
                .await;

            let result = crate::ports::RosterSource::students(&authenticated);

            assert!(result.is_ok(), "Authenticated client should be usable");
        });
    }

    #[test]
    fn given_login_helper_with_bad_credentials_should_fail() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                    "<p>* Oops... O usuário ou senha incorretos!</p>",
                ))
                .mount(&mock_server)
                .await;

            let client = SamClient::<Unauthenticated>::new(mock_server.uri())
                .expect("Client should be created");

            let result = login(client, "u".to_owned(), "wrong".to_owned()).await;

            assert!(result.is_err());
        });
    }
}
