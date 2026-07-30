use anyhow::anyhow;
use anyhow::bail;
use anyhow::Context;
use sam_integration::api::infrastructure::{MsaLessonModel, SamClient};
use student_management::api::{
    application::StudentLessonsGateway,
    domain::{Clef, Lesson, Range},
};

pub(crate) struct StudentLessonSamGateway<'a> {
    sam_client: &'a SamClient,
}

impl<'a> StudentLessonSamGateway<'a> {
    pub(crate) fn new(sam_client: &'a SamClient) -> Self {
        Self { sam_client }
    }
}

#[async_trait::async_trait]
impl<'a> StudentLessonsGateway for StudentLessonSamGateway<'a> {
    async fn get_all_for_student_with_id(&self, id: &str) -> anyhow::Result<Vec<Lesson>> {
        self.sam_client
            .get_student_lessons(id)
            .await?
            .iter()
            .map(|i| i.try_into_lesson())
            .collect()
    }
}

trait TryIntoLesson {
    fn try_into_lesson(&self) -> anyhow::Result<Lesson>;
}

impl TryIntoLesson for &MsaLessonModel {
    fn try_into_lesson(&self) -> anyhow::Result<Lesson> {
        let id: String = self.id.clone().ok_or(anyhow!("Id not present"))?;
        let date: chrono::NaiveDate = self
            .date
            .clone()
            .ok_or(anyhow!("Date not present"))?
            .try_into_naive_date()?;
        let phase = self
            .phases
            .clone()
            .map(|i| i.try_into_range())
            .transpose()?;
        let page = self.pages.clone().map(|i| i.try_into_range()).transpose()?;
        let lesson = self
            .lessons
            .clone()
            .map(|i| i.try_into_range())
            .transpose()?;
        let clef = self.clefs.clone().map(|i| i.try_into_clef()).transpose()?;
        let description = self.description.clone();
        let instructor = self
            .authorizer
            .clone()
            .ok_or_else(|| anyhow!("Expected instructor value but none was found"))?;

        Ok(Lesson {
            id,
            date,
            phase,
            page,
            lesson,
            clef,
            description,
            instructor,
        })
    }
}

trait TryIntoNaiveDate {
    fn try_into_naive_date(&self) -> anyhow::Result<chrono::NaiveDate>;
}

trait TryIntoRange {
    fn try_into_range(&self) -> anyhow::Result<Range>;
}

trait TryIntoClef {
    fn try_into_clef(&self) -> anyhow::Result<Clef>;
}

impl TryIntoNaiveDate for String {
    fn try_into_naive_date(&self) -> anyhow::Result<chrono::NaiveDate> {
        chrono::NaiveDate::parse_from_str(self, "%d/%m/%Y")
            .map_err(anyhow::Error::from)
            .context("Failed to parse lesson date")
    }
}

impl TryIntoRange for String {
    fn try_into_range(&self) -> anyhow::Result<Range> {
        let mut parts = self.split('-').map(|i| i.to_string());

        let from: String = parts
            .next()
            .ok_or(anyhow!("Unable to parse range: '{}'", self))?;

        Ok(parts
            .next()
            .map(|to| Range {
                from: from.clone(),
                to,
            })
            .unwrap_or(Range {
                from: from.clone(),
                to: from,
            }))
    }
}

impl TryIntoClef for String {
    fn try_into_clef(&self) -> anyhow::Result<Clef> {
        match self.to_uppercase().as_str() {
            "SOL" => Ok(Clef::G),
            "DO" | "DÓ" => Ok(Clef::C),
            "FA" | "FÁ" => Ok(Clef::F),
            _ => bail!("Unable to parse clef value '{}'", self),
        }
    }
}
