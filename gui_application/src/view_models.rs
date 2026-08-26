use student_management::domain::entities::{
    Clef, Lesson, MusicianLevel, OrganistLevel, ProgressAssessment, Range, Region, SecretaryType,
    Student, StudentPosition,
};

#[derive(Debug, PartialEq, Clone)]
pub struct StudentListItem {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: String,
    pub raw_level: String,
}

impl From<&Student> for StudentListItem {
    fn from(value: &Student) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            location: value.location.clone(),
            position: position_label(&value.position),
            raw_level: extract_raw_level(&value.position),
        }
    }
}

fn extract_raw_level(position: &StudentPosition) -> String {
    match position {
        StudentPosition::Musician { level } => level.name(),
        _ => String::new(),
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum LessonKind {
    Msa,
    Method,
}

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StudentLessonsView {
    pub msa: Vec<LessonItem>,
    pub method: Vec<LessonItem>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LessonItem {
    pub kind: LessonKind,
    pub id: String,
    pub date: String,
    pub phase: String,
    pub page: String,
    pub lesson: String,
    pub clef: String,
    pub description: String,
    pub instructor: String,
    pub method: String,
}

impl LessonItem {
    pub fn from_domain(kind: LessonKind, value: &Lesson) -> Self {
        Self {
            kind,
            id: value.id.clone().unwrap_or_default(),
            date: value.date.map(|d| d.to_string()).unwrap_or_default(),
            phase: range_label(value.phase.as_ref()),
            page: range_label(value.page.as_ref()),
            lesson: range_label(value.lesson.as_ref()),
            clef: value.clef.as_ref().map(clef_label).unwrap_or_default(),
            description: value.description.clone().unwrap_or_default(),
            instructor: value.instructor.clone().unwrap_or_default(),
            method: value.method.clone().unwrap_or_default(),
        }
    }
}

pub fn position_label(position: &StudentPosition) -> String {
    match position {
        StudentPosition::Musician { level } => {
            format!("Músico · {}", musician_level_label(level))
        }
        StudentPosition::Organist { level } => {
            format!("Organista · {}", organist_level_label(level))
        }
        StudentPosition::Secretary { r#type } => secretary_label(r#type).to_owned(),
        StudentPosition::Unknown(raw) => raw.clone(),
    }
}

fn musician_level_label(level: &MusicianLevel) -> String {
    match level {
        MusicianLevel::Candidate => "Candidato(a)".to_owned(),
        MusicianLevel::Practice => "Ensaio".to_owned(),
        MusicianLevel::YouthService => "RJM".to_owned(),
        MusicianLevel::OfficialService => "Culto Oficial".to_owned(),
        MusicianLevel::Officialized => "Oficializado".to_owned(),
        MusicianLevel::Unknown(raw) => raw.clone(),
    }
}

fn organist_level_label(level: &OrganistLevel) -> String {
    match level {
        OrganistLevel::Candidate => "Candidato(a)".to_owned(),
        OrganistLevel::Practice => "Ensaio".to_owned(),
        OrganistLevel::YouthService => "RJM".to_owned(),
        OrganistLevel::OfficialService => "Culto Oficial".to_owned(),
        OrganistLevel::HalfHour => "Meia hora".to_owned(),
        OrganistLevel::YouthServicePractice => "RJM / Ensaio".to_owned(),
        OrganistLevel::YouthServiceHalfHour => "RJM / Meia hora".to_owned(),
        OrganistLevel::YouthServiceOfficialService => "RJM / Culto Oficial".to_owned(),
        OrganistLevel::YouthServiceOfficialized => "RJM / Oficializado(a)".to_owned(),
        OrganistLevel::Unknown(raw) => raw.clone(),
    }
}

fn secretary_label(secretary: &SecretaryType) -> &'static str {
    match secretary {
        SecretaryType::Gem => "Secretário do GEM",
        SecretaryType::Music => "Secretário da Música",
    }
}

pub fn clef_label(clef: &Clef) -> String {
    match clef {
        Clef::G => "Sol".to_owned(),
        Clef::C => "Dó".to_owned(),
        Clef::F => "Fá".to_owned(),
    }
}

pub fn range_label(range: Option<&Range>) -> String {
    range
        .map(|r| {
            if r.from == r.to {
                r.from.clone()
            } else {
                format!("{} - {}", r.from, r.to)
            }
        })
        .unwrap_or_default()
}

pub fn region_label(region: &Region) -> String {
    match region {
        Region::AraraquaraSaoCarlos => "Araraquara – São Carlos".to_owned(),
        Region::AraraquaraItirapina => "Araraquara – Itirapina".to_owned(),
        Region::Other(raw) => raw.clone(),
    }
}

fn checkpoint_label(level: &MusicianLevel) -> &'static str {
    match level {
        MusicianLevel::Candidate => "Candidato",
        MusicianLevel::Practice => "Ensaio",
        MusicianLevel::YouthService => "Reunião de Jovens e Menores",
        MusicianLevel::OfficialService => "Culto Oficial",
        MusicianLevel::Officialized => "Oficialização",
        MusicianLevel::Unknown(_) => "Desconhecido",
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CheckpointVm {
    pub label: String,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub msa_requirement_met: bool,
    pub method_requirement_met: bool,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(non_snake_case)]
pub struct ProgressViewModel {
    pub msaRelativePercent: f64,
    pub methodRelativePercent: f64,
    pub combinedPercent: f64,
    pub overallCheckpointPercent: f64,
    pub nextLevelLabel: String,
    pub meets_youth_service: bool,
    pub meets_official_service: bool,
    pub meets_officialization: bool,
    pub checkpoints: Vec<CheckpointVm>,
}

impl From<&ProgressAssessment> for ProgressViewModel {
    fn from(a: &ProgressAssessment) -> Self {
        Self {
            msaRelativePercent: a.msaRelativePercent,
            methodRelativePercent: a.methodRelativePercent,
            combinedPercent: a.combinedPercent,
            overallCheckpointPercent: a.overallCheckpointPercent,
            nextLevelLabel: a
                .nextLevel
                .as_ref()
                .map(|l| checkpoint_label(l).to_owned())
                .unwrap_or_default(),
            meets_youth_service: a
                .checkpoints
                .iter()
                .find(|c| c.level == MusicianLevel::YouthService)
                .is_some_and(|c| c.ready_to_advance || c.achieved),
            meets_official_service: a
                .checkpoints
                .iter()
                .find(|c| c.level == MusicianLevel::OfficialService)
                .is_some_and(|c| c.ready_to_advance),
            meets_officialization: a
                .checkpoints
                .iter()
                .find(|c| c.level == MusicianLevel::Officialized)
                .is_some_and(|c| c.ready_to_advance),
            checkpoints: a
                .checkpoints
                .iter()
                .map(|c| CheckpointVm {
                    label: checkpoint_label(&c.level).to_owned(),
                    achieved: c.achieved,
                    ready_to_advance: c.ready_to_advance,
                    msa_requirement_met: c.msa_requirement_met,
                    method_requirement_met: c.method_requirement_met,
                })
                .collect(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnknownLevelVm {
    pub raw: String,
    pub message: String,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgressResult {
    pub is_unknown: bool,
    pub progress: ProgressViewModel,
    pub unknown: UnknownLevelVm,
}

impl ProgressResult {
    pub fn available(vm: ProgressViewModel) -> Self {
        Self {
            is_unknown: false,
            progress: vm,
            unknown: UnknownLevelVm {
                raw: String::new(),
                message: String::new(),
            },
        }
    }
    pub fn unknown(raw: String, message: String) -> Self {
        Self {
            is_unknown: true,
            progress: ProgressViewModel {
                msaRelativePercent: 0.0,
                methodRelativePercent: 0.0,
                combinedPercent: 0.0,
                overallCheckpointPercent: 0.0,
                nextLevelLabel: String::new(),
                meets_youth_service: false,
                meets_official_service: false,
                meets_officialization: false,
                checkpoints: vec![],
            },
            unknown: UnknownLevelVm { raw, message },
        }
    }
    pub fn is_unknown(&self) -> bool {
        self.is_unknown
    }
}

impl Default for ProgressViewModel {
    fn default() -> Self {
        Self {
            msaRelativePercent: 0.0,
            methodRelativePercent: 0.0,
            combinedPercent: 0.0,
            overallCheckpointPercent: 0.0,
            nextLevelLabel: String::new(),
            meets_youth_service: false,
            meets_official_service: false,
            meets_officialization: false,
            checkpoints: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use student_management::domain::entities::CheckpointStatus;

    use super::*;

    fn student_with_position(position: StudentPosition) -> StudentListItem {
        StudentListItem::from(&Student {
            id: "1".to_owned(),
            name: "ALUNO".to_owned(),
            position,
            location: String::new(),
            region: Region::AraraquaraSaoCarlos,
        })
    }

    #[test]
    fn given_musician_levels_should_render_human_labels() {
        for (position, expected) in [
            (
                StudentPosition::Musician {
                    level: MusicianLevel::Candidate,
                },
                "Músico · Candidato(a)",
            ),
            (
                StudentPosition::Musician {
                    level: MusicianLevel::Practice,
                },
                "Músico · Ensaio",
            ),
            (
                StudentPosition::Musician {
                    level: MusicianLevel::YouthService,
                },
                "Músico · RJM",
            ),
            (
                StudentPosition::Musician {
                    level: MusicianLevel::OfficialService,
                },
                "Músico · Culto Oficial",
            ),
            (
                StudentPosition::Musician {
                    level: MusicianLevel::Officialized,
                },
                "Músico · Oficializado",
            ),
            (
                StudentPosition::Musician {
                    level: MusicianLevel::Unknown("X".to_owned()),
                },
                "Músico · X",
            ),
        ] {
            assert_eq!(student_with_position(position).position, expected);
        }
    }

    #[test]
    fn given_progress_report_should_map_to_view_model() {
        let report = ProgressAssessment {
            checkpoints: vec![],
            msaRelativePercent: 75.0,
            methodRelativePercent: 57.5,
            combinedPercent: 66.0,
            overallCheckpointPercent: 33.0,
            nextLevel: None,
        };
        let vm = ProgressViewModel::from(&report);
        assert!((vm.msaRelativePercent - 75.0).abs() < 0.1);
        assert!((vm.methodRelativePercent - 57.5).abs() < 0.1);
        assert!((vm.combinedPercent - 66.0).abs() < 0.1);
        assert!((vm.overallCheckpointPercent - 33.0).abs() < 0.1);
    }

    #[test]
    fn given_region_should_render_label() {
        assert_eq!(
            region_label(&Region::AraraquaraSaoCarlos),
            "Araraquara – São Carlos"
        );
        assert_eq!(
            region_label(&Region::AraraquaraItirapina),
            "Araraquara – Itirapina"
        );
        assert_eq!(region_label(&Region::Other("OUTRA".to_owned())), "OUTRA");
    }

    #[test]
    fn given_all_position_labels() {
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::Candidate
            })
            .position,
            "Organista · Candidato(a)"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::Practice
            })
            .position,
            "Organista · Ensaio"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::YouthService
            })
            .position,
            "Organista · RJM"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::OfficialService
            })
            .position,
            "Organista · Culto Oficial"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::HalfHour
            })
            .position,
            "Organista · Meia hora"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::YouthServicePractice
            })
            .position,
            "Organista · RJM / Ensaio"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::YouthServiceHalfHour
            })
            .position,
            "Organista · RJM / Meia hora"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::YouthServiceOfficialService
            })
            .position,
            "Organista · RJM / Culto Oficial"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::YouthServiceOfficialized
            })
            .position,
            "Organista · RJM / Oficializado(a)"
        );
        assert_eq!(
            student_with_position(StudentPosition::Secretary {
                r#type: SecretaryType::Gem
            })
            .position,
            "Secretário do GEM"
        );
        assert_eq!(
            student_with_position(StudentPosition::Secretary {
                r#type: SecretaryType::Music
            })
            .position,
            "Secretário da Música"
        );
        assert_eq!(
            student_with_position(StudentPosition::Unknown("REGENTE".to_owned())).position,
            "REGENTE"
        );
        assert_eq!(
            student_with_position(StudentPosition::Organist {
                level: OrganistLevel::Unknown("X".to_owned())
            })
            .position,
            "Organista · X"
        );
    }

    #[test]
    fn given_extract_raw_level_only_musician() {
        assert_eq!(
            extract_raw_level(&StudentPosition::Musician {
                level: MusicianLevel::Candidate
            }),
            "Candidate"
        );
        assert_eq!(
            extract_raw_level(&StudentPosition::Organist {
                level: OrganistLevel::Candidate
            }),
            ""
        );
        assert_eq!(
            extract_raw_level(&StudentPosition::Secretary {
                r#type: SecretaryType::Gem
            }),
            ""
        );
        assert_eq!(
            extract_raw_level(&StudentPosition::Unknown("x".to_owned())),
            ""
        );
    }

    #[test]
    fn given_clef_and_range_labels() {
        assert_eq!(clef_label(&Clef::G), "Sol");
        assert_eq!(clef_label(&Clef::C), "Dó");
        assert_eq!(clef_label(&Clef::F), "Fá");
        assert_eq!(range_label(None), "");
        assert_eq!(
            range_label(Some(&Range {
                from: "5".to_owned(),
                to: "5".to_owned()
            })),
            "5"
        );
        assert_eq!(
            range_label(Some(&Range {
                from: "1".to_owned(),
                to: "3".to_owned()
            })),
            "1 - 3"
        );
        assert_eq!(secretary_label(&SecretaryType::Gem), "Secretário do GEM");
        assert_eq!(
            secretary_label(&SecretaryType::Music),
            "Secretário da Música"
        );
    }

    #[test]
    fn given_lesson_item_from_domain() {
        let lesson = Lesson {
            id: Some("1".to_owned()),
            date: Some(chrono::NaiveDate::from_ymd_opt(2025, 1, 15).unwrap()),
            phase: Some(Range {
                from: "1".to_owned(),
                to: "2".to_owned(),
            }),
            page: Some(Range {
                from: "5".to_owned(),
                to: "5".to_owned(),
            }),
            lesson: None,
            clef: Some(Clef::G),
            description: Some("desc".to_owned()),
            instructor: None,
            method: Some("met".to_owned()),
        };
        let item = LessonItem::from_domain(LessonKind::Msa, &lesson);
        assert_eq!(item.id, "1");
        assert_eq!(item.date, "2025-01-15");
        assert_eq!(item.phase, "1 - 2");
        assert_eq!(item.page, "5");
        assert_eq!(item.lesson, "");
        assert_eq!(item.clef, "Sol");
        assert_eq!(item.description, "desc");
        assert_eq!(item.method, "met");
        let empty = LessonItem::from_domain(LessonKind::Method, &Lesson::default());
        assert_eq!(empty.id, "");
        assert_eq!(empty.clef, "");
    }

    #[test]
    fn given_progress_view_model_with_checkpoints() {
        let checkpoints = vec![
            CheckpointStatus {
                level: MusicianLevel::Candidate,
                achieved: true,
                ready_to_advance: false,
                msa_requirement_met: true,
                method_requirement_met: true,
            },
            CheckpointStatus {
                level: MusicianLevel::Practice,
                achieved: true,
                ready_to_advance: false,
                msa_requirement_met: true,
                method_requirement_met: true,
            },
            CheckpointStatus {
                level: MusicianLevel::YouthService,
                achieved: false,
                ready_to_advance: true,
                msa_requirement_met: true,
                method_requirement_met: true,
            },
            CheckpointStatus {
                level: MusicianLevel::OfficialService,
                achieved: false,
                ready_to_advance: false,
                msa_requirement_met: false,
                method_requirement_met: false,
            },
            CheckpointStatus {
                level: MusicianLevel::Officialized,
                achieved: false,
                ready_to_advance: false,
                msa_requirement_met: false,
                method_requirement_met: false,
            },
        ];
        let report = ProgressAssessment {
            checkpoints,
            msaRelativePercent: 75.0,
            methodRelativePercent: 60.0,
            combinedPercent: 67.5,
            overallCheckpointPercent: 53.5,
            nextLevel: Some(MusicianLevel::YouthService),
        };
        let vm = ProgressViewModel::from(&report);
        assert!(vm.meets_youth_service);
        assert!(!vm.meets_official_service);
        assert!(!vm.meets_officialization);
        assert_eq!(vm.checkpoints.len(), 5);
        assert!((vm.msaRelativePercent - 75.0).abs() < 0.1);
        assert!((vm.methodRelativePercent - 60.0).abs() < 0.1);
        assert!((vm.combinedPercent - 67.5).abs() < 0.1);
        assert_eq!(vm.nextLevelLabel, "Reunião de Jovens e Menores");
        let unknown = ProgressResult::unknown("X".to_owned(), "nível não reconhecido".to_owned());
        assert!(unknown.is_unknown());
        let available = ProgressResult::available(vm.clone());
        assert!(!available.is_unknown());
        assert_eq!(ProgressViewModel::default().checkpoints.len(), 0);
    }
}
