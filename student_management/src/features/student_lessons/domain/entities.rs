#[derive(Debug, PartialEq, Clone, Default)]
pub struct Lesson {
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

#[derive(Debug, PartialEq, Clone, Default)]
pub struct StudentLessons {
    pub approved: Vec<Lesson>,
    pub method: Vec<Lesson>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Range {
    pub from: String,
    pub to: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Clef {
    G,
    C,
    F,
}
