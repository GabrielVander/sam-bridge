use student::domain::entities as student_entities;

pub struct RequirementStatusDto {
    pub msa_met: bool,
    pub method_met: bool,
}

pub struct CheckpointStatusDto {
    pub level: String,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub requirement: RequirementStatusDto,
}

pub struct ProgressAssessmentDto {
    pub checkpoints: Vec<CheckpointStatusDto>,
    pub msa_relative_percent: f64,
    pub method_relative_percent: f64,
    pub combined_percent: f64,
    pub overall_checkpoint_percent: f64,
    pub next_level: Option<String>,
}

pub enum AssessStudentProgressOutcome {
    Success(ProgressAssessmentDto),
    NoInstrumentAssigned,
    UnknownLevel(String),
    Failure(String),
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
