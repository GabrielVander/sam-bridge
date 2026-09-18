use student::application::dto as student_dto;

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
    Success(Vec<StudentSummaryDto>),
    Failure(String),
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
    Invalid(String),
}

impl From<student_dto::StudentSummaryDto> for StudentSummaryDto {
    fn from(dto: student_dto::StudentSummaryDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            position: dto.position.into(),
            location: dto.location,
            instrument_name: dto.instrument_name,
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

#[cfg(test)]
mod tests {
    use super::{student_dto, StudentPositionDto, StudentSummaryDto};

    #[test]
    fn every_student_position_variant_is_mapped() {
        let cases = [
            (
                student_dto::StudentPositionDto::Candidate,
                StudentPositionDto::Candidate,
            ),
            (
                student_dto::StudentPositionDto::Practice,
                StudentPositionDto::Practice,
            ),
            (
                student_dto::StudentPositionDto::YouthService,
                StudentPositionDto::YouthService,
            ),
            (
                student_dto::StudentPositionDto::OfficialService,
                StudentPositionDto::OfficialService,
            ),
            (
                student_dto::StudentPositionDto::Officialized,
                StudentPositionDto::Officialized,
            ),
            (
                student_dto::StudentPositionDto::HalfHour,
                StudentPositionDto::HalfHour,
            ),
            (
                student_dto::StudentPositionDto::YouthServiceHalfHour,
                StudentPositionDto::YouthServiceHalfHour,
            ),
            (
                student_dto::StudentPositionDto::YouthServicePractice,
                StudentPositionDto::YouthServicePractice,
            ),
            (
                student_dto::StudentPositionDto::YouthServiceOfficialService,
                StudentPositionDto::YouthServiceOfficialService,
            ),
            (
                student_dto::StudentPositionDto::YouthServiceOfficialized,
                StudentPositionDto::YouthServiceOfficialized,
            ),
            (
                student_dto::StudentPositionDto::GemSecretary,
                StudentPositionDto::GemSecretary,
            ),
            (
                student_dto::StudentPositionDto::MusicSecretary,
                StudentPositionDto::MusicSecretary,
            ),
            (
                student_dto::StudentPositionDto::Invalid("odd".to_owned()),
                StudentPositionDto::Invalid("odd".to_owned()),
            ),
        ];

        for (input, expected) in cases {
            assert_eq!(StudentPositionDto::from(input), expected);
        }
    }

    #[test]
    fn student_summary_fields_are_mapped() {
        let dto = student_dto::StudentSummaryDto {
            id: "1".to_owned(),
            name: "Someone".to_owned(),
            position: student_dto::StudentPositionDto::YouthService,
            location: "Somewhere".to_owned(),
            instrument_name: Some("SAXOFONE TENOR".to_owned()),
        };

        let mapped = StudentSummaryDto::from(dto);

        assert_eq!(
            mapped,
            StudentSummaryDto {
                id: "1".to_owned(),
                name: "Someone".to_owned(),
                position: StudentPositionDto::YouthService,
                location: "Somewhere".to_owned(),
                instrument_name: Some("SAXOFONE TENOR".to_owned()),
            }
        );
    }

    #[test]
    fn missing_instrument_name_stays_missing() {
        let dto = student_dto::StudentSummaryDto {
            id: "2".to_owned(),
            name: "Organist".to_owned(),
            position: student_dto::StudentPositionDto::Practice,
            location: "Somewhere".to_owned(),
            instrument_name: None,
        };

        assert_eq!(StudentSummaryDto::from(dto).instrument_name, None);
    }
}
