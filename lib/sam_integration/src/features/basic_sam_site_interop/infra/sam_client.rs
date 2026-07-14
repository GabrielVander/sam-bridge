use anyhow::Context;
use reqwest::{Client, StatusCode};
use std::collections::HashMap;

use student_management::features::{
    student_lessons::domain::entities::Lesson, student_roster::domain::entities::Student,
};

use crate::features::basic_sam_site_interop::infra::{
    lesson_parser::LessonParser, sam_endpoints::SamEndpoints, student_mapper::StudentResponseJson,
};

pub struct SamClient {
    client: Client,
    base_url: String,
}

impl SamClient {
    pub fn new(base_url: &str) -> anyhow::Result<Self> {
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()
            .context("Unable to instantiate HTTP client")?;

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
        })
    }

    pub async fn login(&self, user: &str, password: &str) -> anyhow::Result<String> {
        let mut form = HashMap::new();
        form.insert("login", user);
        form.insert("password", password);

        let response = self
            .client
            .post(SamEndpoints::auth(&self.base_url))
            .form(&form)
            .send()
            .await?;

        if response.status() == StatusCode::SEE_OTHER {
            response
                .cookies()
                .last()
                .map(|c| c.value().to_owned())
                .ok_or_else(|| anyhow::anyhow!("No session ID was returned from the server"))
        } else {
            Err(anyhow::anyhow!("Authentication failed"))
        }
    }

    pub async fn get_students(&self) -> anyhow::Result<Vec<Student>> {
        self.ensure_session_active().await?;

        self.fetch_student_listing().await?.try_into()
    }

    pub async fn get_student_lessons(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        let raw_html = self
            .client
            .get(SamEndpoints::student_lessons(&self.base_url, id))
            .send()
            .await
            .with_context(|| format!("Student's (#{}) lessons request failed", id))?
            .text()
            .await
            .with_context(|| format!("Unable to decode student's (#{}) lessons response", id))?;

        LessonParser::parse_html(&raw_html)
    }

    async fn ensure_session_active(&self) -> anyhow::Result<()> {
        let response = self
            .client
            .get(SamEndpoints::dashboard(&self.base_url))
            .send()
            .await?;

        if response.status() != StatusCode::OK {
            anyhow::bail!(
                "Session invalid or expired. Expected 200 OK, got {}",
                response.status()
            );
        }
        Ok(())
    }

    async fn fetch_student_listing(&self) -> anyhow::Result<StudentResponseJson> {
        let mut form = HashMap::new();
        form.insert("start", "0");
        form.insert("length", "999999999");
        form.insert("search[value]", "");
        form.insert("search[regex]", "false");

        let response = self
            .client
            .post(SamEndpoints::student_listing(&self.base_url))
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", SamEndpoints::student_referer(&self.base_url))
            .form(&form)
            .send()
            .await
            .context("Student listing request failed")?;

        response
            .json::<StudentResponseJson>()
            .await
            .context("Failed to decode JSON response")
    }
}
