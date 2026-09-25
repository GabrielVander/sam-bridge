use student::application::gateways::MusicianProfileGatewayError;
use student::application::use_cases::AssessStudentProgressError;
use student::domain::entities as student_entities;

use crate::api::error_report::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementStatusDto {
    pub msa_met: bool,
    pub method_met: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MusicianLevelDto {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Officialized,
    Unknown { raw: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointStatusDto {
    pub level: MusicianLevelDto,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub requirement: RequirementStatusDto,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressAssessmentDto {
    pub checkpoints: Vec<CheckpointStatusDto>,
    pub msa_relative_percent: f64,
    pub method_relative_percent: f64,
    pub combined_percent: f64,
    pub overall_checkpoint_percent: f64,
    pub next_level: Option<MusicianLevelDto>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssessStudentProgressOutcome {
    Success { assessment: ProgressAssessmentDto },
    NoInstrumentAssigned,
    UnknownLevel { raw_level: String },
    NotAMusician,
    Failure { report: ErrorReportDto },
}

impl From<Result<student_entities::ProgressAssessment, AssessStudentProgressError>>
    for AssessStudentProgressOutcome
{
    fn from(
        result: Result<student_entities::ProgressAssessment, AssessStudentProgressError>,
    ) -> Self {
        match result {
            Ok(assessment) => Self::Success {
                assessment: assessment.into(),
            },
            Err(AssessStudentProgressError::NoInstrumentAssigned) => Self::NoInstrumentAssigned,
            Err(AssessStudentProgressError::Assessment(
                student_entities::AssessError::UnknownLevel(raw),
            )) => Self::UnknownLevel { raw_level: raw },
            Err(AssessStudentProgressError::Profile(MusicianProfileGatewayError::NotAMusician)) => {
                Self::NotAMusician
            }
            Err(error) => Self::Failure {
                report: error.into(),
            },
        }
    }
}

impl From<student_entities::ProgressAssessment> for ProgressAssessmentDto {
    fn from(assessment: student_entities::ProgressAssessment) -> Self {
        Self {
            checkpoints: assessment
                .checkpoints
                .into_iter()
                .map(CheckpointStatusDto::from)
                .collect(),
            msa_relative_percent: assessment.msa_relative_percent,
            method_relative_percent: assessment.method_relative_percent,
            combined_percent: assessment.combined_percent,
            overall_checkpoint_percent: assessment.overall_checkpoint_percent,
            next_level: assessment.next_level.map(MusicianLevelDto::from),
        }
    }
}

impl From<student_entities::MusicianLevel> for MusicianLevelDto {
    fn from(level: student_entities::MusicianLevel) -> Self {
        match level {
            student_entities::MusicianLevel::Candidate => Self::Candidate,
            student_entities::MusicianLevel::Practice => Self::Practice,
            student_entities::MusicianLevel::YouthService => Self::YouthService,
            student_entities::MusicianLevel::OfficialService => Self::OfficialService,
            student_entities::MusicianLevel::Officialized => Self::Officialized,
            student_entities::MusicianLevel::Unknown(raw) => Self::Unknown { raw },
        }
    }
}

impl From<student_entities::CheckpointStatus> for CheckpointStatusDto {
    fn from(checkpoint: student_entities::CheckpointStatus) -> Self {
        Self {
            level: checkpoint.level.into(),
            achieved: checkpoint.achieved,
            ready_to_advance: checkpoint.ready_to_advance,
            requirement: RequirementStatusDto {
                msa_met: checkpoint.requirement.msa_met,
                method_met: checkpoint.requirement.method_met,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CheckpointStatusDto, MusicianLevelDto, ProgressAssessmentDto, RequirementStatusDto,
    };
    use student::domain::entities::{CheckpointStatus, MusicianLevel, RequirementStatus};

    #[test]
    fn a_full_assessment_is_mapped() {
        let assessment = student::domain::entities::ProgressAssessment {
            checkpoints: vec![CheckpointStatus {
                level: MusicianLevel::YouthService,
                achieved: true,
                ready_to_advance: false,
                requirement: RequirementStatus {
                    msa_met: true,
                    method_met: false,
                },
            }],
            msa_relative_percent: 50.0,
            method_relative_percent: 25.0,
            combined_percent: 37.5,
            overall_checkpoint_percent: 20.0,
            next_level: Some(MusicianLevel::OfficialService),
        };

        let dto = ProgressAssessmentDto::from(assessment);

        assert_eq!(
            dto,
            ProgressAssessmentDto {
                checkpoints: vec![CheckpointStatusDto {
                    level: MusicianLevelDto::YouthService,
                    achieved: true,
                    ready_to_advance: false,
                    requirement: RequirementStatusDto {
                        msa_met: true,
                        method_met: false,
                    },
                }],
                msa_relative_percent: 50.0,
                method_relative_percent: 25.0,
                combined_percent: 37.5,
                overall_checkpoint_percent: 20.0,
                next_level: Some(MusicianLevelDto::OfficialService),
            }
        );
    }

    #[test]
    fn every_musician_level_is_mapped() {
        let cases = [
            (MusicianLevel::Candidate, MusicianLevelDto::Candidate),
            (MusicianLevel::Practice, MusicianLevelDto::Practice),
            (MusicianLevel::YouthService, MusicianLevelDto::YouthService),
            (
                MusicianLevel::OfficialService,
                MusicianLevelDto::OfficialService,
            ),
            (MusicianLevel::Officialized, MusicianLevelDto::Officialized),
            (
                MusicianLevel::Unknown("EXÓTICO".to_owned()),
                MusicianLevelDto::Unknown {
                    raw: "EXÓTICO".to_owned(),
                },
            ),
        ];

        for (level, expected) in cases {
            assert_eq!(MusicianLevelDto::from(level), expected);
        }
    }

    #[test]
    fn no_next_level_maps_to_none() {
        let assessment = student::domain::entities::ProgressAssessment {
            checkpoints: Vec::new(),
            msa_relative_percent: 0.0,
            method_relative_percent: 0.0,
            combined_percent: 0.0,
            overall_checkpoint_percent: 0.0,
            next_level: None,
        };

        let dto = ProgressAssessmentDto::from(assessment);

        assert_eq!(dto.next_level, None);
    }
}
