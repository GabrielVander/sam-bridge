use anyhow::Result;
use sam::client::{SamClient, SamCredentials};

pub(crate) trait SessionOpener {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient>;
}

pub(crate) struct NetworkSessionOpener;

impl SessionOpener for NetworkSessionOpener {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient> {
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
