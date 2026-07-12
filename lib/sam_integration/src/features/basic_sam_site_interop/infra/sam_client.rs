use std::{collections::HashMap, str::Split, sync::OnceLock};

use student_management::features::{
    student_lessons::domain::entities::{Clef, Lesson, Range},
    student_roster::domain::entities::{
        MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

pub struct SamClient {
    client: reqwest::Client,
    base_url: String,
}

impl SamClient {
    pub fn new(base_url: &str) -> Result<Self, AdapterError> {
        let client = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .cookie_store(true)
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.to_owned(),
        })
    }

    pub async fn login(&self, user: &str, password: &str) -> Result<String, AdapterError> {
        self.perform_auth_request(user, password).await
    }

    pub async fn get_students(&self) -> Result<Vec<Student>, AdapterError> {
        self.visit_main_page().await?;
        let response_json: StudentResponseJson = self.retrieve_student_listing().await?;
        response_json.to_entity()
    }

    pub async fn get_student_lessons(&self, id: &str) -> Result<Vec<Lesson>, AdapterError> {
        let url: String = format!("{}/licoes/index/{}", self.base_url, id);

        let raw_response: String = self.client.get(&url).send().await?.text().await?;

        let document: scraper::Html = scraper::Html::parse_document(&raw_response);

        let msa_row_selector: scraper::Selector =
            scraper::Selector::parse(r#"tr[id^="msa_"]"#).expect("Failed to parse row selector");
        let cell_selector: scraper::Selector =
            scraper::Selector::parse("td").expect("Failed to parse td selector");

        let mut lessons: Vec<Lesson> = Vec::new();

        for row in document.select(&msa_row_selector) {
            let id: String = row.value().attr("id").unwrap_or("unknown_id").to_string();
            let mut data: scraper::element_ref::Select = row.select(&cell_selector);

            let date: String = Self::extract_text(&mut data);
            let phase: Option<Range> = Self::parse_fragment(&Self::extract_text(&mut data));
            let page: Option<Range> = Self::parse_fragment(&Self::extract_text(&mut data));
            let lesson_frag: Option<Range> = Self::parse_fragment(&Self::extract_text(&mut data));
            let clef: Option<Clef> = Self::parse_clef(&Self::extract_text(&mut data));

            let desc_text: String = Self::extract_text(&mut data);
            let description: Option<String> = if desc_text.is_empty() {
                None
            } else {
                Some(desc_text)
            };

            let inst_text: String = Self::extract_text(&mut data);
            let instructor: Option<String> = if inst_text.is_empty() {
                None
            } else {
                Some(inst_text)
            };

            lessons.push(Lesson {
                id,
                date,
                phase,
                page,
                lesson: lesson_frag,
                clef,
                description,
                instructor,
            });
        }

        Ok(lessons)
    }

    async fn perform_auth_request(
        &self,
        user: &str,
        password: &str,
    ) -> Result<String, AdapterError> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("login", user);
        form.insert("password", password);

        let url: String = format!("{}/autenticar", self.base_url);
        let response: reqwest::Response = self.client.post(&url).form(&form).send().await?;

        if response.status() == reqwest::StatusCode::SEE_OTHER {
            response
                .cookies()
                .last()
                .map(|c| c.value().to_owned())
                .ok_or(AdapterError::MissingSessionId)
        } else {
            Err(AdapterError::AuthFailed)
        }
    }

    async fn visit_main_page(&self) -> Result<(), AdapterError> {
        let url: String = format!("{}/painel", self.base_url);
        let response: reqwest::Response = self.client.get(&url).send().await?;

        let actual_status: reqwest::StatusCode = response.status();
        if actual_status != reqwest::StatusCode::OK {
            return Err(AdapterError::UnexpectedStatus {
                expected: reqwest::StatusCode::OK,
                actual: actual_status,
            });
        }

        Ok(())
    }

    async fn retrieve_student_listing(&self) -> Result<StudentResponseJson, AdapterError> {
        let mut form: HashMap<&str, &str> = HashMap::new();
        form.insert("start", "0");
        form.insert("length", "999999999");
        form.insert("search[value]", "");
        form.insert("search[regex]", "false");

        let url: String = format!("{}/alunos/listagem", self.base_url);
        let referer_url: String = format!("{}/alunos", self.base_url);

        let response: reqwest::Response = self
            .client
            .post(&url)
            .header("X-Requested-With", "XMLHttpRequest")
            .header("Referer", referer_url)
            .form(&form)
            .send()
            .await?;

        let json: StudentResponseJson = response.json::<StudentResponseJson>().await?;
        Ok(json)
    }

    fn extract_text<'a, I>(iter: &mut I) -> String
    where
        I: Iterator<Item = scraper::ElementRef<'a>>,
    {
        iter.next()
            .map(|td| td.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .unwrap_or_default()
    }

    fn parse_fragment(val: &str) -> Option<Range> {
        if val.is_empty() {
            return None;
        }
        let parts: Vec<&str> = val.split('-').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            Some(Range {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            })
        } else if parts.len() == 1 {
            Some(Range {
                from: parts[0].to_string(),
                to: parts[0].to_string(),
            })
        } else {
            None
        }
    }

    fn parse_clef(val: &str) -> Option<Clef> {
        match val.to_uppercase().as_str() {
            "SOL" => Some(Clef::G),
            "FÁ" | "FA" => Some(Clef::F),
            "DÓ" | "DO" => Some(Clef::C),
            _ => None,
        }
    }
}

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
        expected: reqwest::StatusCode,
        actual: reqwest::StatusCode,
    },
    #[error("Missing required field in response: {0}")]
    MissingField(&'static str),
    #[error("Failed to parse data: {0}")]
    ParseError(String),
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
        let id: String = raw_data
            .first()
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("ID"))?
            .to_owned();

        let name: String = raw_data
            .get(1)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Name"))?
            .to_owned();

        let raw_location: &String = raw_data
            .get(2)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Location"))?;

        let cleaned_location: String =
            remove_double_or_more_spaces(&remove_span_tags(raw_location));
        let (location, region) = self.parse_location_and_region(&cleaned_location)?;

        let raw_position: &String = raw_data
            .get(3)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Position"))?;

        let raw_level: &String = raw_data
            .get(5)
            .and_then(|opt| opt.as_ref())
            .ok_or(AdapterError::MissingField("Level"))?;

        let position: StudentPosition = self.parse_student_position(raw_position, raw_level);

        Ok(Student {
            id,
            name,
            location,
            region,
            position,
        })
    }

    fn parse_location_and_region(&self, value: &str) -> Result<(String, Region), AdapterError> {
        let mut parts: Split<char> = value.split('|');

        let location: String = parts
            .next()
            .ok_or_else(|| AdapterError::ParseError("Missing location data".to_string()))?
            .trim()
            .to_owned();

        let raw_region: &str = parts
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
    let re: &regex::Regex =
        SPAN_RE.get_or_init(|| regex::Regex::new(r"<span[^>]*></span>").unwrap());
    re.replace_all(input, "").to_string()
}

fn remove_double_or_more_spaces(input: &str) -> String {
    static SPACES_RE: OnceLock<regex::Regex> = OnceLock::new();
    let re: &regex::Regex = SPACES_RE.get_or_init(|| regex::Regex::new(r"\s{2,}").unwrap());
    re.replace_all(input, " ").to_string()
}
