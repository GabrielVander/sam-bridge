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
    fn range_try_new_inverted() {
        assert!(Range::try_new(1, 2).is_ok());
        assert_eq!(Range::try_new(5, 3).unwrap_err(), "inverted range 5 > 3");
        assert_eq!(Range::try_new("b".to_owned(), "a".to_owned()).unwrap_err(), "inverted range b > a");
    }

    #[test]
    fn range_try_from_str() {
        let r: Range<i32> = Range::try_from_str("3", "5").unwrap();
        assert_eq!(r.from, 3);
        assert_eq!(r.to, 5);
        let r2: Range<i32> = Range::try_from_str("7", "7").unwrap();
        assert_eq!(r2.from, 7);
        assert!(Range::<i32>::try_from_str("a", "b").is_err());
        assert!(Range::<i32>::try_from_str("5", "3").is_err());
    }

    #[test]
    fn lesson_default_and_clone() {
        let l = Lesson::default();
        assert!(l.id.is_none());
        let sl = StudentLessons::default();
        assert!(sl.approved.is_empty());
        assert_eq!(l.clone(), Lesson::default());
        assert_eq!(Clef::G.clone(), Clef::G);
    }
}
