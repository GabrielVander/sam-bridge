use std::vec::IntoIter;

use crate::parsing::dom::{
    descendants_with_tag, find_descendant, find_descendant_with_id, optional_cell, row_id,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MsaLesson {
    pub id: Option<String>,
    pub date: Option<String>,
    pub phases: Option<String>,
    pub pages: Option<String>,
    pub lessons: Option<String>,
    pub clefs: Option<String>,
    pub description: Option<String>,
    pub authorizer: Option<String>,
}

pub fn parse_msa_lessons_body(body: &str) -> Vec<MsaLesson> {
    if body.trim().is_empty() {
        return Vec::new();
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let root: scraper::ElementRef = document.root_element();

    let Some(msa_section): Option<scraper::ElementRef> =
        find_descendant_with_id(root, "div", "msa")
    else {
        return Vec::new();
    };

    let Some(msa_table): Option<scraper::ElementRef> = find_descendant(msa_section, "table") else {
        return Vec::new();
    };

    let Some(body_element): Option<scraper::ElementRef> = find_descendant(msa_table, "tbody")
    else {
        return Vec::new();
    };

    descendants_with_tag(body_element, "tr")
        .into_iter()
        .map(parse_row)
        .collect()
}

fn parse_row(row: scraper::ElementRef) -> MsaLesson {
    let mut cells: IntoIter<scraper::ElementRef<'_>> = descendants_with_tag(row, "td").into_iter();

    MsaLesson {
        id: row_id(row, "msa_"),
        date: optional_cell(&mut cells),
        phases: optional_cell(&mut cells),
        pages: optional_cell(&mut cells),
        lessons: optional_cell(&mut cells),
        clefs: optional_cell(&mut cells),
        description: optional_cell(&mut cells),
        authorizer: optional_cell(&mut cells),
    }
}
