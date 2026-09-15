// FRB only generates plain, field-accessible Dart classes for types defined
// in this crate's own scanned source (`rust_input: crate`); types from other
// crates come through as opaque handles. Rather than adding a
// `flutter_rust_bridge` dependency to `student` (an application/domain-layer
// crate that must stay ignorant of any specific UI framework), these mirror
// `student::application::dto::{StudentSummaryDto, StudentPositionDto}`
// field-for-field so FRB can expose them to Dart directly.

use student::application::dto as student_dto;

pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub position: StudentPositionDto,
    pub location: String,
}

pub enum RetrieveAllAvailableStudentsOutcome {
    Success(Vec<StudentSummaryDto>),
    Failure(String),
}

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

impl From<student_dto::StudentSummaryDto> for StudentSummaryDto {
    fn from(dto: student_dto::StudentSummaryDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            position: dto.position.into(),
            location: dto.location,
        }
    }
}

impl From<student_dto::StudentPositionDto> for StudentPositionDto {
    fn from(dto: student_dto::StudentPositionDto) -> Self {
        match dto {
            student_dto::StudentPositionDto::Candidate => Self::Candidate,
            student_dto::StudentPositionDto::Practice => Self::Practice,
            student_dto::StudentPositionDto::YouthService => Self::YouthService,
            student_dto::StudentPositionDto::OfficialService => Self::OfficialService,
            student_dto::StudentPositionDto::Officialized => Self::Officialized,
            student_dto::StudentPositionDto::HalfHour => Self::HalfHour,
            student_dto::StudentPositionDto::YouthServiceHalfHour => Self::YouthServiceHalfHour,
            student_dto::StudentPositionDto::YouthServicePractice => Self::YouthServicePractice,
            student_dto::StudentPositionDto::YouthServiceOfficialService => {
                Self::YouthServiceOfficialService
            }
            student_dto::StudentPositionDto::YouthServiceOfficialized => {
                Self::YouthServiceOfficialized
            }
            student_dto::StudentPositionDto::GemSecretary => Self::GemSecretary,
            student_dto::StudentPositionDto::MusicSecretary => Self::MusicSecretary,
            student_dto::StudentPositionDto::Invalid(value) => Self::Invalid(value),
        }
    }
}
