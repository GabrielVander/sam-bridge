use anyhow::Result;
use student_management_sam_adapter::{session_opener::SessionOpener, SamClient};

pub async fn login<O>(opener: O, base_url: String, username: String, password: String) -> Result<SamClient>
where
    O: SessionOpener + Send + Sync + 'static,
{
    // sam is blocking: run on smol's thread pool.
    smol::unblock(move || opener.open(&base_url, &username, &password)).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[derive(Clone)]
    struct FakeOpener {
        fail: bool,
        seen: Arc<Mutex<Vec<(String, String, String)>>>,
    }

    impl SessionOpener for FakeOpener {
        fn open(&self, base_url: &str, username: &str, password: &str) -> Result<SamClient> {
            self.seen
                .lock()
                .expect("lock")
                .push((base_url.to_owned(), username.to_owned(), password.to_owned()));

            if self.fail {
                anyhow::bail!("Invalid credentials");
            }
            // Anonymous clients are network-free to fabricate.
            SamClient::new("http://127.0.0.1:1")
        }
    }

    #[test]
    fn forwards_credentials_and_returns_the_opened_client() {
        smol::block_on(async {
            let seen = Arc::new(Mutex::new(Vec::new()));
            let opener = FakeOpener {
                fail: false,
                seen: seen.clone(),
            };

            let client = login(
                opener,
                "http://sam.test".to_owned(),
                "user".to_owned(),
                "pass".to_owned(),
            )
            .await
            .expect("should succeed");

            assert_eq!(
                *seen.lock().expect("lock"),
                vec![(
                    "http://sam.test".to_owned(),
                    "user".to_owned(),
                    "pass".to_owned()
                )]
            );

            // The returned handle is a usable (anonymous) client type-wise.
            let _: SamClient = client;
        });
    }

    #[test]
    fn propagates_opener_errors() {
        smol::block_on(async {
            let opener = FakeOpener {
                fail: true,
                seen: Arc::default(),
            };

            let result =
                login(opener, "http://sam.test".to_owned(), "u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid credentials"));
        });
    }
}
