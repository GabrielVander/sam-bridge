mod api;
mod bootstrap;
mod frb_generated; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */

pub use api::build_main_application;

pub mod infra {
    pub use crate::bootstrap::infra::{Application, Config};
}
