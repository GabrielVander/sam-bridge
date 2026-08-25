#![allow(non_snake_case)]

use crate::features::student_roster::domain::entities::{Region, StudentPosition};
use flutter_rust_bridge::frb;

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: StudentPosition,
    pub region: Region,
}
