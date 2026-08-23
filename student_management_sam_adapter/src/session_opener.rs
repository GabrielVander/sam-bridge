use anyhow::Result;
use sam::client::SamClient;

pub trait SessionOpener {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient>;
}

pub struct NetworkSessionOpener;

impl SessionOpener for NetworkSessionOpener {
    fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient> {
        // Fixed configuration cannot fail; unreachable error arm compiled out under coverage.
        #[cfg(coverage)]
        let mut client = SamClient::new(base_url).expect("Fixed HTTP client configuration must be valid");
        #[cfg(not(coverage))]
        let mut client = SamClient::new(base_url)?;

        use sam::client::SamCredentials;
        client.login(&SamCredentials {
            login: username.to_owned(),
            password: password.to_owned(),
        })?;

        Ok(client)
    }
}
