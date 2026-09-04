mod lessons;
mod roster;

pub mod application {
    pub mod gateways {
        pub use crate::lessons::application::gateways::*;
        pub use crate::roster::application::gateways::*;
    }

    pub mod use_cases {
        pub use crate::roster::application::use_cases::*;
    }
}

pub mod domain {
    pub mod entities {
        pub use crate::lessons::domain::entities::*;
        pub use crate::roster::domain::entities::*;
    }
}
