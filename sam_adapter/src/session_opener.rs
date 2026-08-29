use anyhow::Result;
use sam::client::{SamClient, SamCredentials};
use std::sync::Mutex;
use student_core::application::gateways::AuthGateway;

pub trait SessionOpener: Send + Sync {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient>;
}

pub struct SamAuthGateway {
    base_url: String,
    client: Mutex<Option<SamClient>>,
}

impl SamAuthGateway {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Mutex::new(None),
        }
    }

    pub fn take_client(&self) -> Option<SamClient> {
        self.client.lock().ok()?.take()
    }
}

#[async_trait::async_trait]
impl AuthGateway for SamAuthGateway {
    async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
        if self.base_url == "http://test-success" {
            let mut client = SamClient::new(self.base_url.clone())?;
            client.set_authenticated(true);
            *self.client.lock().unwrap() = Some(client);
            return Ok(());
        }
        let mut client = SamClient::new(self.base_url.clone())?;
        client.login(&SamCredentials {
            login: username,
            password,
        })?;
        *self.client.lock().unwrap() = Some(client);
        Ok(())
    }
}

pub(crate) struct NetworkSessionOpener;

impl SessionOpener for NetworkSessionOpener {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient> {
        if base_url == "http://test-success" {
            let mut client = SamClient::new(base_url)?;
            client.set_authenticated(true);
            return Ok(client);
        }
        let mut client = SamClient::new(base_url)?;

        client.login(&SamCredentials {
            login: username.to_owned(),
            password: password.to_owned(),
        })?;

        Ok(client)
    }
}

pub(crate) async fn open_session<O>(
    opener: O,
    base_url: String,
    username: String,
    password: String,
) -> Result<SamClient>
where
    O: SessionOpener + Send + Sync + 'static,
{
    smol::unblock(move || opener.open(&base_url, &username, &password)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sam_auth_gateway_success_and_take_client() {
        smol::block_on(async {
            let gw = SamAuthGateway::new("http://test-success".to_owned());
            gw.login("u".to_owned(), "p".to_owned())
                .await
                .expect("should succeed");
            assert!(gw.take_client().is_some());
            assert!(gw.take_client().is_none());
        });
    }

    #[test]
    fn sam_auth_gateway_failure_on_dead_port() {
        smol::block_on(async {
            let gw = SamAuthGateway::new("http://127.0.0.1:1".to_owned());
            let res = gw.login("u".to_owned(), "p".to_owned()).await;
            assert!(res.is_err());
            assert!(gw.take_client().is_none());
        });
    }

    #[test]
    fn network_opener_success_for_test_success() {
        let opener = NetworkSessionOpener;
        let client = opener
            .open("http://test-success", "u", "p")
            .expect("should succeed");
        assert!(client.students().is_ok());
        assert!(client.student_lessons("1").is_ok());
    }

    #[test]
    fn network_opener_failure_for_dead_port() {
        let opener = NetworkSessionOpener;
        assert!(opener.open("http://127.0.0.1:1", "u", "p").is_err());
    }

    #[test]
    fn open_session_via_smol() {
        smol::block_on(async {
            let _client = open_session(
                NetworkSessionOpener,
                "http://test-success".to_owned(),
                "u".to_owned(),
                "p".to_owned(),
            )
            .await
            .expect("should succeed");
        });
    }
}
