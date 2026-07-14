#[derive(Debug)]
pub struct Lesson {
    pub id: String,
    pub date: chrono::NaiveDate,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: String,
}

#[derive(Debug)]
pub struct Range {
    pub from: String,
    pub to: String,
}

#[derive(Debug)]
pub enum Clef {
    G,
    C,
    F,
}
