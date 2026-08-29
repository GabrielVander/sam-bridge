use anyhow::{Context, bail};
use student_core::domain::entities::{Clef, Range};

pub fn parse_range(raw: &str) -> anyhow::Result<Range> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        bail!("Unable to parse range: '{raw}'");
    }
    let mut parts = trimmed.split('-').map(|p| p.trim());
    let from = parts.next().unwrap_or_default();
    if from.is_empty() {
        bail!("Unable to parse range: '{raw}'");
    }
    Ok(match parts.next() {
        Some(to) if !to.is_empty() => Range {
            from: from.to_owned(),
            to: to.to_owned(),
        },
        _ => Range {
            from: from.to_owned(),
            to: from.to_owned(),
        },
    })
}

pub fn parse_clef(raw: &str) -> anyhow::Result<Clef> {
    match raw.trim().to_uppercase().as_str() {
        "SOL" => Ok(Clef::G),
        "DO" | "DÓ" => Ok(Clef::C),
        "FA" | "FÁ" => Ok(Clef::F),
        _ => bail!("Unable to parse clef value '{raw}'"),
    }
}

pub fn parse_naive_date(raw: &str) -> anyhow::Result<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(raw.trim(), "%d/%m/%Y")
        .map_err(anyhow::Error::from)
        .context("Failed to parse lesson date")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_range_with_dash_should_split() {
        assert_eq!(
            parse_range("3.4 - 4.1").unwrap(),
            Range {
                from: "3.4".to_owned(),
                to: "4.1".to_owned()
            }
        );
    }

    #[test]
    fn given_single_value_should_duplicate() {
        assert_eq!(
            parse_range("7").unwrap(),
            Range {
                from: "7".to_owned(),
                to: "7".to_owned()
            }
        );
    }

    #[test]
    fn given_empty_range_should_fail() {
        assert!(parse_range("").is_err());
        assert!(parse_range("   ").is_err());
        assert!(parse_range("-5").is_err(), "Empty 'from' side must fail");
    }

    #[test]
    fn given_trailing_dash_should_duplicate_single_value() {
        assert_eq!(
            parse_range("7-").unwrap(),
            Range {
                from: "7".to_owned(),
                to: "7".to_owned()
            }
        );
    }

    #[test]
    fn given_clef_variants_should_map() {
        assert_eq!(parse_clef("Sol").unwrap(), Clef::G);
        assert_eq!(parse_clef("DO").unwrap(), Clef::C);
        assert_eq!(parse_clef("FÁ").unwrap(), Clef::F);
        assert!(parse_clef("unknown").is_err());
    }

    #[test]
    fn given_valid_dates_should_parse() {
        assert!(parse_naive_date("04/12/2023").is_ok());
        assert!(parse_naive_date("2023-12-04").is_err());
    }
}
