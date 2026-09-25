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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_new_and_single() {
        let r = Range::new("a".to_owned(), "b".to_owned());
        assert_eq!(r.from, "a");
        assert_eq!(r.to, "b");
        let s = Range::single("x".to_owned());
        assert_eq!(s.from, "x");
        assert_eq!(s.to, "x");
        let n = Range::new(3, 5);
        assert_eq!(n.from, 3);
    }

    #[test]
    fn lesson_default_and_clone() {
        let l = Lesson::default();
        assert!(l.id.is_none());
        let sl = StudentLessons::default();
        assert_eq!(sl.msa, Vec::new());
        assert_eq!(l, Lesson::default());
        assert_eq!(Clef::G.clone(), Clef::G);
    }
}
