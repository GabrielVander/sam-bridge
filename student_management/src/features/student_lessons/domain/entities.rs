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
pub struct Range<T = String> {
    pub from: T,
    pub to: T,
}

impl<T> Range<T> {
    pub fn new(from: T, to: T) -> Self {
        Self { from, to }
    }
    pub fn single(value: T) -> Self
    where
        T: Clone,
    {
        Self { from: value.clone(), to: value }
    }
    pub fn try_new(from: T, to: T) -> Result<Self, String>
    where
        T: PartialOrd + std::fmt::Display,
    {
        if from <= to {
            Ok(Self { from, to })
        } else {
            Err(format!("inverted range {from} > {to}"))
        }
    }
}

impl<T> Range<T>
where
    T: std::str::FromStr + PartialOrd + std::fmt::Display,
    T::Err: std::fmt::Display,
{
    pub fn try_from_str(from: &str, to: &str) -> Result<Self, String>
    where
        T: Clone,
    {
        let f: T = from.parse().map_err(|e| format!("{e}"))?;
        let t: T = to.parse().map_err(|e| format!("{e}"))?;
        Self::try_new(f, t)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Clef {
    G,
    C,
    F,
}
