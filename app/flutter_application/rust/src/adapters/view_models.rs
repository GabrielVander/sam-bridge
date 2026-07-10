use student_management::domain::entities::Student;

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
