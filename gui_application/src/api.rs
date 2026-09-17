use crate::infra::{ApplicationFacade, Config};

pub fn build_main_application() -> Result<ApplicationFacade, String> {
    let config: Config = Config {
        sam_client_base_url: "https://musical.congregacao.org.br".to_string(),
        sam_auth_endpoint: "/autenticar".to_string(),
        sam_dashboard_endpoint: "painel".to_string(),
        sam_students_listing_endpoint: "alunos/listagem".to_string(),
        sam_student_lessons_endpoint: "licoes/index".to_string(),
    };

    ApplicationFacade::new(&config)
}

#[cfg(test)]
mod tests {
    use super::build_main_application;

    #[test]
    fn builds_successfully_with_the_default_configuration() {
        let base = tempfile::tempdir().expect("tempdir");

        unsafe {
            std::env::set_var("XDG_DATA_HOME", base.path());
        }

        let result = build_main_application();

        unsafe {
            std::env::remove_var("XDG_DATA_HOME");
        }

        assert!(result.is_ok());
    }
}
