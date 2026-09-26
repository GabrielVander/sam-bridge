use std::sync::Arc;
use student::{
    application::gateways::{StudentGateway, StudentGatewayError},
    domain::entities::{
        Instrument, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

use crate::client::{SamClient, SamStudent};
use crate::diagnostics::{error_chain, failure_kind};
use crate::shared::musicians::{MUSICIAN_ROLE, parse_instrument, parse_musician_level};

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
        MUSICIAN_ROLE => {
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

fn parse_musician_instrument(raw: &str) -> (Option<Instrument>, Option<String>) {
    let instrument: Option<Instrument> = parse_instrument(raw);
    let instrument_name: Option<String> = instrument.as_ref().map(|_| raw.trim().to_owned());

    (instrument, instrument_name)
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
