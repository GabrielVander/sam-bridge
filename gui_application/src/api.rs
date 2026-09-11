use crate::infra::{Application, Config};

pub fn build_main_application() -> Result<Application, String> {
    let config: Config = Config {
        sam_client_base_url: "https://musical.congregacao.org.br".to_string(),
        sam_auth_endpoint: "/autenticar".to_string(),
    };

    Application::new(&config)
}
