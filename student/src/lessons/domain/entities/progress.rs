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
pub struct CheckpointStatus {
    pub level: MusicianLevel,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub requirement: RequirementStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressAssessment {
    pub checkpoints: Vec<CheckpointStatus>,
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
    method: &[Lesson],
) -> Result<ProgressAssessment, AssessError> {
    if let MusicianLevel::Unknown(raw) = assigned_level {
        return Err(AssessError::UnknownLevel(raw.clone()));
    }

    let catalog: InstrumentRequirements = InstrumentRequirements::for_instrument(&instrument)
        .ok_or(AssessError::UnpublishedRequirements(instrument))?;

    let theory_recorded: f64 = max_field(msa_lessons.iter().map(|l| &l.phase));
    let method_recorded: RecordedProgress = recorded_progress(method);

    let checkpoints: Vec<CheckpointStatus> = MusicianLevel::ASCENDING
        .iter()
        .map(|level| {
            build_checkpoint(
                assigned_level,
                level,
                &catalog,
                theory_recorded,
                &method_recorded,
            )
        })
        .collect();

    let target: Option<(&MusicianLevel, &TestRequirement)> = MusicianLevel::ASCENDING
        .iter()
        .zip(checkpoints.iter())
        .find_map(|(level, checkpoint)| {
            if checkpoint.achieved {
                return None;
            }
            catalog
                .requirement_for(level)
                .map(|requirement| (level, requirement))
        });

    let (msa_relative, method_relative, combined, next_level) = match target {
        Some((level, requirement)) => {
            let msa_relative: f64 =
                percentage(theory_recorded, f64::from(requirement.theory.msa_phase));
            let (method_relative, _) = assess_method(requirement, &method_recorded);
            let combined: f64 = msa_relative.midpoint(method_relative);

            (msa_relative, method_relative, combined, Some(level.clone()))
        }
        None => (100.0, 100.0, 100.0, None),
    };

    let achieved_count: usize = checkpoints.iter().filter(|c| c.achieved).count();
    let all_achieved: bool = checkpoints.iter().all(|c| c.achieved);
    let overall_checkpoint: f64 = if all_achieved {
        100.0
    } else {
        (as_f64(achieved_count) + combined / 100.0) / as_f64(checkpoints.len()) * 100.0
    };

    Ok(ProgressAssessment {
        checkpoints,
        msa_relative_percent: msa_relative,
        method_relative_percent: method_relative,
        combined_percent: combined,
        overall_checkpoint_percent: overall_checkpoint,
        next_level,
    })
}

struct RecordedProgress {
    page: f64,
    lesson: f64,
    phase: f64,
}

fn recorded_progress(lessons: &[Lesson]) -> RecordedProgress {
    RecordedProgress {
        page: max_field(lessons.iter().map(|l| &l.page)),
        lesson: max_field(lessons.iter().map(|l| &l.lesson)),
        phase: max_field(lessons.iter().map(|l| &l.phase)),
    }
}

fn max_field<'a>(ranges: impl Iterator<Item = &'a Option<Range>>) -> f64 {
    ranges
        .filter_map(|r| r.as_ref())
        .flat_map(|r| [r.from.trim(), r.to.trim()])
        .filter_map(|v| v.parse::<f64>().ok())
        .fold(0.0_f64, f64::max)
}

fn build_checkpoint(
    assigned_level: &MusicianLevel,
    level: &MusicianLevel,
    catalog: &InstrumentRequirements,
    theory_recorded: f64,
    method_recorded: &RecordedProgress,
) -> CheckpointStatus {
    let achieved: bool = assigned_level.rank() >= level.rank();

    let Some(requirement) = catalog.requirement_for(level) else {
        return CheckpointStatus {
            level: level.clone(),
            achieved,
            ready_to_advance: false,
            requirement: RequirementStatus {
                msa_met: true,
                method_met: true,
            },
        };
    };

    let msa_met: bool = theory_recorded >= f64::from(requirement.theory.msa_phase);
    let (_, method_met) = assess_method(requirement, method_recorded);

    CheckpointStatus {
        level: level.clone(),
        achieved,
        ready_to_advance: !achieved && msa_met && method_met,
        requirement: RequirementStatus {
            msa_met,
            method_met,
        },
    }
}

fn assess_method(requirement: &TestRequirement, recorded: &RecordedProgress) -> (f64, bool) {
    let assessments: Vec<(f64, bool)> = requirement
        .method_alternatives
        .iter()
        .map(|alternative| assess_alternative(alternative, recorded))
        .collect();

    let percent: f64 = assessments.iter().map(|(p, _)| *p).fold(0.0, f64::max);
    let met: bool = assessments.iter().any(|(_, met)| *met);

    (percent, met)
}

fn assess_alternative(alternative: &MethodAlternative, recorded: &RecordedProgress) -> (f64, bool) {
    let measurements: Vec<Option<Measurement>> = alternative
        .components
        .iter()
        .map(|component| component.milestone.measure(recorded))
        .collect();

    let measured_percents: Vec<f64> = measurements.iter().flatten().map(|m| m.percent).collect();
    let all_met: bool = measurements.iter().all(|m| m.is_some_and(|m| m.met));

    let percent: f64 = if measured_percents.is_empty() {
        0.0
    } else {
        measured_percents.iter().sum::<f64>() / as_f64(measured_percents.len())
    };

    (percent, all_met)
}

#[derive(Clone, Copy)]
struct Measurement {
    percent: f64,
    met: bool,
}

impl MethodMilestone {
    fn measure(&self, recorded: &RecordedProgress) -> Option<Measurement> {
        match *self {
            Self::Page(target) => Some(Measurement::against(recorded.page, target)),
            Self::Lesson(target) => Some(Measurement::against(recorded.lesson, target)),
            Self::PageAndLesson { page, lesson } => {
                let page: Measurement = Measurement::against(recorded.page, page);
                let lesson: Measurement = Measurement::against(recorded.lesson, lesson);

                Some(Measurement {
                    percent: page.percent.midpoint(lesson.percent),
                    met: page.met && lesson.met,
                })
            }
            Self::Phase(target) => Some(Measurement::against(recorded.phase, target)),
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

fn percentage(current: f64, max: f64) -> f64 {
    (current / max * 100.0).min(100.0)
}

fn as_f64(count: usize) -> f64 {
    f64::from(u32::try_from(count).unwrap_or(u32::MAX))
}
