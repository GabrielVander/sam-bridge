#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoginInput {
    pub baseUrl: String,
    pub username: String,
    pub password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoginOutput;
