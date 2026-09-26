mod login;
mod logout;
mod remembered_credentials;
mod shared;

pub mod application {
    pub mod use_cases {
        pub use crate::login::application::use_cases::*;
        pub use crate::logout::application::use_cases::*;
        pub use crate::remembered_credentials::application::use_cases::*;
    }

    pub mod gateways {
        pub use crate::login::application::gateways::*;
        pub use crate::remembered_credentials::application::gateways::*;
        pub use shared_kernel::failure_kind::FailureKind;
    }
}

pub mod domain {
    pub mod entities {
        pub use crate::shared::domain::entities::*;
    }
}
