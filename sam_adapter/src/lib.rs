mod authentication;

pub mod adapters {
    pub mod gateways {
        pub use crate::authentication::adapters::gateways::*;
    }
}
