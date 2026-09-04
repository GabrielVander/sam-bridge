use std::sync::Arc;

use crate::{
    domain::entities::{MusicianLevel, OrganistLevel, SecretaryType, StudentPosition},
    student_roster::{application::gateways::StudentGateway, domain::entities::Student},
};

#[derive(Clone)]
pub struct RetrieveAllAvailableStudentsUseCase {
    student_gateway: Arc<dyn StudentGateway + Send + Sync>,
}

impl RetrieveAllAvailableStudentsUseCase {
    pub fn new(gateway: Arc<dyn StudentGateway + Send + Sync>) -> Self {
        Self {
            student_gateway: gateway.clone(),
        }
    }

    pub async fn execute(&self) -> RetrieveAllAvailableStudentsResult {
        let students_result: anyhow::Result<Vec<Student>> =
            self.student_gateway.get_available_records().await;

        match students_result {
            Ok(students) => RetrieveAllAvailableStudentsResult::Success(
                students.into_iter().map(StudentSummaryDto::from).collect(),
            ),
            Err(error) => RetrieveAllAvailableStudentsResult::Failure(
                RetrieveAllAvailableStudentsError::GatewayError {
                    context: error.to_string(),
                },
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetrieveAllAvailableStudentsResult {
    Success(Vec<StudentSummaryDto>),
    Failure(RetrieveAllAvailableStudentsError),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RetrieveAllAvailableStudentsError {
    GatewayError { context: String },
}

#[derive(Debug, Clone, PartialEq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub position: StudentPositionDto,
    pub location: String,
}

#[derive(Debug, Clone, PartialEq)]
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
    Invalid(String),
}

impl From<Student> for StudentSummaryDto {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            name: student.name,
            position: StudentPositionDto::from(student.position),
            location: student.location,
        }
    }
}

impl From<StudentPosition> for StudentPositionDto {
    fn from(position: StudentPosition) -> Self {
        match position {
            StudentPosition::Musician { level } => level.into(),
            StudentPosition::Organist { level } => level.into(),
            StudentPosition::Secretary { r#type } => r#type.into(),
            StudentPosition::Unknown(value) => Self::Invalid(value),
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
            MusicianLevel::Unknown(value) => Self::Invalid(value),
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
            OrganistLevel::Unknown(value) => Self::Invalid(value),
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
    use crate::{
        application::use_cases::{StudentPositionDto, StudentSummaryDto},
        domain::entities::{OrganistLevel, Region, SecretaryType, Student, StudentPosition},
        student_roster::domain::entities::MusicianLevel,
    };

    use pretty_assertions::assert_eq;

    #[test]
    fn maps_all_musician_levels() {
        let input: [Student; 7] = all_musicians_levels();
        let expected: [StudentSummaryDto; 7] = expected_musician_dtos();

        let actual: [StudentSummaryDto; 7] = input.map(StudentSummaryDto::from);

        assert_eq!(actual, expected);
    }

    #[test]
    fn maps_all_organist_levels() {
        let input: [Student; 11] = all_organists_levels();
        let expected: [StudentSummaryDto; 11] = expected_organist_dtos();

        let actual: [StudentSummaryDto; 11] = input.map(StudentSummaryDto::from);

        assert_eq!(actual, expected)
    }

    #[test]
    fn maps_all_secretary_types() {
        let input: [Student; 2] = all_secretary_types();
        let expected: [StudentSummaryDto; 2] = expected_secretary_dtos();

        let actual: [StudentSummaryDto; 2] = input.map(StudentSummaryDto::from);

        assert_eq!(actual, expected);
    }

    #[test]
    fn maps_unknown_positions() {
        let input: [Student; 2] = unknown_positions();
        let expected: [StudentSummaryDto; 2] = expected_unknown_positions_dtos();

        let actual: [StudentSummaryDto; 2] = input.map(StudentSummaryDto::from);

        assert_eq!(actual, expected);
    }

    fn all_musicians_levels() -> [Student; 7] {
        [
            student(
                "Musician - Candidate",
                StudentPosition::Musician {
                    level: MusicianLevel::Candidate,
                },
            ),
            student(
                "Musician - Practice",
                StudentPosition::Musician {
                    level: MusicianLevel::Practice,
                },
            ),
            student(
                "Musician - Youth",
                StudentPosition::Musician {
                    level: MusicianLevel::YouthService,
                },
            ),
            student(
                "Musician - Official",
                StudentPosition::Musician {
                    level: MusicianLevel::OfficialService,
                },
            ),
            student(
                "Musician - Officialized",
                StudentPosition::Musician {
                    level: MusicianLevel::Officialized,
                },
            ),
            student(
                "Musician - Unknown 1",
                StudentPosition::Musician {
                    level: MusicianLevel::Unknown("Strawberry".to_string()),
                },
            ),
            student(
                "Musician - Unknown 2",
                StudentPosition::Musician {
                    level: MusicianLevel::Unknown("Banana".to_string()),
                },
            ),
        ]
    }

    fn expected_musician_dtos() -> [StudentSummaryDto; 7] {
        [
            student_summary("Musician - Candidate", StudentPositionDto::Candidate),
            student_summary("Musician - Practice", StudentPositionDto::Practice),
            student_summary("Musician - Youth", StudentPositionDto::YouthService),
            student_summary("Musician - Official", StudentPositionDto::OfficialService),
            student_summary("Musician - Officialized", StudentPositionDto::Officialized),
            student_summary(
                "Musician - Unknown 1",
                StudentPositionDto::Invalid("Strawberry".to_string()),
            ),
            student_summary(
                "Musician - Unknown 2",
                StudentPositionDto::Invalid("Banana".to_string()),
            ),
        ]
    }

    fn all_organists_levels() -> [Student; 11] {
        [
            student(
                "Organist - Candidate",
                StudentPosition::Organist {
                    level: OrganistLevel::Candidate,
                },
            ),
            student(
                "Organist - Practice",
                StudentPosition::Organist {
                    level: OrganistLevel::Practice,
                },
            ),
            student(
                "Organist - Youth",
                StudentPosition::Organist {
                    level: OrganistLevel::YouthService,
                },
            ),
            student(
                "Organist - HalfHour",
                StudentPosition::Organist {
                    level: OrganistLevel::HalfHour,
                },
            ),
            student(
                "Organist - Official",
                StudentPosition::Organist {
                    level: OrganistLevel::OfficialService,
                },
            ),
            student(
                "Organist - YouthHalfHour",
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceHalfHour,
                },
            ),
            student(
                "Organist - YouthPractice",
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServicePractice,
                },
            ),
            student(
                "Organist - YouthOfficial",
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceOfficialService,
                },
            ),
            student(
                "Organist - YouthOfficialized",
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceOfficialized,
                },
            ),
            student(
                "Organist - Unknown 1",
                StudentPosition::Organist {
                    level: OrganistLevel::Unknown("Peanuts".to_string()),
                },
            ),
            student(
                "Organist - Unknown 2",
                StudentPosition::Organist {
                    level: OrganistLevel::Unknown("Pineapple".to_string()),
                },
            ),
        ]
    }

    fn expected_organist_dtos() -> [StudentSummaryDto; 11] {
        [
            student_summary("Organist - Candidate", StudentPositionDto::Candidate),
            student_summary("Organist - Practice", StudentPositionDto::Practice),
            student_summary("Organist - Youth", StudentPositionDto::YouthService),
            student_summary("Organist - HalfHour", StudentPositionDto::HalfHour),
            student_summary("Organist - Official", StudentPositionDto::OfficialService),
            student_summary(
                "Organist - YouthHalfHour",
                StudentPositionDto::YouthServiceHalfHour,
            ),
            student_summary(
                "Organist - YouthPractice",
                StudentPositionDto::YouthServicePractice,
            ),
            student_summary(
                "Organist - YouthOfficial",
                StudentPositionDto::YouthServiceOfficialService,
            ),
            student_summary(
                "Organist - YouthOfficialized",
                StudentPositionDto::YouthServiceOfficialized,
            ),
            student_summary(
                "Organist - Unknown 1",
                StudentPositionDto::Invalid("Peanuts".to_string()),
            ),
            student_summary(
                "Organist - Unknown 2",
                StudentPositionDto::Invalid("Pineapple".to_string()),
            ),
        ]
    }

    fn all_secretary_types() -> [Student; 2] {
        [
            student(
                "Secretary - GEM",
                StudentPosition::Secretary {
                    r#type: SecretaryType::Gem,
                },
            ),
            student(
                "Secretary - Music",
                StudentPosition::Secretary {
                    r#type: SecretaryType::Music,
                },
            ),
        ]
    }

    fn expected_secretary_dtos() -> [StudentSummaryDto; 2] {
        [
            student_summary("Secretary - GEM", StudentPositionDto::GemSecretary),
            student_summary("Secretary - Music", StudentPositionDto::MusicSecretary),
        ]
    }

    fn unknown_positions() -> [Student; 2] {
        [
            student("Unknown 1", StudentPosition::Unknown("Avocado".to_string())),
            student("Unknown 2", StudentPosition::Unknown("Date".to_string())),
        ]
    }

    fn expected_unknown_positions_dtos() -> [StudentSummaryDto; 2] {
        [
            student_summary(
                "Unknown 1",
                StudentPositionDto::Invalid("Avocado".to_string()),
            ),
            student_summary("Unknown 2", StudentPositionDto::Invalid("Date".to_string())),
        ]
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

    fn student_summary(id: &str, position: StudentPositionDto) -> StudentSummaryDto {
        StudentSummaryDto {
            id: id.to_owned(),
            name: format!("Student {id}"),
            position,
            location: "Location".to_owned(),
        }
    }
}
