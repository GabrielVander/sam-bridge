#[derive(Debug, PartialEq, Eq, Clone, Default)]
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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct StudentLessons {
    pub msa: Vec<Lesson>,
    pub method: Vec<Lesson>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Range<T = String> {
    pub from: T,
    pub to: T,
}

impl<T> Range<T> {
    pub const fn new(from: T, to: T) -> Self {
        Self { from, to }
    }
    pub fn single(value: T) -> Self
    where
        T: Clone,
    {
        Self {
            from: value.clone(),
            to: value,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Clef {
    G,
    C,
    F,
}
