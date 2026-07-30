use crate::features::basic_sam_site_interop::infra::models::msa_lesson_model::MsaLessonModel;

pub(crate) struct MsaLessonHtmlParser {
    table_selector: scraper::Selector,
    body_row_selector: scraper::Selector,
    cell_selector: scraper::Selector,
}

impl MsaLessonHtmlParser {
    pub(crate) fn new() -> anyhow::Result<Self> {
        let table_selector: scraper::Selector = scraper::Selector::parse(r#"div[id=msa].table"#)
            .map_err(|_| anyhow::anyhow!("Failed to compile table selector"))?;
        let body_row_selector: scraper::Selector = scraper::Selector::parse(r#"tbody.tr"#)
            .map_err(|_| anyhow::anyhow!("Failed to compile row selector"))?;
        let cell_selector: scraper::Selector = scraper::Selector::parse("td")
            .map_err(|_| anyhow::anyhow!("Failed to compile cell selector"))?;

        Ok(Self {
            table_selector,
            body_row_selector,
            cell_selector,
        })
    }

    pub fn parse(&self, raw_html: &str) -> anyhow::Result<Vec<MsaLessonModel>> {
        let html: scraper::Html = scraper::Html::parse_document(raw_html);
        let msa_table: scraper::ElementRef = html
            .select(&self.table_selector)
            .next()
            .ok_or(anyhow::anyhow!("No MSA lessons table found"))?;
        let msa_rows = msa_table.select(&self.body_row_selector);

        msa_rows.map(|row| self.parse_row(row)).collect()
    }

    fn parse_row(&self, row: scraper::ElementRef) -> anyhow::Result<MsaLessonModel> {
        let mut cells: scraper::element_ref::Select = row.select(&self.cell_selector);

        let id: Option<String> = row.value().attr("id").map(|value| value.to_string());
        let date: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let phases: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let pages: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let lessons: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let clefs: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let description: Option<String> = cells.next().map(Self::extract_text_from_html_element);
        let authorizer: Option<String> = cells.next().map(Self::extract_text_from_html_element);

        Ok(MsaLessonModel {
            id,
            date,
            phases,
            pages,
            lessons,
            clefs,
            description,
            authorizer,
        })
    }

    fn extract_text_from_html_element(e: scraper::ElementRef) -> String {
        e.text().collect::<Vec<&str>>().join(" ").trim().to_string()
    }
}
