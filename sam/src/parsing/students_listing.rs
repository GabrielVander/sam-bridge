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
        Self {
            id: column(value, 0),
            name: column(value, 1),
            location: column(value, 2),
            role: column(value, 3),
            instrument: column(value, 4),
            level: column(value, 5),
        }
    }
}

fn column(row: &SamSingleStudentJsonResponse, index: usize) -> String {
    row.get(index).cloned().unwrap_or_default()
}
