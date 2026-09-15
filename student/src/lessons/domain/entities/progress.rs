use crate::{
    domain::entities::MusicianLevel,
    lessons::domain::entities::{Lesson, Range},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("UnknownLevel")]
pub struct UnknownLevel(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instrument {
    Violin,
}

pub struct LevelDefinition {
    pub level: MusicianLevel,
    pub msa_phase: Option<f64>,
}

pub const LEVEL_PATH: &[LevelDefinition] = &[
    LevelDefinition {
        level: MusicianLevel::Candidate,
        msa_phase: None,
    },
    LevelDefinition {
        level: MusicianLevel::Practice,
        msa_phase: None,
    },
    LevelDefinition {
        level: MusicianLevel::YouthService,
        msa_phase: Some(12.0),
    },
    LevelDefinition {
        level: MusicianLevel::OfficialService,
        msa_phase: Some(16.0),
    },
    LevelDefinition {
        level: MusicianLevel::Officialized,
        msa_phase: Some(16.0),
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodProfile {
    pub instrument: Instrument,
    pub method_name: &'static str,
    pub total_pages: u32,
    pub total_lessons: u32,
    pub youth_service_page: u32,
    pub youth_service_lesson: u32,
    pub culto_oficial_page: u32,
    pub culto_oficial_lesson: u32,
}

impl MethodProfile {
    #[must_use]
    pub const fn violin_schmoll() -> Self {
        Self {
            instrument: Instrument::Violin,
            method_name: "MÉTODO CCB - SCHIMOLL",
            total_pages: 80,
            total_lessons: 214,
            youth_service_page: 46,
            youth_service_lesson: 113,
            culto_oficial_page: 67,
            culto_oficial_lesson: 162,
        }
    }
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
#[allow(non_snake_case)]
pub struct ProgressAssessment {
    pub checkpoints: Vec<CheckpointStatus>,
    pub msaRelativePercent: f64,
    pub methodRelativePercent: f64,
    pub combinedPercent: f64,
    pub overallCheckpointPercent: f64,
    pub nextLevel: Option<MusicianLevel>,
}

// `.get(idx)` on `LEVEL_PATH` never actually misses: every index used against it
// is derived from `checkpoints`, which is built as a 1:1 map over `LEVEL_PATH`
// and so always has the same length. This fallback exists only because the
// panic-free indexing style below needs a concrete value for the type checker.
const FALLBACK_LEVEL_DEFINITION: &LevelDefinition = &LevelDefinition {
    level: MusicianLevel::Candidate,
    msa_phase: None,
};

fn level_definition_at(idx: usize) -> &'static LevelDefinition {
    LEVEL_PATH.get(idx).unwrap_or(FALLBACK_LEVEL_DEFINITION)
}

fn as_f64(count: usize) -> f64 {
    f64::from(u32::try_from(count).unwrap_or(u32::MAX))
}

pub fn assess(
    assigned_level: &MusicianLevel,
    approved: &[Lesson],
    method: &[Lesson],
    profile: &MethodProfile,
) -> Result<ProgressAssessment, UnknownLevel> {
    if let MusicianLevel::Unknown(raw) = assigned_level {
        return Err(UnknownLevel(raw.clone()));
    }
    let highest_msa_phase = max_field(approved.iter().map(|l| &l.phase));
    let _highest_msa_lesson = max_field(approved.iter().map(|l| &l.lesson));
    let highest_method_page = max_field(method.iter().map(|l| &l.page));
    let highest_method_lesson = max_field(method.iter().map(|l| &l.lesson));

    let checkpoints = build_checkpoints(
        assigned_level,
        highest_msa_phase,
        highest_method_page,
        highest_method_lesson,
        profile,
    );

    let (prev_idx, target_idx) = locate_target(&checkpoints, profile);
    let (msa_relative, method_relative, combined) = compute_relative_percentages(
        prev_idx,
        target_idx,
        highest_msa_phase,
        highest_method_page,
        highest_method_lesson,
        profile,
    );

    let achieved_count = checkpoints.iter().filter(|c| c.achieved).count();
    let all_achieved = checkpoints.iter().all(|c| c.achieved);

    let overall_checkpoint = if all_achieved {
        100.0
    } else {
        (as_f64(achieved_count) + combined / 100.0) / as_f64(checkpoints.len()) * 100.0
    };
    let next_level = if all_achieved {
        None
    } else {
        Some(level_definition_at(target_idx).level.clone())
    };

    Ok(ProgressAssessment {
        checkpoints,
        msaRelativePercent: msa_relative,
        methodRelativePercent: method_relative,
        combinedPercent: combined,
        overallCheckpointPercent: overall_checkpoint,
        nextLevel: next_level,
    })
}

fn build_checkpoints(
    assigned_level: &MusicianLevel,
    highest_msa_phase: f64,
    highest_method_page: f64,
    highest_method_lesson: f64,
    profile: &MethodProfile,
) -> Vec<CheckpointStatus> {
    LEVEL_PATH
        .iter()
        .map(|def| {
            let achieved = assigned_level.rank() >= def.level.rank();
            let msa_met = def.msa_phase.is_none_or(|min| highest_msa_phase >= min);
            let method_met = method_req_met(
                &def.level,
                profile,
                highest_method_page,
                highest_method_lesson,
            );
            let requirement_met = msa_met && method_met;
            let has_measurable_requirement = def.msa_phase.is_some();

            CheckpointStatus {
                level: def.level.clone(),
                achieved,
                ready_to_advance: !achieved && has_measurable_requirement && requirement_met,
                requirement: RequirementStatus {
                    msa_met,
                    method_met,
                },
            }
        })
        .collect()
}

fn locate_target(checkpoints: &[CheckpointStatus], profile: &MethodProfile) -> (usize, usize) {
    let next_idx = checkpoints
        .iter()
        .position(|c| !c.achieved)
        .unwrap_or_else(|| checkpoints.len().saturating_sub(1));

    let target_idx = (next_idx..checkpoints.len())
        .find(|&i| {
            LEVEL_PATH.get(i).is_some_and(|def| {
                def.msa_phase.is_some() || method_threshold_for_level(&def.level, profile) != (0, 0)
            })
        })
        .unwrap_or(next_idx);

    let prev_idx = target_idx.saturating_sub(1);
    let prev_idx = if checkpoints.get(prev_idx).is_some_and(|c| c.achieved) {
        prev_idx
    } else {
        0
    };

    (prev_idx, target_idx)
}

fn compute_relative_percentages(
    prev_idx: usize,
    target_idx: usize,
    highest_msa_phase: f64,
    highest_method_page: f64,
    highest_method_lesson: f64,
    profile: &MethodProfile,
) -> (f64, f64, f64) {
    let prev_def = level_definition_at(prev_idx);
    let next_def = level_definition_at(target_idx);

    let prev_msa = prev_def.msa_phase.unwrap_or(0.0);
    let next_msa = next_def.msa_phase.unwrap_or(prev_msa);
    let delta_msa = (next_msa - prev_msa).max(0.0);
    let msa_relative = if delta_msa == 0.0 {
        0.0
    } else {
        pct((highest_msa_phase - prev_msa).max(0.0), delta_msa)
    };

    let (prev_page, prev_lesson) = method_threshold_for_level(&prev_def.level, profile);
    let (next_page, next_lesson) = method_threshold_for_level(&next_def.level, profile);
    let delta_page = f64::from(next_page.saturating_sub(prev_page));
    let delta_lesson = f64::from(next_lesson.saturating_sub(prev_lesson));
    let page_relative = if delta_page == 0.0 {
        0.0
    } else {
        pct(
            (highest_method_page - f64::from(prev_page)).max(0.0),
            delta_page,
        )
    };
    let lesson_relative = if delta_lesson == 0.0 {
        0.0
    } else {
        pct(
            (highest_method_lesson - f64::from(prev_lesson)).max(0.0),
            delta_lesson,
        )
    };
    let method_relative = page_relative.midpoint(lesson_relative);
    let all_zero_delta = delta_msa == 0.0 && delta_page == 0.0 && delta_lesson == 0.0;
    let combined = if all_zero_delta {
        0.0
    } else if delta_msa == 0.0 {
        method_relative
    } else {
        msa_relative.midpoint(method_relative)
    };

    (msa_relative, method_relative, combined)
}

fn method_req_met(level: &MusicianLevel, profile: &MethodProfile, page: f64, lesson: f64) -> bool {
    let (required_page, required_lesson) = method_threshold_for_level(level, profile);
    page >= f64::from(required_page) && lesson >= f64::from(required_lesson)
}

const fn method_threshold_for_level(level: &MusicianLevel, profile: &MethodProfile) -> (u32, u32) {
    match level {
        MusicianLevel::Candidate | MusicianLevel::Practice | MusicianLevel::Unknown(_) => (0, 0),
        MusicianLevel::YouthService => (profile.youth_service_page, profile.youth_service_lesson),
        MusicianLevel::OfficialService => {
            (profile.culto_oficial_page, profile.culto_oficial_lesson)
        }
        MusicianLevel::Officialized => (profile.total_pages, profile.total_lessons),
    }
}

fn max_field<'a>(ranges: impl Iterator<Item = &'a Option<Range>>) -> f64 {
    ranges
        .filter_map(|r| r.as_ref())
        .flat_map(|r| [r.from.trim(), r.to.trim()])
        .filter_map(|v| v.parse::<f64>().ok())
        .fold(0.0_f64, f64::max)
}

fn pct(current: f64, max: f64) -> f64 {
    if max > 0.0 {
        (current / max * 100.0).min(100.0)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn empty_lessons_yield_all_pending() {
        let report = assess(
            &MusicianLevel::Candidate,
            &[],
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert_ne!(report.checkpoints, Vec::new());
        assert!(report.checkpoints[0].achieved);
        assert!(!report.meets_any_above());
        assert!((report.overallCheckpointPercent - 20.0).abs() < 0.1);
        assert_eq!(report.msaRelativePercent, 0.0);
        assert_eq!(report.methodRelativePercent, 0.0);
    }

    #[test]
    fn supreme_rule_assigned_level_auto_achieves_below_and_at() {
        let approved = vec![msa_lesson("3", "3")];
        let method = vec![method_lesson("10", "20")];

        let report = assess(
            &MusicianLevel::YouthService,
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
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
        let approved = vec![msa_lesson("12", "12")];
        let method = vec![method_lesson("46", "113")];

        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!(report.checkpoints[0].achieved);
        assert!(!report.checkpoints[1].achieved);
        assert!(!report.checkpoints[1].ready_to_advance);
        assert!(!report.checkpoints[2].achieved);
        assert!(report.checkpoints[2].ready_to_advance);
    }

    #[test]
    fn meets_culto_oficial_requires_phase_16() {
        let approved = vec![msa_lesson("15", "16")];
        let method = vec![method_lesson("67", "162")];

        let report = assess(
            &MusicianLevel::YouthService,
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!(report.checkpoints[3].ready_to_advance);
    }

    #[test]
    fn officialization_when_method_completed() {
        let approved = vec![msa_lesson("16", "16")];
        let method = vec![method_lesson("80", "214")];
        let student_level = MusicianLevel::OfficialService;

        let report = assess(
            &student_level,
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!(report.checkpoints[4].ready_to_advance);
        assert!((report.overallCheckpointPercent - 100.0).abs() < f64::EPSILON);
        assert!((report.combinedPercent - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn absent_fields_contribute_zero() {
        let approved = vec![Lesson::default()];
        let method = vec![Lesson::default()];

        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert_eq!(report.msaRelativePercent, 0.0);
        assert_eq!(report.methodRelativePercent, 0.0);
        assert_eq!(report.combinedPercent, 0.0);
    }

    #[test]
    fn unparseable_values_contribute_zero() {
        let approved = vec![msa_lesson("abc", "def")];
        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert_eq!(report.msaRelativePercent, 0.0);
    }

    #[test]
    fn scan_all_takes_max_not_last() {
        let approved = vec![
            msa_lesson("3", "3"),
            msa_lesson("14", "14"),
            msa_lesson("7", "7"),
        ];
        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!((report.msaRelativePercent - 100.0).abs() < 0.1);
        assert_eq!(report.nextLevel, Some(MusicianLevel::YouthService));
    }

    #[test]
    fn range_dash_takes_upper_bound() {
        let approved = vec![msa_lesson("11", "13")];
        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!((report.msaRelativePercent - 100.0).abs() < 0.1);
    }

    #[test]
    fn msa_percent_is_phase_based() {
        let approved = vec![msa_lesson("8", "8")];
        let report = assess(
            &MusicianLevel::Candidate,
            &approved,
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!((report.msaRelativePercent - 66.6).abs() < 0.5);
        assert!((report.combinedPercent - 33.3).abs() < 0.5);
    }

    #[test]
    fn overall_percent_is_method_lesson_based() {
        let method = vec![method_lesson("40", "107")];
        let report = assess(
            &MusicianLevel::Candidate,
            &[],
            &method,
            &MethodProfile::violin_schmoll(),
        )
        .unwrap();

        assert!((report.methodRelativePercent - 90.0).abs() < 5.0);
        assert!((report.overallCheckpointPercent - 29.0).abs() < 5.0);
    }

    #[test]
    fn violin_schmoll_profile_is_correct() {
        let p = MethodProfile::violin_schmoll();
        assert_eq!(p.total_pages, 80);
        assert_eq!(p.total_lessons, 214);
        assert_eq!(p.youth_service_page, 46);
        assert_eq!(p.youth_service_lesson, 113);
        assert_eq!(p.culto_oficial_page, 67);
        assert_eq!(p.culto_oficial_lesson, 162);
    }

    #[test]
    fn unknown_level_returns_err_with_raw() {
        let err = assess(
            &MusicianLevel::Unknown("EXÓTICO".to_owned()),
            &[],
            &[],
            &MethodProfile::violin_schmoll(),
        )
        .unwrap_err();
        assert_eq!(err.0, "EXÓTICO");
        assert!(err.to_string().contains("UnknownLevel"));
    }

    #[test]
    fn unknown_level_does_not_calculate_even_with_high_lessons() {
        let approved = vec![msa_lesson("16", "16")];
        let method = vec![method_lesson("80", "214")];
        let result = assess(
            &MusicianLevel::Unknown("Foo".to_owned()),
            &approved,
            &method,
            &MethodProfile::violin_schmoll(),
        );
        assert!(result.is_err());
    }

    impl ProgressAssessment {
        fn meets_any_above(&self) -> bool {
            self.checkpoints.iter().any(|c| c.ready_to_advance)
        }
    }
}
