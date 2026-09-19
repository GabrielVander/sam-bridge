use student::domain::entities as student_entities;

use crate::infra::ErrorReportDto;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementStatusDto {
    pub msa_met: bool,
    pub method_met: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckpointStatusDto {
    pub level: String,
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
    pub next_level: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssessStudentProgressOutcome {
    Success(ProgressAssessmentDto),
    NoInstrumentAssigned,
    UnknownLevel(String),
    NotAMusician,
    Failure(ErrorReportDto),
}

impl From<student_entities::ProgressAssessment> for ProgressAssessmentDto {
    fn from(assessment: student_entities::ProgressAssessment) -> Self {
        Self {
            checkpoints: assessment
                .checkpoints
                .into_iter()
                .map(CheckpointStatusDto::from)
                .collect(),
            msa_relative_percent: assessment.msaRelativePercent,
            method_relative_percent: assessment.methodRelativePercent,
            combined_percent: assessment.combinedPercent,
            overall_checkpoint_percent: assessment.overallCheckpointPercent,
            next_level: assessment.nextLevel.map(|level| level.name()),
        }
    }
}

impl From<student_entities::CheckpointStatus> for CheckpointStatusDto {
    fn from(checkpoint: student_entities::CheckpointStatus) -> Self {
        Self {
            level: checkpoint.level.name(),
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
    use super::{CheckpointStatusDto, ProgressAssessmentDto, RequirementStatusDto};
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
            msaRelativePercent: 50.0,
            methodRelativePercent: 25.0,
            combinedPercent: 37.5,
            overallCheckpointPercent: 20.0,
            nextLevel: Some(MusicianLevel::OfficialService),
        };

        let dto = ProgressAssessmentDto::from(assessment);

        assert_eq!(
            dto,
            ProgressAssessmentDto {
                checkpoints: vec![CheckpointStatusDto {
                    level: MusicianLevel::YouthService.name(),
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
                next_level: Some(MusicianLevel::OfficialService.name()),
            }
        );
    }

    #[test]
    fn no_next_level_maps_to_none() {
        let assessment = student::domain::entities::ProgressAssessment {
            checkpoints: Vec::new(),
            msaRelativePercent: 0.0,
            methodRelativePercent: 0.0,
            combinedPercent: 0.0,
            overallCheckpointPercent: 0.0,
            nextLevel: None,
        };

        let dto = ProgressAssessmentDto::from(assessment);

        assert_eq!(dto.next_level, None);
    }
}
