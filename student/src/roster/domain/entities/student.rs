use crate::roster::domain::entities::{region::Region, student_position::StudentPosition};
use crate::shared::domain::entities::StudentId;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Student {
    pub id: StudentId,
    pub name: String,
    pub position: StudentPosition,
    pub location: String,
    pub region: Region,
}
