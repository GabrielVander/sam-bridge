#[derive(Debug, PartialEq, Clone)]
pub struct Lesson {
    pub id: String,
    pub date: chrono::NaiveDate,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: String,
    /// Instrument method name for method lessons (e.g. "MÉTODO CCB - SCHIMOLL - VIOLINO").
    /// `None` for MSA lessons.
    pub method: Option<String>,
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
