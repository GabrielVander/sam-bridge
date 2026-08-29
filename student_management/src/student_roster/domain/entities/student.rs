use crate::student_roster::domain::entities::{region::Region, student_position::StudentPosition};

#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub position: StudentPosition,
    pub location: String,
    pub region: Region,
}
