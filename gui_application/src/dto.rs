#![allow(non_snake_case)]

use flutter_rust_bridge::frb;

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub enum DtoPosition {
    Musician { levelName: String },
    Organist { levelName: String },
    Secretary { typeName: String },
    Unknown { raw: String },
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub enum DtoRegion {
    AraraquaraSaoCarlos,
    AraraquaraItirapina,
    Other { raw: String },
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct StudentSummaryDto {
    pub id: String,
    pub name: String,
    pub location: String,
    pub position: DtoPosition,
    pub region: DtoRegion,
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct LessonDto {
    pub id: String,
    pub date: String,
    pub phase: Option<RangeDto>,
    pub page: Option<RangeDto>,
    pub lesson: Option<RangeDto>,
    pub clef: Option<String>,
    pub description: String,
    pub instructor: String,
    pub method: String,
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct RangeDto {
    pub from: String,
    pub to: String,
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct StudentLessonsDto {
    pub approved: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}
