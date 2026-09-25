use student::application::gateways::StudentGatewayError;
use student::domain::entities::{
    MusicianLevel, OrganistLevel, SecretaryType, Student, StudentPosition,
};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub position: StudentPositionDto,
    pub location: String,
    pub instrument_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveAllAvailableStudentsOutcome {
    Success { students: Vec<StudentSummaryDto> },
    Failure { report: ErrorReportDto },
}

/// Musicians, organists and secretaries flattened into the single list of
/// positions SAM's listing shows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StudentPositionDto {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Officialized,
    HalfHour,
    YouthServiceHalfHour,
    YouthServicePractice,
    YouthServiceOfficialService,
    YouthServiceOfficialized,
    GemSecretary,
    MusicSecretary,
    Invalid { raw: String },
}

impl From<Result<Vec<Student>, StudentGatewayError>> for RetrieveAllAvailableStudentsOutcome {
    fn from(result: Result<Vec<Student>, StudentGatewayError>) -> Self {
        match result {
            Ok(students) => Self::Success {
                students: students.into_iter().map(StudentSummaryDto::from).collect(),
            },
            Err(error) => Self::Failure {
                report: error.into(),
            },
        }
    }
}

impl From<Student> for StudentSummaryDto {
    fn from(student: Student) -> Self {
        let instrument_name = match &student.position {
            StudentPosition::Musician {
                instrument_name, ..
            } => instrument_name.clone(),
            StudentPosition::Organist { .. }
            | StudentPosition::Secretary { .. }
            | StudentPosition::Unknown(_) => None,
        };

        Self {
            id: student.id,
            name: student.name,
            position: student.position.into(),
            location: student.location,
            instrument_name,
        }
    }
}

impl From<StudentPosition> for StudentPositionDto {
    fn from(position: StudentPosition) -> Self {
        match position {
            StudentPosition::Musician { level, .. } => level.into(),
            StudentPosition::Organist { level } => level.into(),
            StudentPosition::Secretary { r#type } => r#type.into(),
            StudentPosition::Unknown(raw) => Self::Invalid { raw },
        }
    }
}

impl From<MusicianLevel> for StudentPositionDto {
    fn from(level: MusicianLevel) -> Self {
        match level {
            MusicianLevel::Candidate => Self::Candidate,
            MusicianLevel::Practice => Self::Practice,
            MusicianLevel::YouthService => Self::YouthService,
            MusicianLevel::OfficialService => Self::OfficialService,
            MusicianLevel::Officialized => Self::Officialized,
            MusicianLevel::Unknown(raw) => Self::Invalid { raw },
        }
    }
}

impl From<OrganistLevel> for StudentPositionDto {
    fn from(level: OrganistLevel) -> Self {
        match level {
            OrganistLevel::Candidate => Self::Candidate,
            OrganistLevel::Practice => Self::Practice,
            OrganistLevel::YouthService => Self::YouthService,
            OrganistLevel::HalfHour => Self::HalfHour,
            OrganistLevel::OfficialService => Self::OfficialService,
            OrganistLevel::YouthServiceHalfHour => Self::YouthServiceHalfHour,
            OrganistLevel::YouthServicePractice => Self::YouthServicePractice,
            OrganistLevel::YouthServiceOfficialService => Self::YouthServiceOfficialService,
            OrganistLevel::YouthServiceOfficialized => Self::YouthServiceOfficialized,
            OrganistLevel::Unknown(raw) => Self::Invalid { raw },
        }
    }
}

impl From<SecretaryType> for StudentPositionDto {
    fn from(r#type: SecretaryType) -> Self {
        match r#type {
            SecretaryType::Gem => Self::GemSecretary,
            SecretaryType::Music => Self::MusicSecretary,
        }
    }
}

#[cfg(test)]
mod tests {
    use student::domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    };

    use super::{StudentPositionDto, StudentSummaryDto};

    #[test]
    fn carries_the_instrument_name_of_musicians() {
        let input = student(
            "1",
            StudentPosition::Musician {
                level: MusicianLevel::Practice,
                instrument: Some(Instrument::Saxophone),
                instrument_name: Some("SAXOFONE TENOR".to_owned()),
            },
        );

        let actual = StudentSummaryDto::from(input);

        assert_eq!(
            actual,
            StudentSummaryDto {
                instrument_name: Some("SAXOFONE TENOR".to_owned()),
                ..summary("1", StudentPositionDto::Practice)
            }
        );
    }

    #[test]
    fn only_musicians_have_an_instrument_name() {
        let positions = [
            StudentPosition::Musician {
                level: MusicianLevel::Candidate,
                instrument: None,
                instrument_name: None,
            },
            StudentPosition::Organist {
                level: OrganistLevel::Practice,
            },
            StudentPosition::Secretary {
                r#type: SecretaryType::Gem,
            },
            StudentPosition::Unknown("Avocado".to_owned()),
        ];

        for position in positions {
            assert_eq!(
                StudentSummaryDto::from(student("1", position)).instrument_name,
                None
            );
        }
    }

    #[test]
    fn every_musician_level_is_mapped() {
        let cases = [
            (MusicianLevel::Candidate, StudentPositionDto::Candidate),
            (MusicianLevel::Practice, StudentPositionDto::Practice),
            (
                MusicianLevel::YouthService,
                StudentPositionDto::YouthService,
            ),
            (
                MusicianLevel::OfficialService,
                StudentPositionDto::OfficialService,
            ),
            (
                MusicianLevel::Officialized,
                StudentPositionDto::Officialized,
            ),
            (
                MusicianLevel::Unknown("Strawberry".to_owned()),
                StudentPositionDto::Invalid {
                    raw: "Strawberry".to_owned(),
                },
            ),
        ];

        for (level, expected) in cases {
            let position = StudentPosition::Musician {
                level,
                instrument: None,
                instrument_name: None,
            };
            assert_eq!(StudentPositionDto::from(position), expected);
        }
    }

    #[test]
    fn every_organist_level_is_mapped() {
        let cases = [
            (OrganistLevel::Candidate, StudentPositionDto::Candidate),
            (OrganistLevel::Practice, StudentPositionDto::Practice),
            (
                OrganistLevel::YouthService,
                StudentPositionDto::YouthService,
            ),
            (OrganistLevel::HalfHour, StudentPositionDto::HalfHour),
            (
                OrganistLevel::OfficialService,
                StudentPositionDto::OfficialService,
            ),
            (
                OrganistLevel::YouthServiceHalfHour,
                StudentPositionDto::YouthServiceHalfHour,
            ),
            (
                OrganistLevel::YouthServicePractice,
                StudentPositionDto::YouthServicePractice,
            ),
            (
                OrganistLevel::YouthServiceOfficialService,
                StudentPositionDto::YouthServiceOfficialService,
            ),
            (
                OrganistLevel::YouthServiceOfficialized,
                StudentPositionDto::YouthServiceOfficialized,
            ),
            (
                OrganistLevel::Unknown("Peanuts".to_owned()),
                StudentPositionDto::Invalid {
                    raw: "Peanuts".to_owned(),
                },
            ),
        ];

        for (level, expected) in cases {
            let position = StudentPosition::Organist { level };
            assert_eq!(StudentPositionDto::from(position), expected);
        }
    }

    #[test]
    fn every_secretary_type_is_mapped() {
        let cases = [
            (SecretaryType::Gem, StudentPositionDto::GemSecretary),
            (SecretaryType::Music, StudentPositionDto::MusicSecretary),
        ];

        for (r#type, expected) in cases {
            let position = StudentPosition::Secretary { r#type };
            assert_eq!(StudentPositionDto::from(position), expected);
        }
    }

    #[test]
    fn an_unknown_position_keeps_what_sam_wrote() {
        let position = StudentPosition::Unknown("Avocado".to_owned());

        assert_eq!(
            StudentPositionDto::from(position),
            StudentPositionDto::Invalid {
                raw: "Avocado".to_owned(),
            }
        );
    }

    fn student(id: &str, position: StudentPosition) -> Student {
        Student {
            id: id.to_owned(),
            name: format!("Student {id}"),
            position,
            location: "Location".to_owned(),
            region: Region::AraraquaraSaoCarlos,
        }
    }

    fn summary(id: &str, position: StudentPositionDto) -> StudentSummaryDto {
        StudentSummaryDto {
            id: id.to_owned(),
            name: format!("Student {id}"),
            position,
            location: "Location".to_owned(),
            instrument_name: None,
        }
    }
}
