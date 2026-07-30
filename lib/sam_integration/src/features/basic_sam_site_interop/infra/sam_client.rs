use std::collections::HashMap;

use anyhow::Context;
use anyhow::Result;
use anyhow::bail;

use crate::features::basic_sam_site_interop::infra::models::student_listing_json_model::StudentListingJson;
use crate::features::basic_sam_site_interop::infra::models::students_model::StudentModel;
use crate::features::basic_sam_site_interop::infra::{
    lesson_parser::MsaLessonHtmlParser, models::msa_lesson_model::MsaLessonModel,
    sam_endpoints::SamEndpoints,
};

pub struct SamClient {
    client: reqwest::Client,
    base_url: String,
}

impl SamClient {
    pub fn new(base_url: &str) -> Result<Self> {
        let client: reqwest::Client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .context("Unable to instantiate HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
        })
    }

    pub async fn login(&self, user: &str, password: &str) -> Result<String> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("login", user);
        form.insert("password", password);

        let response: reqwest::Response = self
            .client
            .post(SamEndpoints::auth(&self.base_url))
            .form(&form)
            .send()
            .await
            .context("Login request failed")?;

        if response.status() == reqwest::StatusCode::SEE_OTHER {
            response
                .cookies()
                .last()
                .map(|c| c.value().to_owned())
                .ok_or_else(|| anyhow::anyhow!("No session ID was returned from the server"))
        } else {
            Err(anyhow::anyhow!("Invalid credentials"))
        }
    }

    pub async fn get_students(&self) -> Result<Vec<StudentModel>> {
        self.ensure_session_active().await?;

        self.fetch_student_listing().await.map(|i| i.into())
    }

    pub async fn get_student_lessons(&self, id: &str) -> Result<Vec<MsaLessonModel>> {
        let msa_parser: MsaLessonHtmlParser =
            MsaLessonHtmlParser::new().context("Unable to instantiate MSA lesson HTML parser")?;

        let raw_html: String = self
            .client
            .get(SamEndpoints::student_lessons(&self.base_url, id))
            .send()
            .await
            .with_context(|| format!("Student's (#{}) lessons request failed", id))?
            .text()
            .await
            .with_context(|| format!("Unable to decode student's (#{}) lessons response", id))?;

        msa_parser.parse(&raw_html)
    }

    async fn ensure_session_active(&self) -> Result<()> {
        let response = self
            .client
            .get(SamEndpoints::dashboard(&self.base_url))
            .send()
            .await?;

        if response.status() != reqwest::StatusCode::OK {
            bail!(
                "Session invalid or expired. Expected 200 OK, got {}",
                response.status()
            );
        }

        Ok(())
    }

    async fn fetch_student_listing(&self) -> Result<StudentListingJson> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("start", "0");
        form.insert("length", "999999999");
        form.insert("search[value]", "");
        form.insert("search[regex]", "false");

        self.client
            .post(SamEndpoints::student_listing(&self.base_url))
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", SamEndpoints::student_referer(&self.base_url))
            .form(&form)
            .send()
            .await
            .context("Student listing request failed")?
            .json::<StudentListingJson>()
            .await
            .context("Failed to decode JSON response")
    }
}
