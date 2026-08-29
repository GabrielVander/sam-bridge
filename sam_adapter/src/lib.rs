mod facade;
pub mod gateways;
mod mapping;
pub mod session_opener;

mod students;
pub use facade::{AuthSession, authenticate};
pub use gateways::SamGateways;
pub use session_opener::SamAuthGateway;

pub mod infra {
    pub mod gateways {
        pub use crate::students::infra::gateways::StudentsGatewaySamImpl;
    }
}
