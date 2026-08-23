#[derive(Debug, PartialEq, Clone, Default)]
pub struct Lesson {
    pub id: Option<String>,
    /// ISO-8601 date when present.
    pub date: Option<chrono::NaiveDate>,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: Option<String>,
    /// Instrument method name for method lessons.
    /// `None` for approved (MSA) lessons.
    pub method: Option<String>,
}

/// Both lesson kinds for one student. SAM exposes them on the same page.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct StudentLessons {
    /// Approved ("MSA") lessons.
    pub approved: Vec<Lesson>,
    /// Instrument-method lessons.
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
