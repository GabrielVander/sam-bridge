use std::str::Split;
use std::sync::OnceLock;

use anyhow::{anyhow, Context};
use sam_integration::api::infrastructure::{SamClient, StudentModel};
use student_management::api::domain::{
    MusicianLevel, OrganistLevel, Region, SecretaryType, StudentPosition,
};
use student_management::api::{application::StudentsRetrievalGateway, domain::Student};

pub(crate) struct StudentSamGateway<'a> {
    sam_client: &'a SamClient,
}

impl<'a> StudentSamGateway<'a> {
    pub(crate) fn new(sam_client: &'a SamClient) -> Self {
        Self { sam_client }
    }
}

#[async_trait::async_trait]
impl<'a> StudentsRetrievalGateway for StudentSamGateway<'a> {
    async fn get_avaliable_records(&self) -> anyhow::Result<Vec<Student>> {
        self.sam_client
            .get_students()
            .await?
            .iter()
            .map(|i| i.try_into_student())
            .collect()
    }
}

trait TryIntoStudent {
    fn try_into_student(&self) -> anyhow::Result<Student>;
}

impl TryIntoStudent for StudentModel {
    fn try_into_student(&self) -> anyhow::Result<Student> {
        let id: String = self
            .id
            .clone()
            .ok_or(anyhow!("Expected id value but none was found"))?;
        let name: String = self
            .name
            .clone()
            .ok_or(anyhow!("Expected name value but none was found"))?;
        let position = self
            .try_into_student_position()
            .context("Unable to parse student position")?;
        let (location, region) = self
            .location
            .clone()
            .map(|i| i.remove_html_tags())
            .ok_or(anyhow!("Expected location value but none was found"))?
            .try_into_location_bundle()?;

        Ok(Student {
            id,
            name,
            position,
            location,
            region,
        })
    }
}

trait TryIntoStudentPosition {
    fn try_into_student_position(&self) -> anyhow::Result<StudentPosition>;
}

trait RemoveHtmlTags {
    fn remove_html_tags(&self) -> String;
}

trait TryIntoLocationBundle {
    fn try_into_location_bundle(self) -> anyhow::Result<(String, Region)>;
}

impl TryIntoStudentPosition for StudentModel {
    fn try_into_student_position(&self) -> anyhow::Result<StudentPosition> {
        let r#type: String = self
            .r#type
            .clone()
            .ok_or(anyhow!("Expected student type but none was given"))?;
        let level: String = self
            .level
            .clone()
            .ok_or(anyhow!("Expected student level but none was given"))?;

        match r#type.to_uppercase().as_str() {
            "MÚSICO" => Ok(StudentPosition::Musician {
                level: level.into_musician_level(),
            }),
            "ORGANISTA" => Ok(StudentPosition::Organist {
                level: level.into_organist_level(),
            }),
            "SECRETÁRIO DO GEM" => Ok(StudentPosition::Secretary {
                r#type: SecretaryType::Gem,
            }),
            "SECRETÁRIO DA MÚSICA" => Ok(StudentPosition::Secretary {
                r#type: SecretaryType::Music,
            }),
            other => Ok(StudentPosition::Unknown(other.to_owned())),
        }
    }
}

impl TryIntoLocationBundle for String {
    fn try_into_location_bundle(self) -> anyhow::Result<(String, Region)> {
        let mut parts: Split<char> = self.split('|');

        let location: String = parts
            .next()
            .ok_or_else(|| anyhow!("Missing location data"))?
            .trim()
            .to_owned();

        let raw_region: &str = parts
            .next()
            .ok_or_else(|| anyhow!("Missing region data"))?
            .trim();

        let region = match raw_region.to_uppercase().as_str() {
            "BR-SP-ARARAQUARA-SÃO CARLOS" => Region::AraraquaraSaoCarlos,
            "BR-SP-ARARAQUARA-ITIRAPINA" => Region::AraraquaraItirapina,
            _ => Region::Other(self.to_owned()),
        };

        Ok((location, region))
    }
}

impl RemoveHtmlTags for String {
    fn remove_html_tags(&self) -> String {
        static SPAN_RE: OnceLock<regex::Regex> = OnceLock::new();
        static SPACES_RE: OnceLock<regex::Regex> = OnceLock::new();

        let span_re = SPAN_RE.get_or_init(|| regex::Regex::new(r"<span[^>]*></span>").unwrap());
        let spaces_re = SPACES_RE.get_or_init(|| regex::Regex::new(r"\s{2,}").unwrap());

        let no_spans = span_re.replace_all(self, "");
        spaces_re.replace_all(&no_spans, " ").to_string()
    }
}

trait IntoMusicianLevel {
    fn into_musician_level(self) -> MusicianLevel;
}

trait IntoOrganistLevel {
    fn into_organist_level(self) -> OrganistLevel;
}

impl IntoMusicianLevel for String {
    fn into_musician_level(self) -> MusicianLevel {
        match self.to_uppercase().as_str() {
            "CANDIDATO(A)" => MusicianLevel::Candidate,
            "CULTO OFICIAL" => MusicianLevel::OfficialService,
            "ENSAIO" => MusicianLevel::Practice,
            "RJM" => MusicianLevel::YouthService,
            other => MusicianLevel::Unknown(other.to_owned()),
        }
    }
}

impl IntoOrganistLevel for String {
    fn into_organist_level(self) -> OrganistLevel {
        match self.to_uppercase().as_str() {
            "CANDIDATO(A)" => OrganistLevel::Candidate,
            "CULTO OFICIAL" => OrganistLevel::OfficialService,
            "ENSAIO" => OrganistLevel::Practice,
            "RJM" => OrganistLevel::YouthService,
            "MEIA HORA" => OrganistLevel::HafHour,
            "RJM / CULTO OFICIAL" => OrganistLevel::YouthServiceOfficialService,
            "RJM / ENSAIO" => OrganistLevel::YouthServicePractice,
            "RJM / MEIA HORA" => OrganistLevel::YouthServiceHafHour,
            "RJM / OFICIALIZADO(A)" => OrganistLevel::YouthServiceOfficialized,
            other => OrganistLevel::Unknown(other.to_owned()),
        }
    }
}
