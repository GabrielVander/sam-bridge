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
