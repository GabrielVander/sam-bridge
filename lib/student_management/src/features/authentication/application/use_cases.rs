use crate::features::authentication::domain::gateways::AuthGateway;

pub struct LoginUseCase<'a, T: AuthGateway> {
    gateway: &'a T,
}

impl<'a, T: AuthGateway> LoginUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, username: String, password: String) -> Result<(), ()> {
        self.gateway
            .login(username, password)
            .await
            .inspect_err(|e| println!("{:#?}", e))
            .map_err(|_| ())
    }
}
