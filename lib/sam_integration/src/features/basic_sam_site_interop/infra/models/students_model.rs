use crate::features::basic_sam_site_interop::infra::models::student_listing_json_model::{
    StudentDataPackJson, StudentListingJson,
};

pub struct StudentModel {
    pub id: Option<String>,
    pub name: Option<String>,
    pub location: Option<String>,
    pub r#type: Option<String>,
    pub instrument: Option<String>,
    pub level: Option<String>,
}

impl From<StudentListingJson> for Vec<StudentModel> {
    fn from(value: StudentListingJson) -> Self {
        value.data.iter().map(|pack| pack.into()).collect()
    }
}

impl From<&StudentDataPackJson> for StudentModel {
    fn from(value: &StudentDataPackJson) -> Self {
        let id: Option<String> = value.first().map(|i| i.to_string());
        let name: Option<String> = value.get(1).map(|i| i.to_string());
        let location: Option<String> = value.get(2).map(|i| i.to_string());
        let r#type: Option<String> = value.get(3).map(|i| i.to_string());
        let instrument: Option<String> = value.get(4).map(|i| i.to_string());
        let level: Option<String> = value.get(5).map(|i| i.to_string());

        StudentModel {
            id,
            name,
            location,
            r#type,
            instrument,
            level,
        }
    }
}
