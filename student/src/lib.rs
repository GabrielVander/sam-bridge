mod lessons;
mod roster;
mod shared;

pub mod application {
    pub mod dto {
        pub use crate::lessons::application::dto::*;
        pub use crate::roster::application::dto::*;
    }

    pub mod gateways {
        pub use crate::lessons::application::gateways::*;
        pub use crate::roster::application::gateways::*;
        pub use shared_kernel::failure_kind::FailureKind;
    }

    pub mod use_cases {
        pub use crate::lessons::application::use_cases::*;
        pub use crate::roster::application::use_cases::*;
    }
}

pub mod domain {
    pub mod entities {
        pub use crate::lessons::domain::entities::*;
        pub use crate::roster::domain::entities::*;
        pub use crate::shared::domain::entities::*;
    }
}
