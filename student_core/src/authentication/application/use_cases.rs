use std::sync::Arc;

use crate::authentication::application::gateways::AuthGateway;

pub struct LoginUseCase {
    gateway: Arc<dyn AuthGateway>,
}

impl LoginUseCase {
    pub fn new(gateway: Arc<dyn AuthGateway>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, input: LoginInput) -> anyhow::Result<LoginOutput> {
        self.gateway.login(input.username, input.password).await?;
        Ok(LoginOutput)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginInput {
    pub base_url: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LoginOutput;

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
            let gateway = Arc::new(FakeAuthGateway::default());
            let use_case = LoginUseCase::new(gateway.clone());

            use_case
                .execute(LoginInput {
                    base_url: String::new(),
                    username: "user".to_owned(),
                    password: "pass".to_owned(),
                })
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
            let gateway = Arc::new(FakeAuthGateway {
                fail: true,
                ..Default::default()
            });
            let use_case = LoginUseCase::new(gateway);

            let result = use_case
                .execute(LoginInput {
                    base_url: String::new(),
                    username: "u".to_owned(),
                    password: "p".to_owned(),
                })
                .await;

            assert!(result.is_err());
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .contains("Invalid credentials")
            );
        });
    }
}
