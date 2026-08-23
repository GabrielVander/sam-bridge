use student_management::api::domain::{Clef, Lesson, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition};

#[derive(Debug, PartialEq, Clone)]
pub struct StudentListItem {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: String,
}

impl From<&Student> for StudentListItem {
    fn from(value: &Student) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            location: value.location.clone(),
            position: position_label(&value.position),
        }
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
    /// ISO-8601 (`YYYY-MM-DD`) — FRB-friendly primitive for the Dart side.
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

/// Region label kept for completeness of the view surface (e.g. future filters).
pub fn region_label(region: &Region) -> String {
    match region {
        Region::AraraquaraSaoCarlos => "Araraquara – São Carlos".to_owned(),
        Region::AraraquaraItirapina => "Araraquara – Itirapina".to_owned(),
        Region::Other(raw) => raw.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::domain::Range;

    fn student_with_position(position: StudentPosition) -> StudentListItem {
        StudentListItem::from(&Student {
            id: "1".to_owned(),
            name: "ALUNO".to_owned(),
            position,
            location: "BAIRRO".to_owned(),
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
                    level: MusicianLevel::Unknown("X".to_owned()),
                },
                "Músico · X",
            ),
        ] {
            assert_eq!(student_with_position(position).position, expected);
        }
    }

    #[test]
    fn given_organist_levels_should_render_human_labels() {
        for (position, expected) in [
            (
                StudentPosition::Organist {
                    level: OrganistLevel::Candidate,
                },
                "Organista · Candidato(a)",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::Practice,
                },
                "Organista · Ensaio",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::YouthService,
                },
                "Organista · RJM",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::OfficialService,
                },
                "Organista · Culto Oficial",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::HafHour,
                },
                "Organista · Meia hora",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceOfficialized,
                },
                "Organista · RJM / Oficializado(a)",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServicePractice,
                },
                "Organista · RJM / Ensaio",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceHafHour,
                },
                "Organista · RJM / Meia hora",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceOfficialService,
                },
                "Organista · RJM / Culto Oficial",
            ),
            (
                StudentPosition::Organist {
                    level: OrganistLevel::Unknown("Y".to_owned()),
                },
                "Organista · Y",
            ),
        ] {
            assert_eq!(student_with_position(position).position, expected);
        }
    }

    #[test]
    fn given_secretary_and_unknown_should_render_labels() {
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
    }

    fn lesson(fields: impl FnOnce(&mut Lesson)) -> Lesson {
        let mut l = Lesson::default();
        fields(&mut l);
        l
    }

    fn ymd(y: i32, m: u32, d: u32) -> chrono::NaiveDate {
        chrono::NaiveDate::from_ymd_opt(y, m, d).expect("valid date")
    }

    #[test]
    fn given_msa_lesson_should_map_all_display_fields() {
        let domain = lesson(|l| {
            l.id = Some("9".to_owned());
            l.date = Some(ymd(2025, 9, 9));
            l.phase = Some(Range {
                from: "4.5".to_owned(),
                to: "4.5".to_owned(),
            });
            l.page = Some(Range {
                from: "38".to_owned(),
                to: "40".to_owned(),
            });
            l.lesson = Some(Range {
                from: "7".to_owned(),
                to: "8".to_owned(),
            });
            l.clef = Some(Clef::G);
            l.description = Some("obs".to_owned());
            l.instructor = Some("AUTH".to_owned());
        });

        let item = LessonItem::from_domain(LessonKind::Msa, &domain);

        assert_eq!(item.kind, LessonKind::Msa);
        assert_eq!(item.date, "2025-09-09");
        assert_eq!(item.phase, "4.5");
        assert_eq!(item.page, "38 - 40");
        assert_eq!(item.lesson, "7 - 8");
        assert_eq!(item.clef, "Sol");
        assert_eq!(item.description, "obs");
        assert_eq!(item.method, "");
        assert_eq!(item.instructor, "AUTH");
    }

    #[test]
    fn given_method_lesson_should_carry_method_name() {
        let domain = lesson(|l| {
            l.id = Some("8".to_owned());
            l.method = Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_owned());
        });

        let item = LessonItem::from_domain(LessonKind::Method, &domain);

        assert_eq!(item.kind, LessonKind::Method);
        assert_eq!(item.method, "MÉTODO CCB - SCHIMOLL - VIOLINO");
        assert_eq!(item.instructor, "", "Absent instructor renders empty");
        assert_eq!(item.date, "", "Absent date renders empty");
    }

    #[test]
    fn given_all_clefs_should_render_labels() {
        let make = |clef: Option<Clef>| {
            let domain = lesson(|l| l.clef = clef);
            LessonItem::from_domain(LessonKind::Msa, &domain).clef
        };

        assert_eq!(make(Some(Clef::G)), "Sol");
        assert_eq!(make(Some(Clef::C)), "Dó");
        assert_eq!(make(Some(Clef::F)), "Fá");
        assert_eq!(make(None), "");
    }

    #[test]
    fn given_region_should_render_label() {
        assert_eq!(region_label(&Region::AraraquaraSaoCarlos), "Araraquara – São Carlos");
        assert_eq!(region_label(&Region::AraraquaraItirapina), "Araraquara – Itirapina");
        assert_eq!(region_label(&Region::Other("OUTRA".to_owned())), "OUTRA");
    }
}
