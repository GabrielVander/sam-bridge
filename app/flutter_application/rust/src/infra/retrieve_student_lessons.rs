use sam_site::infrastructure::sam_site_adapter::SamSiteAdapter;
use student_management::domain::gateway::StudentGateway;

use crate::adapters::view_models::SingleLessonViewModel;

pub async fn retrieve_student_lessons(
    user: String,
    pass: String,
    student: String,
) -> Result<Vec<SingleLessonViewModel>, String> {
    let adapter: SamSiteAdapter = SamSiteAdapter::new("https://musical.congregacao.org.br/")
        .map_err(|e| format!("{:#?}", e))?;

    let _session_id: String = adapter
        .login(&user, &pass)
        .await
        .map_err(|e| format!("{:#?}", e))?;

    Ok(adapter
        .get_all_lessons_for_student_with_id(&student)
        .await
        .map_err(|e| format!("{:#?}", e))?
        .iter()
        .map(SingleLessonViewModel::from)
        .collect())
}
