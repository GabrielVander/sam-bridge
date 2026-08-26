use std::sync::OnceLock;

use regex::Regex;
use student_management::domain::entities::{
    MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
};

pub fn map(dto: &sam::client::SamStudent) -> anyhow::Result<Student> {
    let cleaned_location = clean_location(&dto.location);
    let (location, region) = split_location_bundle(&cleaned_location);

    Ok(Student {
        id: dto.id.trim().to_owned(),
        name: dto.name.trim().to_owned(),
        position: parse_position(&dto.role, &dto.level),
        location,
        region,
    })
}

fn parse_position(role: &str, level: &str) -> StudentPosition {
    match role.trim().to_uppercase().as_str() {
        "MÚSICO" => StudentPosition::Musician {
            level: into_musician_level(level),
        },
        "ORGANISTA" => StudentPosition::Organist {
            level: into_organist_level(level),
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

    let span_re = SPAN_RE.get_or_init(|| Regex::new(r"<span[^>]*></span>").expect("Valid regex"));
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

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::SamStudent;

    fn dto_with(overrides: impl FnOnce(&mut SamStudent)) -> SamStudent {
        let mut dto = SamStudent {
            id: "99998".to_owned(),
            name: "PEDRO ÁLVARES CABRAL".to_owned(),
            location:
                "JARDIM PALMARES DO NORTE <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
                    .to_owned(),
            role: "MÚSICO".to_owned(),
            instrument: "VIOLINO".to_owned(),
            level: "CANDIDATO(A)".to_owned(),
        };
        overrides(&mut dto);
        dto
    }

    #[test]
    fn given_musician_candidate_should_map_fully() {
        let student = map(&dto_with(|_| {})).expect("Mapping should succeed");

        assert_eq!(student.id, "99998");
        assert_eq!(student.name, "PEDRO ÁLVARES CABRAL");
        assert_eq!(
            student.position,
            StudentPosition::Musician {
                level: MusicianLevel::Candidate
            }
        );
        assert_eq!(student.location, "JARDIM PALMARES DO NORTE");
        assert_eq!(student.region, Region::AraraquaraSaoCarlos);
    }

    #[test]
    fn given_every_field_absent_should_still_produce_a_student() {
        let student = map(&SamStudent {
            id: String::new(),
            name: String::new(),
            location: String::new(),
            role: String::new(),
            instrument: String::new(),
            level: String::new(),
        })
        .expect("Absent fields must not fail");

        assert_eq!(student.id, "");
        assert_eq!(student.name, "");
        assert_eq!(student.position, StudentPosition::Unknown(String::new()));
        assert_eq!(student.location, "");
        assert_eq!(student.region, Region::Other(String::new()));
    }

    #[test]
    fn given_secretary_roles_should_map() {
        for (raw, expected) in [
            ("SECRETÁRIO DO GEM", SecretaryType::Gem),
            ("SECRETÁRIO DA MÚSICA", SecretaryType::Music),
        ] {
            let student = map(&dto_with(|d| d.role = raw.to_owned())).expect("Should map");

            assert_eq!(
                student.position,
                StudentPosition::Secretary { r#type: expected },
                "Role '{raw}' should map to its secretary type"
            );
        }
    }

    #[test]
    fn given_unknown_role_or_level_should_preserve_raw_value() {
        let unknown_role = map(&dto_with(|d| d.role = "REGENTE".to_owned())).unwrap();
        assert_eq!(
            unknown_role.position,
            StudentPosition::Unknown("REGENTE".to_owned())
        );

        let unknown_level = map(&dto_with(|d| d.level = "AVANÇADO".to_owned())).unwrap();
        assert_eq!(
            unknown_level.position,
            StudentPosition::Musician {
                level: MusicianLevel::Unknown("AVANÇADO".to_owned())
            }
        );
    }

    #[test]
    fn given_every_musician_level_should_map() {
        for (raw, expected) in [
            ("CANDIDATO(A)", MusicianLevel::Candidate),
            ("CULTO OFICIAL", MusicianLevel::OfficialService),
            ("ENSAIO", MusicianLevel::Practice),
            ("RJM", MusicianLevel::YouthService),
            ("EXÓTICO", MusicianLevel::Unknown("EXÓTICO".to_owned())),
        ] {
            let student =
                map(&dto_with(|d| d.level = raw.to_owned())).expect("Mapping should succeed");

            assert_eq!(
                student.position,
                StudentPosition::Musician { level: expected },
                "Musician level '{raw}' should map correctly"
            );
        }
    }

    #[test]
    fn given_every_organist_level_should_map() {
        for (raw, expected) in [
            ("CANDIDATO(A)", OrganistLevel::Candidate),
            ("CULTO OFICIAL", OrganistLevel::OfficialService),
            ("ENSAIO", OrganistLevel::Practice),
            ("RJM", OrganistLevel::YouthService),
            ("MEIA HORA", OrganistLevel::HalfHour),
            (
                "RJM / CULTO OFICIAL",
                OrganistLevel::YouthServiceOfficialService,
            ),
            ("RJM / ENSAIO", OrganistLevel::YouthServicePractice),
            ("RJM / MEIA HORA", OrganistLevel::YouthServiceHalfHour),
            (
                "RJM / OFICIALIZADO(A)",
                OrganistLevel::YouthServiceOfficialized,
            ),
            ("EXÓTICO", OrganistLevel::Unknown("EXÓTICO".to_owned())),
        ] {
            let student = map(&dto_with(|d| {
                d.role = "ORGANISTA".to_owned();
                d.level = raw.to_owned();
            }))
            .expect("Mapping should succeed");

            assert_eq!(
                student.position,
                StudentPosition::Organist { level: expected },
                "Organist level '{raw}' should map correctly"
            );
        }
    }

    #[test]
    fn given_multiple_spans_and_spaces_should_collapse_to_single_space() {
        let student = map(&dto_with(|d| {
            d.location =
                "A   B<span></span><span class='x'></span>  C | BR-SP-ARARAQUARA-SÃO CARLOS"
                    .to_owned()
        }))
        .unwrap();

        assert_eq!(student.location, "A B C");
    }

    #[test]
    fn given_missing_pipe_in_location_should_yield_empty_region_part() {
        let student = map(&dto_with(|d| d.location = "SEM REGIÃO AQUI".to_owned())).unwrap();

        assert_eq!(student.location, "SEM REGIÃO AQUI");
        assert_eq!(student.region, Region::Other(String::new()));
    }

    #[test]
    fn given_unrecognized_region_should_other_the_region_part() {
        let student = map(&dto_with(|d| {
            d.location = "BAIRRO ALTO <span></span> | SP-CAMPINAS".to_owned()
        }))
        .expect("Mapping should succeed");

        assert_eq!(student.location, "BAIRRO ALTO");
        assert_eq!(student.region, Region::Other("SP-CAMPINAS".to_owned()));
    }

    #[test]
    fn given_itirapina_region_should_map() {
        let student = map(&dto_with(|d| {
            d.location = "CENTRO | BR-SP-ARARAQUARA-ITIRAPINA".to_owned()
        }))
        .expect("Mapping should succeed");

        assert_eq!(student.region, Region::AraraquaraItirapina);
    }

    #[test]
    fn given_case_differences_should_still_map() {
        let student = map(&dto_with(|d| {
            d.role = "músico".to_owned();
            d.level = "rjm".to_owned();
        }))
        .unwrap();

        assert_eq!(
            student.position,
            StudentPosition::Musician {
                level: MusicianLevel::YouthService
            }
        );
    }
}
