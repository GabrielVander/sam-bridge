pub(crate) mod features;

pub mod api {
    pub mod adapters {
        pub use crate::features::basic_sam_site_interop::adapters::gateways;
    }

    pub mod infrastructure {
        pub use crate::features::basic_sam_site_interop::infra::sam_client::SamClient;
    }
}
