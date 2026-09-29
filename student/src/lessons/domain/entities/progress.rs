use crate::lessons::domain::entities::{
    InstrumentRequirements, Lesson, MethodAlternative, MethodMilestone, Range, TestRequirement,
};
use crate::shared::domain::entities::{Instrument, MusicianLevel};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssessError {
    #[error("unrecognized musician level {0:?}")]
    UnknownLevel(String),
    #[error("no published test requirements for {0:?}")]
    UnpublishedRequirements(Instrument),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementStatus {
    pub msa_met: bool,
    pub method_met: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Checkpoint {
    pub level: MusicianLevel,
    pub status: CheckpointStatus,
    pub requirement: RequirementStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckpointStatus {
    Achieved,
    ReadyForExam,
    Pending,
}

impl Checkpoint {
    #[must_use]
    pub fn is_achieved(&self) -> bool {
        self.status == CheckpointStatus::Achieved
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressAssessment {
    pub checkpoints: Vec<Checkpoint>,
    pub msa_relative_percent: f64,
    pub method_relative_percent: f64,
    pub combined_percent: f64,
    pub overall_checkpoint_percent: f64,
    pub next_level: Option<MusicianLevel>,
}

pub fn assess(
    assigned_level: &MusicianLevel,
    instrument: Instrument,
    msa_lessons: &[Lesson],
    method_lessons: &[Lesson],
) -> Result<ProgressAssessment, AssessError> {
    if let MusicianLevel::Unknown(raw) = assigned_level {
        return Err(AssessError::UnknownLevel(raw.clone()));
    }

    let catalog: InstrumentRequirements = InstrumentRequirements::for_instrument(&instrument)
        .ok_or(AssessError::UnpublishedRequirements(instrument))?;
    let recorded: RecordedProgress = RecordedProgress::from_lessons(msa_lessons, method_lessons);

    let checkpoints: Vec<Checkpoint> = MusicianLevel::ASCENDING
        .iter()
        .map(|level| build_checkpoint(assigned_level, level, &catalog, &recorded))
        .collect();
    let next_exam: ExamProgress = progress_towards_next_exam(&checkpoints, &catalog, &recorded);
    let overall_checkpoint_percent: f64 =
        overall_checkpoint_percent(&checkpoints, next_exam.combined_percent);

    Ok(ProgressAssessment {
        checkpoints,
        msa_relative_percent: next_exam.msa_percent,
        method_relative_percent: next_exam.method_percent,
        combined_percent: next_exam.combined_percent,
        overall_checkpoint_percent,
        next_level: next_exam.level,
    })
}

struct RecordedProgress {
    theory_phase: f64,
    method_page: f64,
    method_lesson: f64,
    method_phase: f64,
}

impl RecordedProgress {
    fn from_lessons(msa_lessons: &[Lesson], method_lessons: &[Lesson]) -> Self {
        Self {
            theory_phase: highest(msa_lessons.iter().map(|lesson| &lesson.phase)),
            method_page: highest(method_lessons.iter().map(|lesson| &lesson.page)),
            method_lesson: highest(method_lessons.iter().map(|lesson| &lesson.lesson)),
            method_phase: highest(method_lessons.iter().map(|lesson| &lesson.phase)),
        }
    }
}

fn highest<'a>(ranges: impl Iterator<Item = &'a Option<Range>>) -> f64 {
    ranges
        .flatten()
        .flat_map(|range| [range.from.trim(), range.to.trim()])
        .filter_map(|value| value.parse::<f64>().ok())
        .fold(0.0_f64, f64::max)
}

fn build_checkpoint(
    assigned_level: &MusicianLevel,
    level: &MusicianLevel,
    catalog: &InstrumentRequirements,
    recorded: &RecordedProgress,
) -> Checkpoint {
    let achieved: bool = assigned_level.rank() >= level.rank();

    let Some(requirement) = catalog.requirement_for(level) else {
        return Checkpoint {
            level: level.clone(),
            status: checkpoint_status(achieved, false),
            requirement: RequirementStatus {
                msa_met: true,
                method_met: true,
            },
        };
    };

    let msa_met: bool = measure_theory(requirement, recorded).met;
    let method_met: bool = measure_method(requirement, recorded).met;

    Checkpoint {
        level: level.clone(),
        status: checkpoint_status(achieved, msa_met && method_met),
        requirement: RequirementStatus {
            msa_met,
            method_met,
        },
    }
}

const fn checkpoint_status(achieved: bool, requirement_met: bool) -> CheckpointStatus {
    match (achieved, requirement_met) {
        (true, _) => CheckpointStatus::Achieved,
        (false, true) => CheckpointStatus::ReadyForExam,
        (false, false) => CheckpointStatus::Pending,
    }
}

fn measure_theory(requirement: &TestRequirement, recorded: &RecordedProgress) -> Measurement {
    Measurement::against(recorded.theory_phase, requirement.theory.msa_phase)
}

fn measure_method(requirement: &TestRequirement, recorded: &RecordedProgress) -> Measurement {
    let alternatives: Vec<Measurement> = requirement
        .method_alternatives
        .iter()
        .map(|alternative| measure_alternative(alternative, recorded))
        .collect();

    Measurement {
        percent: alternatives
            .iter()
            .map(|alternative| alternative.percent)
            .fold(0.0, f64::max),
        met: alternatives.iter().any(|alternative| alternative.met),
    }
}

fn measure_alternative(
    alternative: &MethodAlternative,
    recorded: &RecordedProgress,
) -> Measurement {
    let components: Vec<Option<Measurement>> = alternative
        .components
        .iter()
        .map(|component| component.milestone.measure(recorded))
        .collect();

    let measured_percents: Vec<f64> = components
        .iter()
        .flatten()
        .map(|component| component.percent)
        .collect();

    Measurement {
        percent: average(&measured_percents),
        met: components
            .iter()
            .all(|component| component.is_some_and(|component| component.met)),
    }
}

fn average(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    values.iter().sum::<f64>() / as_f64(values.len())
}

#[derive(Clone, Copy)]
struct Measurement {
    percent: f64,
    met: bool,
}

impl MethodMilestone {
    fn measure(&self, recorded: &RecordedProgress) -> Option<Measurement> {
        match *self {
            Self::Page(target) => Some(Measurement::against(recorded.method_page, target)),
            Self::Lesson(target) => Some(Measurement::against(recorded.method_lesson, target)),
            Self::PageAndLesson { page, lesson } => {
                let page: Measurement = Measurement::against(recorded.method_page, page);
                let lesson: Measurement = Measurement::against(recorded.method_lesson, lesson);

                Some(Measurement {
                    percent: page.percent.midpoint(lesson.percent),
                    met: page.met && lesson.met,
                })
            }
            Self::Phase(target) => Some(Measurement::against(recorded.method_phase, target)),
            Self::Module(_) | Self::ExerciseRange { .. } | Self::Complete | Self::Unmeasured => {
                None
            }
        }
    }
}

impl Measurement {
    fn against(recorded: f64, target: u32) -> Self {
        let target: f64 = f64::from(target);

        Self {
            percent: percentage(recorded, target),
            met: recorded >= target,
        }
    }
}

struct ExamProgress {
    level: Option<MusicianLevel>,
    msa_percent: f64,
    method_percent: f64,
    combined_percent: f64,
}

impl ExamProgress {
    const fn journey_complete() -> Self {
        Self {
            level: None,
            msa_percent: 100.0,
            method_percent: 100.0,
            combined_percent: 100.0,
        }
    }
}

fn progress_towards_next_exam(
    checkpoints: &[Checkpoint],
    catalog: &InstrumentRequirements,
    recorded: &RecordedProgress,
) -> ExamProgress {
    checkpoints
        .iter()
        .filter(|checkpoint| !checkpoint.is_achieved())
        .find_map(|checkpoint| {
            catalog
                .requirement_for(&checkpoint.level)
                .map(|requirement| exam_progress(requirement, recorded))
        })
        .unwrap_or_else(ExamProgress::journey_complete)
}

fn exam_progress(requirement: &TestRequirement, recorded: &RecordedProgress) -> ExamProgress {
    let msa_percent: f64 = measure_theory(requirement, recorded).percent;
    let method_percent: f64 = measure_method(requirement, recorded).percent;

    ExamProgress {
        level: Some(requirement.level.clone()),
        msa_percent,
        method_percent,
        combined_percent: msa_percent.midpoint(method_percent),
    }
}

fn overall_checkpoint_percent(checkpoints: &[Checkpoint], next_exam_percent: f64) -> f64 {
    if checkpoints.iter().all(Checkpoint::is_achieved) {
        return 100.0;
    }

    let achieved: usize = checkpoints
        .iter()
        .filter(|checkpoint| checkpoint.is_achieved())
        .count();

    (as_f64(achieved) + next_exam_percent / 100.0) / as_f64(checkpoints.len()) * 100.0
}

fn percentage(current: f64, max: f64) -> f64 {
    (current / max * 100.0).min(100.0)
}

fn as_f64(count: usize) -> f64 {
    f64::from(u32::try_from(count).unwrap_or(u32::MAX))
}
