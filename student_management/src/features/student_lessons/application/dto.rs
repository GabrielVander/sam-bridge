#![allow(non_snake_case)]

use crate::features::student_lessons::domain::entities::{Clef, Range};
use flutter_rust_bridge::frb;

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct LessonDto {
    pub id: Option<String>,
    pub date: Option<chrono::NaiveDate>,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: Option<String>,
    pub method: Option<String>,
}

#[frb(non_opaque)]
#[derive(Debug, Clone, PartialEq)]
pub struct StudentLessonsDto {
    pub approved: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}
