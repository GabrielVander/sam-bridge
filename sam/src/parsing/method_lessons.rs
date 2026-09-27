use std::vec::IntoIter;

use crate::parsing::dom::{
    descendants_with_tag, find_descendant, find_descendant_with_id, optional_cell, row_id,
};

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct MtdLesson {
    pub id: Option<String>,
    pub pages: Option<String>,
    pub lesson: Option<String>,
    pub method: Option<String>,
    pub date: Option<String>,
    pub authorizer: Option<String>,
    pub registration_date: Option<String>,
    pub observations: Option<String>,
}

pub fn parse_method_lessons_body(body: &str) -> Vec<MtdLesson> {
    if body.trim().is_empty() {
        return Vec::new();
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let root: scraper::ElementRef = document.root_element();

    let Some(mtd_table): Option<scraper::ElementRef> =
        find_descendant_with_id(root, "table", "datatable3")
    else {
        return Vec::new();
    };

    let Some(body_element): Option<scraper::ElementRef> = find_descendant(mtd_table, "tbody")
    else {
        return Vec::new();
    };

    descendants_with_tag(body_element, "tr")
        .into_iter()
        .map(parse_row)
        .collect()
}

fn parse_row(row: scraper::ElementRef) -> MtdLesson {
    let mut cells: IntoIter<scraper::ElementRef<'_>> = descendants_with_tag(row, "td").into_iter();

    MtdLesson {
        id: row_id(row, "mtd_"),
        pages: optional_cell(&mut cells),
        lesson: optional_cell(&mut cells),
        method: optional_cell(&mut cells),
        date: optional_cell(&mut cells),
        authorizer: optional_cell(&mut cells),
        registration_date: optional_cell(&mut cells),
        observations: optional_cell(&mut cells),
    }
}
