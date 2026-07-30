#[derive(serde::Deserialize, Debug)]
pub(crate) struct StudentListingJson {
    pub data: Vec<StudentDataPackJson>,
}

pub(crate) type StudentDataPackJson = Vec<String>;
