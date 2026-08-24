use crate::features::student_lessons::domain::entities::{Lesson, Range};
use crate::features::student_roster::domain::entities::MusicianLevel;

#[derive(Debug, Clone, PartialEq)]
pub enum Instrument {
    Violin,
}

/// Universal level path — same for every musician regardless of instrument.
pub struct LevelDefinition {
    pub level: MusicianLevel,
    pub label: &'static str,
    pub msa_phase: Option<f64>,
}

pub const LEVEL_PATH: &[LevelDefinition] = &[
    LevelDefinition { level: MusicianLevel::Candidate, label: "Candidato", msa_phase: None },
    LevelDefinition { level: MusicianLevel::Practice, label: "Ensaio", msa_phase: None },
    LevelDefinition { level: MusicianLevel::YouthService, label: "Reunião de Jovens e Menores", msa_phase: Some(12.0) },
    LevelDefinition { level: MusicianLevel::OfficialService, label: "Culto Oficial", msa_phase: Some(16.0) },
    LevelDefinition { level: MusicianLevel::Officialized, label: "Oficialização", msa_phase: Some(16.0) },
];

/// Instrument-specific method thresholds overlaid onto the universal path.
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

pub fn violin_schmoll_profile() -> MethodProfile {
    MethodProfile {
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

#[derive(Debug, Clone, PartialEq)]
pub struct CheckpointStatus {
    pub level: MusicianLevel,
    pub label: &'static str,
    pub achieved: bool,
    pub ready_to_advance: bool,
    /// Whether this checkpoint's MSA phase requirement is met by lesson data alone.
    pub msa_requirement_met: bool,
    /// Whether this checkpoint's method page/lesson requirements are met by lesson data alone.
    pub method_requirement_met: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CategoryProgress {
    pub current: f64,
    pub max: f64,
    pub percent: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressAssessment {
    pub checkpoints: Vec<CheckpointStatus>,
    pub msa_phase_progress: CategoryProgress,
    pub msa_lesson_percent: f64,
    pub method_page_percent: f64,
    pub method_lesson_percent: f64,
    pub overall_percent: f64,
}

pub fn assess(
    assigned_level: &MusicianLevel,
    approved: &[Lesson],
    method: &[Lesson],
    profile: &MethodProfile,
) -> ProgressAssessment {
    let highest_msa_phase = max_field(approved.iter().map(|l| &l.phase));
    let highest_msa_lesson = max_field(approved.iter().map(|l| &l.lesson));
    let highest_method_page = max_field(method.iter().map(|l| &l.page));
    let highest_method_lesson = max_field(method.iter().map(|l| &l.lesson));

    let checkpoints: Vec<CheckpointStatus> = LEVEL_PATH
        .iter()
        .map(|def| {
            let assigned_rank = assigned_level.rank();
            let checkpoint_rank = def.level.rank();
            let achieved = assigned_rank >= checkpoint_rank;

            let msa_met = def.msa_phase.is_none_or(|min| highest_msa_phase >= min);
            let method_met = method_req_met(&def.level, profile, highest_method_page, highest_method_lesson);
            let requirement_met = msa_met && method_met;
            // Ensaio has no formal requirement; readiness is the conductor's call.
            let has_measurable_requirement = def.msa_phase.is_some();

            CheckpointStatus {
                level: def.level.clone(),
                label: def.label,
                achieved,
                ready_to_advance: !achieved && has_measurable_requirement && requirement_met,
                msa_requirement_met: msa_met,
                method_requirement_met: method_met,
            }
        })
        .collect();

    let total_phases = 16.0;
    let total_msa_lessons = 113.0;
    let total_pages = profile.total_pages as f64;
    let total_lessons = profile.total_lessons as f64;

    ProgressAssessment {
        checkpoints,
        msa_phase_progress: CategoryProgress {
            current: highest_msa_phase,
            max: total_phases,
            percent: pct(highest_msa_phase, total_phases),
        },
        msa_lesson_percent: pct(highest_msa_lesson, total_msa_lessons),
        method_page_percent: pct(highest_method_page, total_pages),
        method_lesson_percent: pct(highest_method_lesson, total_lessons),
        overall_percent: pct(highest_method_lesson, total_lessons),
    }
}

fn method_req_met(
    level: &MusicianLevel,
    profile: &MethodProfile,
    page: f64,
    lesson: f64,
) -> bool {
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
            page >= profile.total_pages as f64
                && lesson >= profile.total_lessons as f64
        }
        _ => true,
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
    if max > 0.0 { (current / max * 100.0).min(100.0) } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msa_lesson(phase_from: &str, phase_to: &str) -> Lesson {
        Lesson {
            phase: Some(Range { from: phase_from.to_owned(), to: phase_to.to_owned() }),
            ..Default::default()
        }
    }

    fn method_lesson(page: &str, lesson: &str) -> Lesson {
        Lesson {
            page: Some(Range { from: page.to_owned(), to: page.to_owned() }),
            lesson: Some(Range { from: lesson.to_owned(), to: lesson.to_owned() }),
            ..Default::default()
        }
    }

    #[test]
    fn empty_lessons_yield_all_pending() {
        let report = assess(&MusicianLevel::Candidate, &[], &[], &violin_schmoll_profile());

        assert!(!report.checkpoints.is_empty());
        assert!(report.checkpoints[0].achieved); // Candidate auto-achieved
        assert!(!report.meets_any_above());
        assert_eq!(report.overall_percent, 0.0);
    }

    #[test]
    fn supreme_rule_assigned_level_auto_achieves_below_and_at() {
        // Student assigned Youth Service — all levels ≤ YS are achieved regardless of lessons.
        let approved = vec![msa_lesson("3", "3")]; // way below Phase 12
        let method = vec![method_lesson("10", "20")]; // way below page 46

        let report = assess(&MusicianLevel::YouthService, &approved, &method, &violin_schmoll_profile());

        assert!(report.checkpoints[0].achieved); // Candidate
        assert!(report.checkpoints[1].achieved); // Ensaio
        assert!(report.checkpoints[2].achieved); // RJM — supreme rule
        assert!(!report.checkpoints[3].achieved); // Culto Oficial — above assignment
        assert!(!report.checkpoints[4].achieved);
    }

    #[test]
    fn requirements_met_but_not_assigned_shows_ready_to_advance() {
        // Lessons meet RJM thresholds but student is still Candidate.
        let approved = vec![msa_lesson("12", "12")];
        let method = vec![method_lesson("46", "113")];

        let report = assess(&MusicianLevel::Candidate, &approved, &method, &violin_schmoll_profile());

        assert!(report.checkpoints[0].achieved); // Candidate
        assert!(!report.checkpoints[1].achieved); // Ensaio not assigned
        assert!(!report.checkpoints[1].ready_to_advance); // no formal requirement
        assert!(!report.checkpoints[2].achieved); // RJM not assigned yet
        assert!(report.checkpoints[2].ready_to_advance); // requirements met
    }

    #[test]
    fn meets_culto_oficial_requires_phase_16() {
        // Phase 15 is NOT enough for Culto Oficial.
        let approved = vec![msa_lesson("15", "16")]; // reaches 16
        let method = vec![method_lesson("67", "162")];

        let report = assess(&MusicianLevel::YouthService, &approved, &method, &violin_schmoll_profile());

        assert!(report.checkpoints[3].ready_to_advance); // meets Culto Oficial reqs
    }

    #[test]
    fn officialization_when_method_completed() {
        let approved = vec![msa_lesson("16", "16")];
        let method = vec![method_lesson("80", "214")];
        let student_level = MusicianLevel::OfficialService;

        let report = assess(&student_level, &approved, &method, &violin_schmoll_profile());

        assert!(report.checkpoints[4].ready_to_advance); // ready for Oficialização
        assert!((report.overall_percent - 100.0).abs() < f64::EPSILON);
    }

    #[test]
    fn absent_fields_contribute_zero() {
        let approved = vec![Lesson::default()];
        let method = vec![Lesson::default()];

        let report = assess(&MusicianLevel::Candidate, &approved, &method, &violin_schmoll_profile());

        assert_eq!(report.msa_phase_progress.current, 0.0);
        assert_eq!(report.method_page_percent, 0.0);
    }

    #[test]
    fn unparseable_values_contribute_zero() {
        let approved = vec![msa_lesson("abc", "def")];
        let report = assess(&MusicianLevel::Candidate, &approved, &[], &violin_schmoll_profile());

        assert_eq!(report.msa_phase_progress.current, 0.0);
    }

    #[test]
    fn scan_all_takes_max_not_last() {
        let approved = vec![
            msa_lesson("3", "3"),
            msa_lesson("14", "14"),
            msa_lesson("7", "7"),
        ];
        let report = assess(&MusicianLevel::Candidate, &approved, &[], &violin_schmoll_profile());

        assert_eq!(report.msa_phase_progress.current, 14.0);
    }

    #[test]
    fn range_dash_takes_upper_bound() {
        let approved = vec![msa_lesson("11", "13")];
        let report = assess(&MusicianLevel::Candidate, &approved, &[], &violin_schmoll_profile());

        assert_eq!(report.msa_phase_progress.current, 13.0);
    }

    #[test]
    fn msa_percent_is_phase_based() {
        let approved = vec![msa_lesson("8", "8")];
        let report = assess(&MusicianLevel::Candidate, &approved, &[], &violin_schmoll_profile());

        assert!((report.msa_phase_progress.percent - 50.0).abs() < 0.1); // 8/16
    }

    #[test]
    fn overall_percent_is_method_lesson_based() {
        let method = vec![method_lesson("40", "107")];
        let report = assess(&MusicianLevel::Candidate, &[], &method, &violin_schmoll_profile());

        assert!((report.overall_percent - 50.0).abs() < 0.1); // 107/214
    }

    #[test]
    fn violin_schmoll_profile_is_correct() {
        let p = violin_schmoll_profile();
        assert_eq!(p.total_pages, 80);
        assert_eq!(p.total_lessons, 214);
        assert_eq!(p.youth_service_page, 46);
        assert_eq!(p.youth_service_lesson, 113);
        assert_eq!(p.culto_oficial_page, 67);
        assert_eq!(p.culto_oficial_lesson, 162);
    }

    impl ProgressAssessment {
        fn meets_any_above(&self) -> bool {
            self.checkpoints.iter().any(|c| c.ready_to_advance)
        }
    }
}
