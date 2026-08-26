mod authentication;
mod student_lessons;
mod student_roster;

pub mod application {
    pub mod gateways {
        pub use crate::authentication::application::gateways::*;
        pub use crate::student_lessons::application::gateways::*;
        pub use crate::student_roster::application::gateways::*;
    }

    pub mod use_cases {
        pub use crate::authentication::application::use_cases::*;
        pub use crate::student_roster::application::use_cases::*;
    }
}

pub mod domain {
    pub mod entities {
        pub use crate::student_lessons::domain::entities::*;
        pub use crate::student_roster::domain::entities::*;
    }
}
