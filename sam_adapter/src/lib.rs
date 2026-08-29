mod facade;
pub mod gateways;
mod mapping;
pub mod session_opener;

mod authentication;
mod students;
pub use facade::{AuthSession, authenticate};
pub use gateways::SamGateways;
pub use session_opener::SamAuthGateway;

pub mod adapters {
    pub mod gateways {
        pub use crate::authentication::adapters::gateways::*;
        pub use crate::students::infra::gateways::*;
    }
}
