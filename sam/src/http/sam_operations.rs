use thiserror::Error;

#[derive(Debug, Clone)]
pub struct SamOperations {
    client: reqwest::blocking::Client,
    authentication_url: String,
    dashboard_url: String,
    students_listing_url: String,
    student_lessons_base_url: String,
}

impl SamOperations {
    #[must_use]
    pub fn new(
        client: reqwest::blocking::Client,
        base_url: &str,
        authentication_endpoint: &str,
        dashboard_endpoint: &str,
        students_listing_endpoint: &str,
        student_lessons_endpoint: &str,
    ) -> Self {
        let normalized_base_url: &str = base_url.trim_end_matches('/');

        let authentication_url: String = format!("{normalized_base_url}/{authentication_endpoint}");
        let dashboard_url: String = format!("{normalized_base_url}/{dashboard_endpoint}");
        let students_listing_url: String =
            format!("{normalized_base_url}/{students_listing_endpoint}");
        let student_lessons_base_url: String =
            format!("{normalized_base_url}/{student_lessons_endpoint}");

        Self {
            client,
            authentication_url,
            dashboard_url,
            students_listing_url,
            student_lessons_base_url,
        }
    }

    pub(crate) fn authenticate(
        &self,
        login: &str,
        password: &str,
    ) -> Result<SamResponse, SamOperationError> {
        Self::execute(
            self.client
                .post(self.authentication_url.clone())
                .form(&[("login", login), ("password", password)]),
            "authentication",
        )
    }

    pub(crate) fn dashboard(&self) -> Result<SamResponse, SamOperationError> {
        Self::execute(self.client.get(self.dashboard_url.clone()), "dashboard")
    }

    pub(crate) fn students_listing(&self) -> Result<SamResponse, SamOperationError> {
        Self::execute(
            self.client.get(self.students_listing_url.clone()),
            "students_listing",
        )
    }

    pub(crate) fn student_lessons(
        &self,
        student_id: &str,
    ) -> Result<SamResponse, SamOperationError> {
        Self::execute(
            self.client
                .get(format!("{}/{student_id}", self.student_lessons_base_url)),
            "student_lessons",
        )
    }

    fn execute(
        request: reqwest::blocking::RequestBuilder,
        operation: &str,
    ) -> Result<SamResponse, SamOperationError> {
        request
            .send()
            .map_err(|e| SamOperationError::RequestError {
                source: e,
                operation: operation.to_string(),
            })
            .and_then(|r| {
                SamResponse::from_reqwest_response(r).map_err(|e| SamOperationError::DecodeError {
                    source: e,
                    operation: operation.to_string(),
                })
            })
    }
}

pub struct SamResponse {
    pub status: u16,
    pub body: String,
}

impl SamResponse {
    fn from_reqwest_response(
        response: reqwest::blocking::Response,
    ) -> Result<Self, reqwest::Error> {
        Ok(Self {
            status: response.status().as_u16(),
            body: response.text()?,
        })
    }
}

#[derive(Error, Debug)]
pub enum SamOperationError {
    #[error("Request failed for operation '{operation}'")]
    RequestError {
        source: reqwest::Error,
        operation: String,
    },
    #[error("Unable to decode response for operation '{operation}'")]
    DecodeError {
        source: reqwest::Error,
        operation: String,
    },
}
