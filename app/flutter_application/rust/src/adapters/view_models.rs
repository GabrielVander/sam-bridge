use student_management::domain::entities::{Lesson, Student};

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

impl From<&Lesson> for SingleLessonViewModel {
    fn from(value: &Lesson) -> Self {
        Self {
            id: value.id.clone(),
            date: value.date.clone(),
            phase: value
                .phase
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("".to_string()),
            page: value
                .page
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("".to_string()),
            lesson: value
                .lesson
                .as_ref()
                .map(|i| format!("{} - {}", i.from, i.to))
                .unwrap_or("".to_string()),
            clef: value
                .clef
                .as_ref()
                .map(|i| format!("{:#?}", i))
                .unwrap_or("".to_string()),
            description: value
                .description
                .as_ref()
                .map(|i| format!("{:#?}", i))
                .unwrap_or("".to_string()),
            instructor: value
                .instructor
                .as_ref()
                .map(|i| format!("{:#?}", i))
                .unwrap_or("".to_string()),
        }
    }
}
