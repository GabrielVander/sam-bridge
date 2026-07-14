use student_management::api::domain::{Clef, Lesson, Student};

pub struct SingleStudentViewModel {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: String,
}

impl From<&Student> for SingleStudentViewModel {
    fn from(value: &Student) -> Self {
        Self {
            id: value.id.clone(),
            name: value.name.clone(),
            location: value.location.clone(),
            position: format!("{:?}", value.position),
        }
    }
}

pub struct SingleLessonViewModel {
    pub id: String,
    pub date: String,
    pub phase: String,
    pub page: String,
    pub lesson: String,
    pub clef: String,
    pub description: String,
    pub instructor: String,
}

fn displat_clef(clef: &Clef) -> String {
    match clef {
        Clef::G => "Sol".to_string(),
        Clef::C => "Dó".to_string(),
        Clef::F => "Fá".to_string(),
    }
}

impl From<&Lesson> for SingleLessonViewModel {
    fn from(value: &Lesson) -> Self {
        Self {
            id: value.id.clone(),
            date: value.date.to_string(),
            phase: value
                .phase
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("N/A".to_string()),
            page: value
                .page
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("N/A".to_string()),
            lesson: value
                .lesson
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("N/A".to_string()),
            clef: value
                .clef
                .as_ref()
                .map(displat_clef)
                .unwrap_or("N/A".to_string()),
            description: value
                .description
                .as_ref()
                .map(|i| i.to_string())
                .unwrap_or("".to_string()),
            instructor: value.instructor.clone(),
        }
    }
}
