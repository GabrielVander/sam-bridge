use anyhow::{Context, bail};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SamStudent {
    pub id: String,
    pub name: String,
    pub location: String,
    pub role: String,
    pub instrument: String,
    pub level: String,
}

pub fn parse_students_listing(
    response_status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Result<Vec<SamStudent>> {
    if response_status != reqwest::StatusCode::OK {
        bail!("Unexpected status for student listing response: {response_status:?}");
    }

    let response: SamStudentsJsonResponse =
        serde_json::from_str(body).context("Unable to decode student listing JSON response")?;

    Ok(response.data.iter().map(SamStudent::from).collect())
}

#[derive(serde::Deserialize, Debug)]
struct SamStudentsJsonResponse {
    data: Vec<SamSingleStudentJsonResponse>,
}

type SamSingleStudentJsonResponse = Vec<String>;

impl From<&SamSingleStudentJsonResponse> for SamStudent {
    fn from(value: &SamSingleStudentJsonResponse) -> Self {
        let id: String = value.first().map_or(String::new(), Clone::clone);
        let name: String = value.get(1).map_or(String::new(), Clone::clone);
        let location: String = value.get(2).map_or(String::new(), Clone::clone);
        let role: String = value.get(3).map_or(String::new(), Clone::clone);
        let instrument: String = value.get(4).map_or(String::new(), Clone::clone);
        let level: String = value.get(5).map_or(String::new(), Clone::clone);

        Self {
            id,
            name,
            location,
            role,
            instrument,
            level,
        }
    }
}
