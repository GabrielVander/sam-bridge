use student_management::api::domain::{Clef, Lesson, MusicianLevel, OrganistLevel, ProgressAssessment, Region, SecretaryType, Student, StudentPosition};

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
        _ => "Candidate".to_owned(),
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

fn position_label(position: &StudentPosition) -> String {
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
        OrganistLevel::HafHour => "Meia hora".to_owned(),
        OrganistLevel::YouthServicePractice => "RJM / Ensaio".to_owned(),
        OrganistLevel::YouthServiceHafHour => "RJM / Meia hora".to_owned(),
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

fn clef_label(clef: &Clef) -> String {
    match clef {
        Clef::G => "Sol".to_owned(),
        Clef::C => "Dó".to_owned(),
        Clef::F => "Fá".to_owned(),
    }
}

fn range_label(range: Option<&student_management::api::domain::Range>) -> String {
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

#[derive(Debug, PartialEq, Clone)]
pub struct CheckpointVm {
    pub label: String,
    pub achieved: bool,
    pub ready_to_advance: bool,
    pub msa_requirement_met: bool,
    pub method_requirement_met: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ProgressViewModel {
    pub msa_percent: f64,
    pub method_page_percent: f64,
    pub method_lesson_percent: f64,
    pub overall_percent: f64,
    pub meets_youth_service: bool,
    pub meets_official_service: bool,
    pub meets_officialization: bool,
    pub checkpoints: Vec<CheckpointVm>,
}

impl From<&ProgressAssessment> for ProgressViewModel {
    fn from(a: &ProgressAssessment) -> Self {
        Self {
            msa_percent: a.msa_phase_progress.percent,
            method_page_percent: a.method_page_percent,
            method_lesson_percent: a.method_lesson_percent,
            overall_percent: a.overall_percent,
            meets_youth_service: a.checkpoints.get(2).is_some_and(|c| c.ready_to_advance || c.achieved),
            meets_official_service: a.checkpoints.get(3).is_some_and(|c| c.ready_to_advance),
            meets_officialization: a.checkpoints.get(4).is_some_and(|c| c.ready_to_advance),
            checkpoints: a.checkpoints.iter().map(|c| CheckpointVm {
                label: c.label.to_owned(),
                achieved: c.achieved,
                ready_to_advance: c.ready_to_advance,
                msa_requirement_met: c.msa_requirement_met,
                method_requirement_met: c.method_requirement_met,
            }).collect(),
        }
    }
}


#[cfg(test)]
mod tests {
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

    // ... existing view model tests ...
    #[test]
    fn given_musician_levels_should_render_human_labels() {
        for (position, expected) in [
            (StudentPosition::Musician { level: MusicianLevel::Candidate }, "Músico · Candidato(a)"),
            (StudentPosition::Musician { level: MusicianLevel::Practice }, "Músico · Ensaio"),
            (StudentPosition::Musician { level: MusicianLevel::YouthService }, "Músico · RJM"),
            (StudentPosition::Musician { level: MusicianLevel::OfficialService }, "Músico · Culto Oficial"),
            (StudentPosition::Musician { level: MusicianLevel::Unknown("X".to_owned()) }, "Músico · X"),
        ] {
            assert_eq!(student_with_position(position).position, expected);
        }
    }

    #[test]
    fn given_progress_report_should_map_to_view_model() {
        use student_management::api::domain::CategoryProgress;

        let report = student_management::api::domain::ProgressAssessment {
            checkpoints: vec![],
            msa_phase_progress: CategoryProgress { current: 12.0, max: 16.0, percent: 75.0 },
            msa_lesson_percent: 0.0,
            method_page_percent: 57.5,
            method_lesson_percent: 52.8,
            overall_percent: 52.8,
        };
        let vm = ProgressViewModel::from(&report);
        assert!((vm.msa_percent - 75.0).abs() < 0.1);
        assert!((vm.method_page_percent - 57.5).abs() < 0.1);
    }

    #[test]
    fn given_region_should_render_label() {
        assert_eq!(region_label(&Region::AraraquaraSaoCarlos), "Araraquara – São Carlos");
        assert_eq!(region_label(&Region::Other("OUTRA".to_owned())), "OUTRA");
    }
}
