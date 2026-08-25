#![allow(non_snake_case)]

use flutter_rust_bridge::frb;

#[frb(non_opaque)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoginInput {
    pub baseUrl: String,
    pub username: String,
    pub password: String,
}

#[frb(non_opaque)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, PartialEq)]
pub struct LoginOutput;
