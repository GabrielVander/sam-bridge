use async_trait::async_trait;
use reqwest::StatusCode;
use std::collections::HashMap;
use std::sync::OnceLock;

use student_management::domain::{
    entities::{MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition},
    gateway::StudentGateway,
};

#[derive(thiserror::Error, Debug)]
pub enum AdapterError {
    #[error("Network or request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Authentication failed")]
    AuthFailed,
    #[error("No session ID was returned from the server")]
    MissingSessionId,
    #[error("Expected network response status {expected}, but got {actual}")]
    UnexpectedStatus {
        expected: StatusCode,
        actual: StatusCode,
    },
    #[error("Missing required field in response: {0}")]
    MissingField(&'static str),
    #[error("Failed to parse data: {0}")]
    ParseError(String),
}

pub struct SamSiteAdapter {
    client: reqwest::Client,
    base_url: String,
    session_id: Option<String>,
}

impl SamSiteAdapter {
    pub fn new(base_url: &str) -> Result<Self, AdapterError> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true) // Automatically handles the session cookie
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
            session_id: None,
        })
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<String, AdapterError> {
        let session_id = self.perform_auth_request(user, password).await?;
        self.session_id = Some(session_id.clone());
        Ok(session_id)
    }

    async fn perform_auth_request(
        &self,
        user: &str,
        password: &str,
    ) -> Result<String, AdapterError> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("login", user);
        form.insert("password", password);

        let url = format!("{}/autenticar", self.base_url);
        let response = self.client.post(&url).form(&form).send().await?;

        if response.status() == StatusCode::SEE_OTHER {
            response
                .cookies()
                .last()
                .map(|c| c.value().to_owned())
                .ok_or(AdapterError::MissingSessionId)
        } else {
            Err(AdapterError::AuthFailed)
        }
    }

    pub async fn get_students(&self) -> Result<Vec<Student>, AdapterError> {
        // Visit the main panel first to set any necessary state/cookies
        self.visit_main_page().await?;
        let response_json = self.retrieve_student_listing().await?;
        response_json.to_entity()
    }

    async fn retrieve_student_listing(&self) -> Result<StudentResponseJson, AdapterError> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("start", "0");
        form.insert("length", "999999999");
        form.insert("search[value]", "");
        form.insert("search[regex]", "false");

        let url = format!("{}/alunos/listagem", self.base_url);
        let referer_url = format!("{}/alunos", self.base_url);

        let request = self
            .client
            .post(&url)
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", referer_url)
            .form(&form);

        let response = request.send().await?;
        let json = response.json::<StudentResponseJson>().await?;
        Ok(json)
    }

    async fn visit_main_page(&self) -> Result<(), AdapterError> {
        let url = format!("{}/painel", self.base_url);
        let response = self.client.get(&url).send().await?;

        let actual_status = response.status();
        if actual_status != StatusCode::OK {
            return Err(AdapterError::UnexpectedStatus {
                expected: StatusCode::OK,
                actual: actual_status,
            });
        }

        Ok(())
    }
}

#[async_trait]
impl StudentGateway for SamSiteAdapter {
    async fn login(&self, username: String, password: String) -> Result<(), String> {
        self.login(username, password).await
    }

    async fn get_avaliable_records(&self) -> Result<Vec<Student>, String> {
        self.get_students().await.map_err(|e| e.to_string())
    }
}

#[derive(serde::Deserialize, Debug)]
struct StudentResponseJson {
    data: Vec<Vec<Option<String>>>,
}

impl StudentResponseJson {
    pub fn to_entity(&self) -> Result<Vec<Student>, AdapterError> {
        self.data
            .iter()
            .map(|raw_student| self.parse_student(raw_student))
            .collect()
    }

    fn parse_student(&self, raw_data: &[Option<String>]) -> Result<Student, AdapterError> {
        let id = raw_data
            .first()
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("ID"))?
            .to_owned();

        let name = raw_data
            .get(1)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Name"))?
            .to_owned();

        let raw_location = raw_data
            .get(2)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Location"))?;

        let cleaned_location = remove_double_or_more_spaces(&remove_span_tags(raw_location));
        let (location, region) = self.parse_location_and_region(&cleaned_location)?;

        let raw_position = raw_data
            .get(3)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Position"))?;

        let raw_level = raw_data
            .get(5)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Level"))?;

        let position = self.parse_student_position(raw_position, raw_level);

        Ok(Student {
            id,
            name,
            location,
            region,
            position,
        })
    }

    fn parse_location_and_region(&self, value: &str) -> Result<(String, Region), AdapterError> {
        let mut parts = value.split('|');

        let location = parts
            .next()
            .ok_or_else(|| AdapterError::ParseError("Missing location data".to_string()))?
            .trim()
            .to_owned();

        let raw_region = parts
            .next()
            .ok_or_else(|| AdapterError::ParseError("Missing region data".to_string()))?
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

fn remove_span_tags(input: &str) -> String {
    static SPAN_RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = SPAN_RE.get_or_init(|| regex::Regex::new(r"<span[^>]*></span>").unwrap());
    re.replace_all(input, "").to_string()
}

fn remove_double_or_more_spaces(input: &str) -> String {
    static SPACES_RE: OnceLock<regex::Regex> = OnceLock::new();
    let re = SPACES_RE.get_or_init(|| regex::Regex::new(r"\s{2,}").unwrap());
    re.replace_all(input, " ").to_string()
}
