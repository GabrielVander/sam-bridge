use crate::infra::{Application, Config};

pub fn build_main_application() -> Application {
    let config: Config = Config {
        sam_client_base_url: "https://musical.congregacao.org.br".to_string(),
    };

    Application::new(config)
}
