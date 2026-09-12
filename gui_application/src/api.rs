use crate::infra::{ApplicationFacade, Config};

pub fn build_main_application() -> Result<ApplicationFacade, String> {
    let config: Config = Config {
        sam_client_base_url: "https://musical.congregacao.org.br".to_string(),
        sam_auth_endpoint: "/autenticar".to_string(),
    };

    ApplicationFacade::new(&config)
}
