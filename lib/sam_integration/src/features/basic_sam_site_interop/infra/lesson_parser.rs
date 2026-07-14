use anyhow::Context;
use scraper::{ElementRef, Html, Selector};
use student_management::api::domain::{Clef, Lesson, Range};

pub struct LessonParser;

impl LessonParser {
    pub fn parse_html(html: &str) -> anyhow::Result<Vec<Lesson>> {
        let document = Html::parse_document(html);
        let row_selector = Selector::parse(r#"tr[id^="msa_"]"#)
            .map_err(|_| anyhow::anyhow!("Failed to compile row selector"))?;
        let cell_selector = Selector::parse("td")
            .map_err(|_| anyhow::anyhow!("Failed to compile cell selector"))?;

        let mut lessons = Vec::new();

        for row in document.select(&row_selector) {
            lessons.push(Self::parse_row(row, &cell_selector)?);
        }

        Ok(lessons)
    }

    fn parse_row(row: ElementRef, cell_selector: &Selector) -> anyhow::Result<Lesson> {
        let id = row.value().attr("id").unwrap_or("unknown_id").to_string();
        let mut data = row.select(cell_selector);

        let date_str = Self::extract_text(&mut data);
        let date = chrono::NaiveDate::parse_from_str(&date_str, "%d/%m/%Y")
            .context("Failed to parse lesson date")?;

        let phase = Self::parse_fragment(&Self::extract_text(&mut data));
        let page = Self::parse_fragment(&Self::extract_text(&mut data));
        let lesson_frag = Self::parse_fragment(&Self::extract_text(&mut data));
        let clef = Self::parse_clef(&Self::extract_text(&mut data));

        let desc_text = Self::extract_text(&mut data);
        let description = if desc_text.is_empty() {
            None
        } else {
            Some(desc_text)
        };
        let instructor = Self::extract_text(&mut data);

        Ok(Lesson {
            id,
            date,
            phase,
            page,
            lesson: lesson_frag,
            clef,
            description,
            instructor,
        })
    }

    fn extract_text<'a, I>(iter: &mut I) -> String
    where
        I: Iterator<Item = ElementRef<'a>>,
    {
        iter.next()
            .map(|td| td.text().collect::<Vec<_>>().join(" ").trim().to_string())
            .unwrap_or_default()
    }

    fn parse_fragment(val: &str) -> Option<Range> {
        if val.is_empty() {
            return None;
        }
        let parts: Vec<&str> = val.split('-').map(|s| s.trim()).collect();
        match parts.len() {
            2 => Some(Range {
                from: parts[0].to_string(),
                to: parts[1].to_string(),
            }),
            1 => Some(Range {
                from: parts[0].to_string(),
                to: parts[0].to_string(),
            }),
            _ => None,
        }
    }

    fn parse_clef(val: &str) -> Option<Clef> {
        match val.to_uppercase().as_str() {
            "SOL" => Some(Clef::G),
            "FÁ" | "FA" => Some(Clef::F),
            "DÓ" | "DO" => Some(Clef::C),
            _ => None,
        }
    }
}
