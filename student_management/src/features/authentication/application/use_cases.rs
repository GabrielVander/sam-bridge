use crate::api::application::AuthGateway;

pub struct LoginUseCase<'a, T: AuthGateway> {
    gateway: &'a T,
}

impl<'a, T: AuthGateway> LoginUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, username: String, password: String) -> anyhow::Result<()> {
        self.gateway.login(username, password).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeAuthGateway {
        seen: Mutex<Vec<(String, String)>>,
        fail: bool,
    }

    #[async_trait]
    impl AuthGateway for FakeAuthGateway {
        async fn login(&self, username: String, password: String) -> anyhow::Result<()> {
            self.seen.lock().expect("lock").push((username, password));
            if self.fail {
                anyhow::bail!("Invalid credentials");
            }
            Ok(())
        }
    }

    #[test]
    fn forwards_credentials_and_succeeds() {
        smol::block_on(async {
            let gateway = FakeAuthGateway::default();
            let use_case = LoginUseCase::new(&gateway);

            use_case
                .execute("user".to_owned(), "pass".to_owned())
                .await
                .expect("should succeed");

            assert_eq!(
                *gateway.seen.lock().expect("lock"),
                vec![("user".to_owned(), "pass".to_owned())]
            );
        });
    }

    #[test]
    fn propagates_gateway_errors() {
        smol::block_on(async {
            let gateway = FakeAuthGateway {
                fail: true,
                ..Default::default()
            };
            let use_case = LoginUseCase::new(&gateway);

            let result = use_case.execute("u".to_owned(), "p".to_owned()).await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Invalid credentials"));
        });
    }
}
