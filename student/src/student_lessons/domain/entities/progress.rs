use crate::{
    domain::entities::MusicianLevel,
    student_lessons::domain::entities::{Lesson, Range},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("UnknownLevel")]
pub struct UnknownLevel(pub String);

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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
    pub fn violin_schmoll() -> Self {
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

#[derive(Debug, Clone, PartialEq)]
pub struct CheckpointStatus {
    pub level: MusicianLevel,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub msa_requirement_met: bool,
    pub method_requirement_met: bool,
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

    let checkpoints: Vec<CheckpointStatus> = LEVEL_PATH
        .iter()
        .map(|def| {
            let assigned_rank = assigned_level.rank();
            let checkpoint_rank = def.level.rank();
            let achieved = assigned_rank >= checkpoint_rank;

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
                msa_requirement_met: msa_met,
                method_requirement_met: method_met,
            }
        })
        .collect();

    let achieved_count = checkpoints.iter().filter(|c| c.achieved).count();
    let next_idx = checkpoints
        .iter()
        .enumerate()
        .find(|(_, c)| !c.achieved)
        .map(|(i, _)| i)
        .unwrap_or(checkpoints.len() - 1);
    let next_measurable_idx = (next_idx..checkpoints.len())
        .find(|&i| {
            LEVEL_PATH[i].msa_phase.is_some()
                || method_threshold_for_level(&LEVEL_PATH[i].level, profile) != (0, 0)
        })
        .unwrap_or(next_idx);
    let target_idx = next_measurable_idx;
    let prev_idx = target_idx.saturating_sub(1);
    // Ensure prev is the last achieved before target, or 0
    let prev_idx = if checkpoints[prev_idx].achieved {
        prev_idx
    } else {
        0
    };
    let next_def = &LEVEL_PATH[target_idx];
    let prev_msa = LEVEL_PATH[prev_idx].msa_phase.unwrap_or(0.0);
    let next_msa = next_def.msa_phase.unwrap_or(prev_msa);
    let delta_msa = (next_msa - prev_msa).max(0.0);
    let msa_relative = if delta_msa == 0.0 {
        0.0
    } else {
        pct((highest_msa_phase - prev_msa).max(0.0), delta_msa)
    };

    let (prev_page, prev_lesson) = method_threshold_for_level(&LEVEL_PATH[prev_idx].level, profile);
    let (next_page, next_lesson) = method_threshold_for_level(&next_def.level, profile);
    let delta_page = next_page.saturating_sub(prev_page) as f64;
    let delta_lesson = next_lesson.saturating_sub(prev_lesson) as f64;
    let page_relative = if delta_page == 0.0 {
        0.0
    } else {
        pct(
            (highest_method_page - prev_page as f64).max(0.0),
            delta_page,
        )
    };
    let lesson_relative = if delta_lesson == 0.0 {
        0.0
    } else {
        pct(
            (highest_method_lesson - prev_lesson as f64).max(0.0),
            delta_lesson,
        )
    };
    let method_relative = (page_relative + lesson_relative) / 2.0;
    let all_zero_delta = delta_msa == 0.0 && delta_page == 0.0 && delta_lesson == 0.0;
    let combined = if all_zero_delta {
        0.0
    } else if delta_msa == 0.0 {
        method_relative
    } else {
        (msa_relative + method_relative) / 2.0
    };
    let overall_checkpoint = if checkpoints.iter().all(|c| c.achieved) {
        100.0
    } else {
        (achieved_count as f64 + combined / 100.0) / checkpoints.len() as f64 * 100.0
    };
    let next_level = if checkpoints.iter().all(|c| c.achieved) {
        None
    } else {
        Some(next_def.level.clone())
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

fn method_req_met(level: &MusicianLevel, profile: &MethodProfile, page: f64, lesson: f64) -> bool {
    match level {
        MusicianLevel::Candidate | MusicianLevel::Practice => true,
        MusicianLevel::YouthService => {
            page >= profile.youth_service_page as f64
                && lesson >= profile.youth_service_lesson as f64
        }
        MusicianLevel::OfficialService => {
            page >= profile.culto_oficial_page as f64
                && lesson >= profile.culto_oficial_lesson as f64
        }
        MusicianLevel::Officialized => {
            page >= profile.total_pages as f64 && lesson >= profile.total_lessons as f64
        }
        MusicianLevel::Unknown(_) => unreachable!("assess early-returned for Unknown"),
    }
}

fn method_threshold_for_level(level: &MusicianLevel, profile: &MethodProfile) -> (u32, u32) {
    match level {
        MusicianLevel::Candidate | MusicianLevel::Practice => (0, 0),
        MusicianLevel::YouthService => (profile.youth_service_page, profile.youth_service_lesson),
        MusicianLevel::OfficialService => {
            (profile.culto_oficial_page, profile.culto_oficial_lesson)
        }
        MusicianLevel::Officialized => (profile.total_pages, profile.total_lessons),
        MusicianLevel::Unknown(_) => (0, 0),
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

        assert!(!report.checkpoints.is_empty());
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
