use std::sync::Arc;

use async_trait::async_trait;
use student::{
    application::gateways::{StudentGateway, StudentGatewayError},
    domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

use crate::client::{SamClient, SamStudent};

pub struct StudentGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl StudentGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl StudentGateway for StudentGatewaySamImpl {
    async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        let sam_students: Vec<SamStudent> = self
            .client
            .students()
            .map_err(|_| StudentGatewayError::UnableToPerformOperation)?;

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
        "MÚSICO" => StudentPosition::Musician {
            level: parse_musician_level(level),
            instrument: parse_instrument(instrument),
        },
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

fn parse_instrument(instrument: &str) -> Option<Instrument> {
    match instrument {
        "A DEFINIR" => None,
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

#[cfg(test)]
mod tests {
    use student::domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, StudentPosition,
    };

    use super::{
        clean_location, parse_instrument, parse_musician_level, parse_organist_level,
        parse_position, parse_region,
    };

    #[test]
    fn known_musician_candidate_is_recognized() {
        assert_eq!(
            parse_position("MÚSICO", "CANDIDATO(A)", "A DEFINIR"),
            StudentPosition::Musician {
                level: MusicianLevel::Candidate,
                instrument: None,
            }
        );
    }

    #[test]
    fn musician_with_a_confirmed_instrument_is_recognized() {
        assert_eq!(
            parse_position("MÚSICO", "RJM", "VIOLINO"),
            StudentPosition::Musician {
                level: MusicianLevel::YouthService,
                instrument: Some(Instrument::Violin),
            }
        );
    }

    #[test]
    fn organist_role_is_recognized() {
        assert_eq!(
            parse_position("ORGANISTA", "RJM / MEIA HORA", "A DEFINIR"),
            StudentPosition::Organist {
                level: OrganistLevel::YouthServiceHalfHour,
            }
        );
    }

    #[test]
    fn gem_secretary_role_is_recognized() {
        assert_eq!(
            parse_position("SECRETÁRIO DO GEM", "RJM", "A DEFINIR"),
            StudentPosition::Secretary {
                r#type: SecretaryType::Gem,
            }
        );
    }

    #[test]
    fn unknown_role_falls_through_to_unknown() {
        assert_eq!(
            parse_position("BATERISTA", "CANDIDATO(A)", "A DEFINIR"),
            StudentPosition::Unknown("BATERISTA".to_owned())
        );
    }

    #[test]
    fn unknown_musician_level_falls_through_to_unknown() {
        assert_eq!(
            parse_musician_level("PRÁTICO(A)"),
            MusicianLevel::Unknown("PRÁTICO(A)".to_owned())
        );
    }

    #[test]
    fn compound_musician_level_of_unconfirmed_meaning_falls_through_to_unknown() {
        assert_eq!(
            parse_musician_level("RJM / ENSAIO"),
            MusicianLevel::Unknown("RJM / ENSAIO".to_owned())
        );
    }

    #[test]
    fn unknown_organist_level_falls_through_to_unknown() {
        assert_eq!(
            parse_organist_level("ALGO NOVO"),
            OrganistLevel::Unknown("ALGO NOVO".to_owned())
        );
    }

    #[test]
    fn unassigned_instrument_is_none() {
        assert_eq!(parse_instrument("A DEFINIR"), None);
    }

    #[test]
    fn saxophone_subtypes_all_map_to_the_generic_saxophone_requirement() {
        for raw in [
            "SAXOFONE ALTO",
            "SAXOFONE SOPRANO CUR",
            "SAXOFONE SOPRANO RET",
            "SAXOFONE TENOR",
        ] {
            assert_eq!(parse_instrument(raw), Some(Instrument::Saxophone));
        }
    }

    #[test]
    fn trumpet_aliases_map_to_trumpet() {
        for raw in ["TROMPETE", "CORNET", "FLUGELHORN"] {
            assert_eq!(parse_instrument(raw), Some(Instrument::Trumpet));
        }
    }

    #[test]
    fn unpublished_but_real_instrument_categories_are_recognized() {
        assert_eq!(
            parse_instrument("CORNE INGLÊS"),
            Some(Instrument::EnglishHorn)
        );
        assert_eq!(
            parse_instrument("VIOLINO CONTRALTO"),
            Some(Instrument::ContraltoViolin)
        );
    }

    #[test]
    fn unrecognized_instrument_falls_through_to_unknown() {
        assert_eq!(
            parse_instrument("BANDOLIM"),
            Some(Instrument::Unknown("BANDOLIM".to_owned()))
        );
    }

    #[test]
    fn confirmed_region_is_recognized() {
        assert_eq!(
            parse_region(
                "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
            ),
            Region::AraraquaraSaoCarlos
        );
    }

    #[test]
    fn unrecognized_region_falls_through_to_other() {
        assert_eq!(
            parse_region("SOME OTHER LOCATION"),
            Region::Other("SOME OTHER LOCATION".to_owned())
        );
    }

    #[test]
    fn html_markup_is_stripped_from_location() {
        assert_eq!(
            clean_location(
                "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
            ),
            "JARDIM PALMARES DO SUL | BR-SP-ARARAQUARA-SÃO CARLOS"
        );
    }

    #[test]
    fn location_without_markup_is_unchanged() {
        assert_eq!(clean_location("SOME PLAIN LOCATION"), "SOME PLAIN LOCATION");
    }

    #[test]
    fn empty_location_stays_empty() {
        assert_eq!(clean_location(""), "");
    }
}
