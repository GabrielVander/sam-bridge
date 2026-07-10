use sam_site::infrastructure::sam_site_adapter::SamSiteAdapter;

use crate::adapters::view_models::SingleStudentViewModel;

pub async fn retrieve_students_default(
    user: String,
    pass: String,
) -> Result<Vec<SingleStudentViewModel>, String> {
    let mut adapter: SamSiteAdapter = SamSiteAdapter::new("https://musical.congregacao.org.br/")
        .map_err(|e| format!("{:#?}", e))?;

    let _session_id: String = adapter
        .login(&user, &pass)
        .await
        .map_err(|e| format!("{:#?}", e))?;

    Ok(adapter
        .get_students()
        .await
        .map_err(|e| format!("{:#?}", e))?
        .iter()
        .map(SingleStudentViewModel::from)
        .collect())
}
