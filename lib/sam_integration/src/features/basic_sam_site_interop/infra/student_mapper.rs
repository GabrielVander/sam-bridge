use regex::Regex;
use std::{str::Split, sync::OnceLock};
use student_management::api::domain::{
    MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
};

#[derive(serde::Deserialize, Debug)]
pub struct StudentResponseJson {
    pub data: Vec<Vec<Option<String>>>,
}

impl TryInto<Vec<Student>> for StudentResponseJson {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Vec<Student>, Self::Error> {
        self.data
            .iter()
            .map(|raw| self.parse_student(raw))
            .collect()
    }
}

impl StudentResponseJson {
    fn parse_student(&self, raw_data: &[Option<String>]) -> anyhow::Result<Student> {
        let id = self.extract_field(raw_data, 0, "ID")?;
        let name = self.extract_field(raw_data, 1, "Name")?;

        let raw_location = self.extract_field(raw_data, 2, "Location")?;
        let cleaned_location = self.clean_location_string(&raw_location);
        let (location, region) = self.parse_location_and_region(&cleaned_location)?;

        let raw_position = self.extract_field(raw_data, 3, "Position")?;
        let raw_level = self.extract_field(raw_data, 5, "Level")?;
        let position = self.parse_student_position(&raw_position, &raw_level);

        Ok(Student {
            id,
            name,
            location,
            region,
            position,
        })
    }

    fn extract_field(
        &self,
        data: &[Option<String>],
        index: usize,
        field_name: &'static str,
    ) -> anyhow::Result<String> {
        data.get(index)
            .and_then(|opt| opt.as_ref())
            .map(|s| s.to_owned())
            .ok_or_else(|| anyhow::anyhow!(format!("Missing attribute {}", field_name)))
    }

    fn clean_location_string(&self, input: &str) -> String {
        static SPAN_RE: OnceLock<Regex> = OnceLock::new();
        static SPACES_RE: OnceLock<Regex> = OnceLock::new();

        let span_re = SPAN_RE.get_or_init(|| Regex::new(r"<span[^>]*></span>").unwrap());
        let spaces_re = SPACES_RE.get_or_init(|| Regex::new(r"\s{2,}").unwrap());

        let no_spans = span_re.replace_all(input, "");
        spaces_re.replace_all(&no_spans, " ").to_string()
    }

    fn parse_location_and_region(&self, value: &str) -> anyhow::Result<(String, Region)> {
        let mut parts: Split<char> = value.split('|');

        let location: String = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing location data"))?
            .trim()
            .to_owned();

        let raw_region: &str = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Missing region data"))?
            .trim();

        Ok((location, self.parse_region(raw_region)))
    }

    fn parse_region(&self, value: &str) -> Region {
        match value {
            "BR-SP-ARARAQUARA-SÃO CARLOS" => Region::AraraquaraSaoCarlos,
            "BR-SP-ARARAQUARA-ITIRAPINA" => Region::AraraquaraItirapina,
            _ => Region::Other(value.to_owned()),
        }
    }

    fn parse_student_position(&self, raw_position: &str, raw_level: &str) -> StudentPosition {
        match raw_position.to_uppercase().as_str() {
            "MÚSICO" => StudentPosition::Musician {
                level: self.parse_musician_level(raw_level),
            },
            "ORGANISTA" => StudentPosition::Organist {
                level: self.parse_organist_level(raw_level),
            },
            "SECRETÁRIO DO GEM" => StudentPosition::Secretary {
                r#type: SecretaryType::Gem,
            },
            "SECRETÁRIO DA MÚSICA" => StudentPosition::Secretary {
                r#type: SecretaryType::Music,
            },
            other => StudentPosition::Unknown(other.to_owned()),
        }
    }

    fn parse_musician_level(&self, value: &str) -> MusicianLevel {
        match value.to_uppercase().as_str() {
            "CANDIDATO(A)" => MusicianLevel::Candidate,
            "CULTO OFICIAL" => MusicianLevel::OfficialService,
            "ENSAIO" => MusicianLevel::Practice,
            "RJM" => MusicianLevel::YouthService,
            other => MusicianLevel::Unknown(other.to_owned()),
        }
    }

    fn parse_organist_level(&self, value: &str) -> OrganistLevel {
        match value.to_uppercase().as_str() {
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
