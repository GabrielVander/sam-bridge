use crate::student_roster::domain::entities::{Region, StudentPosition};

#[derive(Debug, Clone, PartialEq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: StudentPosition,
    pub region: Region,
}
