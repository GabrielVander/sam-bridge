use std::sync::Arc;
use student::{
    application::gateways::{StudentGateway, StudentGatewayError},
    domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

use crate::client::{SamClient, SamStudent};
use crate::diagnostics::{error_chain, failure_kind};

pub struct StudentGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl StudentGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}
impl StudentGateway for StudentGatewaySamImpl {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        let sam_students: Vec<SamStudent> = self.client.students().map_err(|error| {
            StudentGatewayError::UnableToPerformOperation {
                kind: failure_kind(&error),
                details: error_chain(&error),
            }
        })?;

        Ok(sam_students.into_iter().map(Student::from).collect())
    }
}

impl From<SamStudent> for Student {
    fn from(sam_student: SamStudent) -> Self {
        let position: StudentPosition = parse_position(
            &sam_student.role,
            &sam_student.level,
            &sam_student.instrument,
        );
        let region: Region = parse_region(&sam_student.location);
        let location: String = clean_location(&sam_student.location);

        Self {
            id: sam_student.id,
            name: sam_student.name,
            position,
            location,
            region,
        }
    }
}

fn clean_location(raw: &str) -> String {
    let mut without_tags: String = String::with_capacity(raw.len());
    let mut inside_tag: bool = false;

    for ch in raw.chars() {
        match ch {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => without_tags.push(ch),
            _ => {}
        }
    }

    without_tags
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_position(role: &str, level: &str, instrument: &str) -> StudentPosition {
    match role {
        "MÚSICO" => {
            let (instrument, instrument_name) = parse_musician_instrument(instrument);

            StudentPosition::Musician {
                level: parse_musician_level(level),
                instrument,
                instrument_name,
            }
        }
        "ORGANISTA" => StudentPosition::Organist {
            level: parse_organist_level(level),
        },
        "SECRETÁRIO DO GEM" => StudentPosition::Secretary {
            r#type: SecretaryType::Gem,
        },
        other => StudentPosition::Unknown(other.to_owned()),
    }
}

fn parse_musician_level(level: &str) -> MusicianLevel {
    match level {
        "CANDIDATO(A)" => MusicianLevel::Candidate,
        "ENSAIO" => MusicianLevel::Practice,
        "RJM" => MusicianLevel::YouthService,
        "CULTO OFICIAL" => MusicianLevel::OfficialService,
        other => MusicianLevel::Unknown(other.to_owned()),
    }
}

fn parse_organist_level(level: &str) -> OrganistLevel {
    match level {
        "CANDIDATO(A)" => OrganistLevel::Candidate,
        "ENSAIO" => OrganistLevel::Practice,
        "RJM" => OrganistLevel::YouthService,
        "CULTO OFICIAL" => OrganistLevel::OfficialService,
        "RJM / MEIA HORA" => OrganistLevel::YouthServiceHalfHour,
        other => OrganistLevel::Unknown(other.to_owned()),
    }
}

/// Both values come from the same trimmed text, so they are always both present or both absent.
fn parse_musician_instrument(raw: &str) -> (Option<Instrument>, Option<String>) {
    let instrument: Option<Instrument> = parse_instrument(raw);
    let instrument_name: Option<String> = instrument.as_ref().map(|_| raw.trim().to_owned());

    (instrument, instrument_name)
}

fn parse_instrument(instrument: &str) -> Option<Instrument> {
    match instrument.trim() {
        "" | "A DEFINIR" => None,
        "VIOLINO" => Some(Instrument::Violin),
        "VIOLA" => Some(Instrument::Viola),
        "VIOLONCELO" => Some(Instrument::Cello),
        "FLAUTA" => Some(Instrument::Flute),
        "OBOÉ" => Some(Instrument::Oboe),
        "FAGOTE" => Some(Instrument::Bassoon),
        "CLARINETE" => Some(Instrument::Clarinet),
        "CLARINETE ALTO" => Some(Instrument::AltoClarinet),
        "CLARINETE BAIXO" => Some(Instrument::BassClarinet),
        "SAXOFONE ALTO" | "SAXOFONE SOPRANO CUR" | "SAXOFONE SOPRANO RET" | "SAXOFONE TENOR" => {
            Some(Instrument::Saxophone)
        }
        "TROMPETE" | "CORNET" | "FLUGELHORN" => Some(Instrument::Trumpet),
        "TROMPA" => Some(Instrument::FrenchHorn),
        "TROMBONE" => Some(Instrument::Trombone),
        "EUPHONIUM" => Some(Instrument::Euphonium),
        "TUBA" => Some(Instrument::Tuba),
        "CORNE INGLÊS" => Some(Instrument::EnglishHorn),
        "VIOLINO CONTRALTO" => Some(Instrument::ContraltoViolin),
        other => Some(Instrument::Unknown(other.to_owned())),
    }
}

fn parse_region(location: &str) -> Region {
    if location.contains("ARARAQUARA-SÃO CARLOS") {
        Region::AraraquaraSaoCarlos
    } else if location.contains("ARARAQUARA-ITIRAPINA") {
        Region::AraraquaraItirapina
    } else {
        Region::Other(location.to_owned())
    }
}
