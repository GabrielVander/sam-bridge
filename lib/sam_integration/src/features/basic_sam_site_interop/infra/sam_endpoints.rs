pub struct SamEndpoints;

impl SamEndpoints {
    pub fn auth(base: &str) -> String {
        format!("{}/autenticar", base)
    }
    pub fn dashboard(base: &str) -> String {
        format!("{}/painel", base)
    }
    pub fn student_listing(base: &str) -> String {
        format!("{}/alunos/listagem", base)
    }
    pub fn student_referer(base: &str) -> String {
        format!("{}/alunos", base)
    }
    pub fn student_lessons(base: &str, id: &str) -> String {
        format!("{}/licoes/index/{}", base, id)
    }
}
