use crate::lessons::domain::entities::{
    InstrumentRequirements, Lesson, MethodAlternative, MethodMilestone, Range, TestRequirement,
};
use crate::shared::domain::entities::{Instrument, MusicianLevel};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssessError {
    #[error("UnknownLevel")]
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

const LEVELS: [MusicianLevel; 5] = [
    MusicianLevel::Candidate,
    MusicianLevel::Practice,
    MusicianLevel::YouthService,
    MusicianLevel::OfficialService,
    MusicianLevel::Officialized,
];

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

fn milestone_percent(milestone: &MethodMilestone, recorded: &RecordedProgress) -> Option<f64> {
    match *milestone {
        MethodMilestone::Page(target) => Some(pct(recorded.page, f64::from(target))),
        MethodMilestone::Lesson(target) => Some(pct(recorded.lesson, f64::from(target))),
        MethodMilestone::PageAndLesson { page, lesson } => Some(
            pct(recorded.page, f64::from(page)).midpoint(pct(recorded.lesson, f64::from(lesson))),
        ),
        MethodMilestone::Phase(target) => Some(pct(recorded.phase, f64::from(target))),
        MethodMilestone::Module(_)
        | MethodMilestone::ExerciseRange { .. }
        | MethodMilestone::Complete
        | MethodMilestone::Described(_) => None,
    }
}

fn milestone_met(milestone: &MethodMilestone, recorded: &RecordedProgress) -> Option<bool> {
    match *milestone {
        MethodMilestone::Page(target) => Some(recorded.page >= f64::from(target)),
        MethodMilestone::Lesson(target) => Some(recorded.lesson >= f64::from(target)),
        MethodMilestone::PageAndLesson { page, lesson } => {
            Some(recorded.page >= f64::from(page) && recorded.lesson >= f64::from(lesson))
        }
        MethodMilestone::Phase(target) => Some(recorded.phase >= f64::from(target)),
        MethodMilestone::Module(_)
        | MethodMilestone::ExerciseRange { .. }
        | MethodMilestone::Complete
        | MethodMilestone::Described(_) => None,
    }
}

fn assess_alternative(alternative: &MethodAlternative, recorded: &RecordedProgress) -> (f64, bool) {
    let mut measured_percents = Vec::new();
    let mut all_met = true;

    for component in &alternative.components {
        if let Some(percent) = milestone_percent(&component.milestone, recorded) {
            measured_percents.push(percent);
        }
        match milestone_met(&component.milestone, recorded) {
            Some(met) => all_met &= met,
            None => all_met = false,
        }
    }

    let percent = if measured_percents.is_empty() {
        0.0
    } else {
        measured_percents.iter().sum::<f64>() / as_f64(measured_percents.len())
    };

    (percent, all_met)
}

fn assess_method(requirement: &TestRequirement, recorded: &RecordedProgress) -> (f64, bool) {
    let assessments: Vec<(f64, bool)> = requirement
        .method_alternatives
        .iter()
        .map(|alternative| assess_alternative(alternative, recorded))
        .collect();

    let percent = assessments.iter().map(|(p, _)| *p).fold(0.0, f64::max);
    let met = assessments.iter().any(|(_, met)| *met);

    (percent, met)
}

fn build_checkpoint(
    assigned_level: &MusicianLevel,
    level: &MusicianLevel,
    catalog: &InstrumentRequirements,
    theory_recorded: f64,
    method_recorded: &RecordedProgress,
) -> CheckpointStatus {
    let achieved = assigned_level.rank() >= level.rank();

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

    let msa_met = theory_recorded >= f64::from(requirement.theory.msa_phase);
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

fn as_f64(count: usize) -> f64 {
    f64::from(u32::try_from(count).unwrap_or(u32::MAX))
}

fn max_field<'a>(ranges: impl Iterator<Item = &'a Option<Range>>) -> f64 {
    ranges
        .filter_map(|r| r.as_ref())
        .flat_map(|r| [r.from.trim(), r.to.trim()])
        .filter_map(|v| v.parse::<f64>().ok())
        .fold(0.0_f64, f64::max)
}

fn pct(current: f64, max: f64) -> f64 {
    (current / max * 100.0).min(100.0)
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
    let catalog = InstrumentRequirements::for_instrument(&instrument)
        .ok_or(AssessError::UnpublishedRequirements(instrument))?;

    let theory_recorded = max_field(msa_lessons.iter().map(|l| &l.phase));
    let method_recorded = recorded_progress(method);

    let checkpoints: Vec<CheckpointStatus> = LEVELS
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

    let target = LEVELS
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
            let msa_relative = pct(theory_recorded, f64::from(requirement.theory.msa_phase));
            let (method_relative, _) = assess_method(requirement, &method_recorded);
            let combined = msa_relative.midpoint(method_relative);
            (msa_relative, method_relative, combined, Some(level.clone()))
        }
        None => (100.0, 100.0, 100.0, None),
    };

    let achieved_count = checkpoints.iter().filter(|c| c.achieved).count();
    let all_achieved = checkpoints.iter().all(|c| c.achieved);
    let overall_checkpoint = if all_achieved {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::domain::entities::MethodComponent;

    fn msa_lesson(phase_from: &str, phase_to: &str) -> Lesson {
        Lesson {
            phase: Some(Range {
                from: phase_from.to_owned(),
                to: phase_to.to_owned(),
            }),
            ..Default::default()
        }
    }

    fn method_lesson(page: &str, lesson: &str) -> Lesson {
        Lesson {
            page: Some(Range {
                from: page.to_owned(),
                to: page.to_owned(),
            }),
            lesson: Some(Range {
                from: lesson.to_owned(),
                to: lesson.to_owned(),
            }),
            ..Default::default()
        }
    }

    fn method_phase_lesson(phase: &str) -> Lesson {
        Lesson {
            phase: Some(Range {
                from: phase.to_owned(),
                to: phase.to_owned(),
            }),
            ..Default::default()
        }
    }

    #[test]
    fn empty_lessons_yield_all_pending() {
        let report = assess(&MusicianLevel::Candidate, Instrument::Violin, &[], &[]).unwrap();

        assert_ne!(report.checkpoints, Vec::new());
        assert!(report.checkpoints[0].achieved);
        assert!(!report.meets_any_above());
        assert!((report.overall_checkpoint_percent - 20.0).abs() < 0.1);
        assert_eq!(report.msa_relative_percent, 0.0);
        assert_eq!(report.method_relative_percent, 0.0);
    }

    #[test]
    fn supreme_rule_assigned_level_auto_achieves_below_and_at() {
        let msa_lessons = vec![msa_lesson("3", "3")];
        let method = vec![method_lesson("10", "20")];

        let report = assess(
            &MusicianLevel::YouthService,
            Instrument::Violin,
            &msa_lessons,
            &method,
        )
        .unwrap();

        assert!(report.checkpoints[0].achieved);
        assert!(report.checkpoints[1].achieved);
        assert!(report.checkpoints[2].achieved);
        assert!(!report.checkpoints[3].achieved);
        assert!(!report.checkpoints[4].achieved);
    }

    #[test]
    fn requirements_met_but_not_assigned_shows_ready_to_advance() {
        let msa_lessons = vec![msa_lesson("12", "12")];
        let method = vec![method_lesson("46", "113")];

        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &method,
        )
        .unwrap();

        assert!(report.checkpoints[0].achieved);
        assert!(!report.checkpoints[1].achieved);
        assert!(!report.checkpoints[1].ready_to_advance);
        assert!(!report.checkpoints[2].achieved);
        assert!(report.checkpoints[2].ready_to_advance);
    }

    #[test]
    fn best_alternative_is_picked_even_when_it_is_not_the_first_listed() {
        let method = vec![method_lesson("0", "6")];

        let report = assess(&MusicianLevel::Candidate, Instrument::Violin, &[], &method).unwrap();

        assert!(report.method_relative_percent > 40.0);
    }

    #[test]
    fn meets_culto_oficial_requires_phase_16() {
        let msa_lessons = vec![msa_lesson("15", "16")];
        let method = vec![method_lesson("67", "162")];

        let report = assess(
            &MusicianLevel::YouthService,
            Instrument::Violin,
            &msa_lessons,
            &method,
        )
        .unwrap();

        assert!(report.checkpoints[3].ready_to_advance);
    }

    #[test]
    fn officialization_is_unverifiable_from_data_alone() {
        let msa_lessons = vec![msa_lesson("16", "16")];
        let method = vec![method_lesson("999", "999")];

        let report = assess(
            &MusicianLevel::OfficialService,
            Instrument::Violin,
            &msa_lessons,
            &method,
        )
        .unwrap();

        assert!(!report.checkpoints[4].ready_to_advance);
        assert!(!report.checkpoints[4].requirement.method_met);
    }

    #[test]
    fn all_levels_achieved_reaches_full_percentages() {
        let report = assess(&MusicianLevel::Officialized, Instrument::Violin, &[], &[]).unwrap();

        assert!(report.checkpoints.iter().all(|c| c.achieved));
        assert_eq!(report.next_level, None);
        assert!((report.overall_checkpoint_percent - 100.0).abs() < f64::EPSILON);
        assert!((report.combined_percent - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn absent_fields_contribute_zero() {
        let msa_lessons = vec![Lesson::default()];
        let method = vec![Lesson::default()];

        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &method,
        )
        .unwrap();

        assert_eq!(report.msa_relative_percent, 0.0);
        assert_eq!(report.method_relative_percent, 0.0);
        assert_eq!(report.combined_percent, 0.0);
    }

    #[test]
    fn unparseable_values_contribute_zero() {
        let msa_lessons = vec![msa_lesson("abc", "def")];
        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &[],
        )
        .unwrap();

        assert_eq!(report.msa_relative_percent, 0.0);
    }

    #[test]
    fn scan_all_takes_max_not_last() {
        let msa_lessons = vec![
            msa_lesson("3", "3"),
            msa_lesson("14", "14"),
            msa_lesson("7", "7"),
        ];
        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &[],
        )
        .unwrap();

        assert!((report.msa_relative_percent - 100.0).abs() < 0.1);
        assert_eq!(report.next_level, Some(MusicianLevel::YouthService));
    }

    #[test]
    fn range_dash_takes_upper_bound() {
        let msa_lessons = vec![msa_lesson("11", "13")];
        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &[],
        )
        .unwrap();

        assert!((report.msa_relative_percent - 100.0).abs() < 0.1);
    }

    #[test]
    fn msa_percent_is_phase_based() {
        let msa_lessons = vec![msa_lesson("8", "8")];
        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &[],
        )
        .unwrap();

        assert!((report.msa_relative_percent - 66.6).abs() < 0.5);
    }

    #[test]
    fn phase_based_method_milestone_is_measurable() {
        let method = vec![method_phase_lesson("10")];
        let report = assess(&MusicianLevel::Candidate, Instrument::Flute, &[], &method).unwrap();

        assert!(report.method_relative_percent > 0.0);
    }

    #[test]
    fn unknown_level_returns_err_with_raw() {
        let err = assess(
            &MusicianLevel::Unknown("EXÓTICO".to_owned()),
            Instrument::Violin,
            &[],
            &[],
        )
        .unwrap_err();
        assert_eq!(err, AssessError::UnknownLevel("EXÓTICO".to_owned()));
        assert!(err.to_string().contains("UnknownLevel"));
    }

    #[test]
    fn unknown_level_does_not_calculate_even_with_high_lessons() {
        let msa_lessons = vec![msa_lesson("16", "16")];
        let method = vec![method_lesson("80", "214")];
        let result = assess(
            &MusicianLevel::Unknown("Foo".to_owned()),
            Instrument::Violin,
            &msa_lessons,
            &method,
        );
        assert!(result.is_err());
    }

    #[test]
    fn unpublished_instrument_requirements_return_err() {
        let result = assess(
            &MusicianLevel::Candidate,
            Instrument::AltoClarinet,
            &[],
            &[],
        );
        assert_eq!(
            result,
            Err(AssessError::UnpublishedRequirements(
                Instrument::AltoClarinet
            ))
        );
    }

    impl ProgressAssessment {
        fn meets_any_above(&self) -> bool {
            self.checkpoints.iter().any(|c| c.ready_to_advance)
        }
    }

    // The rules for comparing recorded progress with a method milestone are pure,
    // so they are pinned directly with milestones built here: that keeps them
    // independent of the published requirement tables.

    fn recorded(page: f64, lesson: f64, phase: f64) -> RecordedProgress {
        RecordedProgress {
            page,
            lesson,
            phase,
        }
    }

    #[test]
    fn a_page_milestone_is_met_from_the_target_page_on() {
        let milestone = MethodMilestone::Page(10);

        assert_eq!(
            milestone_met(&milestone, &recorded(10.0, 0.0, 0.0)),
            Some(true)
        );
        assert_eq!(
            milestone_met(&milestone, &recorded(9.0, 0.0, 0.0)),
            Some(false)
        );
    }

    #[test]
    fn a_lesson_milestone_is_met_from_the_target_lesson_on() {
        let milestone = MethodMilestone::Lesson(20);

        assert_eq!(
            milestone_met(&milestone, &recorded(0.0, 20.0, 0.0)),
            Some(true)
        );
        assert_eq!(
            milestone_met(&milestone, &recorded(0.0, 19.0, 0.0)),
            Some(false)
        );
    }

    #[test]
    fn a_phase_milestone_is_met_from_the_target_phase_on() {
        let milestone = MethodMilestone::Phase(4);

        assert_eq!(
            milestone_met(&milestone, &recorded(0.0, 0.0, 4.0)),
            Some(true)
        );
        assert_eq!(
            milestone_met(&milestone, &recorded(0.0, 0.0, 3.0)),
            Some(false)
        );
    }

    #[test]
    fn a_page_and_lesson_milestone_needs_both_targets_reached() {
        let milestone = MethodMilestone::PageAndLesson {
            page: 10,
            lesson: 20,
        };

        assert_eq!(
            milestone_met(&milestone, &recorded(10.0, 20.0, 0.0)),
            Some(true)
        );
        assert_eq!(
            milestone_met(&milestone, &recorded(10.0, 19.0, 0.0)),
            Some(false),
            "the page alone is not enough"
        );
        assert_eq!(
            milestone_met(&milestone, &recorded(9.0, 20.0, 0.0)),
            Some(false),
            "the lesson alone is not enough"
        );
    }

    #[test]
    fn milestones_that_cannot_be_measured_from_the_lessons_are_neither_met_nor_unmet() {
        for milestone in [
            MethodMilestone::Module(2),
            MethodMilestone::ExerciseRange { from: 1, to: 9 },
            MethodMilestone::Complete,
            MethodMilestone::Described("a prova prática"),
        ] {
            assert_eq!(milestone_met(&milestone, &recorded(99.0, 99.0, 99.0)), None);
            assert_eq!(
                milestone_percent(&milestone, &recorded(99.0, 99.0, 99.0)),
                None
            );
        }
    }

    fn component(milestone: MethodMilestone) -> MethodComponent {
        MethodComponent {
            method_name: "Método",
            milestone,
        }
    }

    #[test]
    fn an_alternative_is_met_only_when_every_component_is() {
        let alternative = MethodAlternative {
            components: vec![
                component(MethodMilestone::Page(10)),
                component(MethodMilestone::Lesson(20)),
            ],
        };

        let (_, met) = assess_alternative(&alternative, &recorded(10.0, 19.0, 0.0));

        assert!(!met, "one unmet component keeps the alternative unmet");
        let (_, met) = assess_alternative(&alternative, &recorded(10.0, 20.0, 0.0));
        assert!(met);
    }

    #[test]
    fn an_alternative_with_a_component_that_cannot_be_measured_is_never_met() {
        let alternative = MethodAlternative {
            components: vec![
                component(MethodMilestone::Page(10)),
                component(MethodMilestone::Complete),
            ],
        };

        let (percent, met) = assess_alternative(&alternative, &recorded(10.0, 0.0, 0.0));

        assert!(!met);
        assert_eq!(percent, 100.0, "only the measurable component counts");
    }

    #[test]
    fn an_alternatives_percent_is_the_average_of_its_measurable_components() {
        let alternative = MethodAlternative {
            components: vec![
                component(MethodMilestone::Page(20)),
                component(MethodMilestone::Lesson(10)),
            ],
        };

        let (percent, _) = assess_alternative(&alternative, &recorded(10.0, 10.0, 0.0));

        assert_eq!(percent, 75.0, "the average of 50% and 100%");
    }

    #[test]
    fn overall_progress_adds_the_share_of_the_next_level_already_covered() {
        // Candidate: 1 of 5 checkpoints achieved. Towards Practice, MSA is at
        // 6 of 12 phases (50%) and the method at 0%, so the combined progress
        // is 25% and the journey is (1 + 0.25) / 5 = 25% complete.
        let msa_lessons = vec![msa_lesson("6", "6")];

        let report = assess(
            &MusicianLevel::Candidate,
            Instrument::Violin,
            &msa_lessons,
            &[],
        )
        .unwrap();

        assert_eq!(report.combined_percent, 25.0);
        assert_eq!(report.overall_checkpoint_percent, 25.0);
    }
}
