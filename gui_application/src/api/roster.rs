use student::application::gateways::StudentGatewayError;
use student::domain::entities::{
    Instrument, MusicianLevel, OrganistLevel, Student, StudentPosition,
};

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub position: StudentPositionDto,
    pub location: String,
    pub instrument: Option<InstrumentDto>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstrumentDto {
    Violin,
    Viola,
    Cello,
    Flute,
    Oboe,
    Bassoon,
    Clarinet,
    AltoClarinet,
    BassClarinet,
    AltoSaxophone,
    CurvedSopranoSaxophone,
    StraightSopranoSaxophone,
    TenorSaxophone,
    Trumpet,
    Cornet,
    Flugelhorn,
    FrenchHorn,
    Trombone,
    Euphonium,
    Tuba,
    EnglishHorn,
    ContraltoViolin,
    Unknown { raw: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetrieveAllAvailableStudentsOutcomeDto {
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
    YouthServiceHalfHour,
    GemSecretary,
    Invalid { raw: String },
}

impl From<Result<Vec<Student>, StudentGatewayError>> for RetrieveAllAvailableStudentsOutcomeDto {
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
        let instrument: Option<InstrumentDto> = match &student.position {
            StudentPosition::Musician { instrument, .. } => {
                instrument.clone().map(InstrumentDto::from)
            }
            StudentPosition::Organist { .. }
            | StudentPosition::GemSecretary
            | StudentPosition::Unknown(_) => None,
        };

        Self {
            id: student.id.as_str().to_owned(),
            name: student.name,
            position: student.position.into(),
            location: student.location,
            instrument,
        }
    }
}

impl From<Instrument> for InstrumentDto {
    fn from(instrument: Instrument) -> Self {
        match instrument {
            Instrument::Violin => Self::Violin,
            Instrument::Viola => Self::Viola,
            Instrument::Cello => Self::Cello,
            Instrument::Flute => Self::Flute,
            Instrument::Oboe => Self::Oboe,
            Instrument::Bassoon => Self::Bassoon,
            Instrument::Clarinet => Self::Clarinet,
            Instrument::AltoClarinet => Self::AltoClarinet,
            Instrument::BassClarinet => Self::BassClarinet,
            Instrument::AltoSaxophone => Self::AltoSaxophone,
            Instrument::CurvedSopranoSaxophone => Self::CurvedSopranoSaxophone,
            Instrument::StraightSopranoSaxophone => Self::StraightSopranoSaxophone,
            Instrument::TenorSaxophone => Self::TenorSaxophone,
            Instrument::Trumpet => Self::Trumpet,
            Instrument::Cornet => Self::Cornet,
            Instrument::Flugelhorn => Self::Flugelhorn,
            Instrument::FrenchHorn => Self::FrenchHorn,
            Instrument::Trombone => Self::Trombone,
            Instrument::Euphonium => Self::Euphonium,
            Instrument::Tuba => Self::Tuba,
            Instrument::EnglishHorn => Self::EnglishHorn,
            Instrument::ContraltoViolin => Self::ContraltoViolin,
            Instrument::Unknown(raw) => Self::Unknown { raw },
        }
    }
}

impl From<StudentPosition> for StudentPositionDto {
    fn from(position: StudentPosition) -> Self {
        match position {
            StudentPosition::Musician { level, .. } => level.into(),
            StudentPosition::Organist { level } => level.into(),
            StudentPosition::GemSecretary => Self::GemSecretary,
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
            OrganistLevel::OfficialService => Self::OfficialService,
            OrganistLevel::YouthServiceHalfHour => Self::YouthServiceHalfHour,
            OrganistLevel::Unknown(raw) => Self::Invalid { raw },
        }
    }
}
