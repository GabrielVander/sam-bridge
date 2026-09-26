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
