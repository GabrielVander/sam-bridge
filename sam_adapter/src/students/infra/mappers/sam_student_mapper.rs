use std::sync::OnceLock;

use regex::Regex;
use sam::client::SamStudent;
use student_core::domain::entities::{
    MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
};

pub(crate) struct SamStudentMapper;

impl SamStudentMapper {
    pub fn to_student_entity(sam_student: &SamStudent) -> Student {
        let cleaned_location = Self::clean_location(&sam_student.location);
        let (location, region) = Self::split_location_bundle(&cleaned_location);

        Student {
            id: sam_student.id.trim().to_owned(),
            name: sam_student.name.trim().to_owned(),
            position: Self::parse_position(&sam_student.role, &sam_student.level),
            location,
            region,
        }
    }

    fn parse_position(role: &str, level: &str) -> StudentPosition {
        match role.trim().to_uppercase().as_str() {
            "MÚSICO" => StudentPosition::Musician {
                level: Self::into_musician_level(level),
            },
            "ORGANISTA" => StudentPosition::Organist {
                level: Self::into_organist_level(level),
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

    fn into_musician_level(level: &str) -> MusicianLevel {
        match level.trim().to_uppercase().as_str() {
            "CANDIDATO(A)" => MusicianLevel::Candidate,
            "CULTO OFICIAL" => MusicianLevel::OfficialService,
            "ENSAIO" => MusicianLevel::Practice,
            "RJM" => MusicianLevel::YouthService,
            other => MusicianLevel::Unknown(other.to_owned()),
        }
    }

    fn into_organist_level(level: &str) -> OrganistLevel {
        match level.trim().to_uppercase().as_str() {
            "CANDIDATO(A)" => OrganistLevel::Candidate,
            "CULTO OFICIAL" => OrganistLevel::OfficialService,
            "ENSAIO" => OrganistLevel::Practice,
            "RJM" => OrganistLevel::YouthService,
            "MEIA HORA" => OrganistLevel::HalfHour,
            "RJM / CULTO OFICIAL" => OrganistLevel::YouthServiceOfficialService,
            "RJM / ENSAIO" => OrganistLevel::YouthServicePractice,
            "RJM / MEIA HORA" => OrganistLevel::YouthServiceHalfHour,
            "RJM / OFICIALIZADO(A)" => OrganistLevel::YouthServiceOfficialized,
            other => OrganistLevel::Unknown(other.to_owned()),
        }
    }

    fn clean_location(raw: &str) -> String {
        static SPAN_RE: OnceLock<Regex> = OnceLock::new();
        static SPACES_RE: OnceLock<Regex> = OnceLock::new();

        let span_re =
            SPAN_RE.get_or_init(|| Regex::new(r"<span[^>]*></span>").expect("Valid regex"));
        let spaces_re = SPACES_RE.get_or_init(|| Regex::new(r"\s{2,}").expect("Valid regex"));

        spaces_re
            .replace_all(&span_re.replace_all(raw, ""), " ")
            .trim()
            .to_owned()
    }

    fn split_location_bundle(cleaned: &str) -> (String, Region) {
        let mut parts = cleaned.split('|');

        let location = parts.next().unwrap_or_default().trim().to_owned();
        let raw_region = parts.next().unwrap_or_default().trim();

        let region = match raw_region.to_uppercase().as_str() {
            "BR-SP-ARARAQUARA-SÃO CARLOS" => Region::AraraquaraSaoCarlos,
            "BR-SP-ARARAQUARA-ITIRAPINA" => Region::AraraquaraItirapina,
            "" => Region::Other(String::new()),
            _ => Region::Other(raw_region.to_owned()),
        };

        (location, region)
    }
}
