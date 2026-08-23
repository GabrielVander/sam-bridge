
#[derive(Debug, PartialEq, Clone, Default)]
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

pub(crate) fn parse_method_lessons_body(body: &str) -> Vec<MtdLesson> {
    if body.trim().is_empty() {
        return Vec::new();
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let selectors: &Selectors = selectors();

    let Some(mtd_table): Option<scraper::ElementRef> = document.select(&selectors.mtd_table).next()
    else {
        return Vec::new();
    };

    mtd_table
        .select(&selectors.body_row)
        .map(|row| parse_row(row, &selectors.cell))
        .collect()
}

struct Selectors {
    mtd_table: scraper::Selector,
    body_row: scraper::Selector,
    cell: scraper::Selector,
}

fn selectors() -> &'static Selectors {
    static SELECTORS: std::sync::OnceLock<Selectors> = std::sync::OnceLock::new();

    SELECTORS.get_or_init(|| Selectors {
        mtd_table: scraper::Selector::parse("table#datatable3")
            .expect("The MTD lessons table selector must be valid"),
        body_row: scraper::Selector::parse("tbody tr")
            .expect("The body row selector must be valid"),
        cell: scraper::Selector::parse("td").expect("The cell selector must be valid"),
    })
}

fn parse_row(row: scraper::ElementRef, cell_selector: &scraper::Selector) -> MtdLesson {
    let id = row
        .value()
        .attr("id")
        .and_then(|value| value.strip_prefix("mtd_"))
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    let mut cells: scraper::element_ref::Select = row.select(cell_selector);

    MtdLesson {
        id,
        pages: optional_cell(&mut cells),
        lesson: optional_cell(&mut cells),
        method: optional_cell(&mut cells),
        date: optional_cell(&mut cells),
        authorizer: optional_cell(&mut cells),
        registration_date: optional_cell(&mut cells),
        observations: optional_cell(&mut cells),
    }
}

fn optional_cell(cells: &mut scraper::element_ref::Select) -> Option<String> {
    cells
        .next()
        .map(extract_cell_text)
        .filter(|text| !text.is_empty())
}

fn extract_cell_text(element: scraper::ElementRef) -> String {
    element
        .text()
        .collect::<Vec<&str>>()
        .join(" ")
        .trim()
        .to_owned()
}

#[cfg(test)]
mod method_lessons_tests {
    use super::{MtdLesson, parse_method_lessons_body};

    fn mtd_lessons_page(rows_html: &str) -> String {
        format!(
            r#"<html><body><table id="datatable3" class="table table-striped table-bordered table-hover dataTable no-footer" role="grid">
    <thead>
        <tr class="active" role="row"><th>Páginas</th><th>Lição</th><th>Método</th><th>Data da Lição</th><th>Autorizante</th><th>Data de Cadastro</th><th>Observações</th><th>Ações</th></tr>
    </thead>
    <tbody>{rows_html}</tbody>
</table></body></html>"#
        )
    }

    #[test]
    fn given_full_mtd_table_should_return_all_rows() {
        let response_body: &str = &mtd_lessons_page(
            r#"<tr id="mtd_738654" role="row" class="odd">
            <td>00</td>
            <td>00</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>24/03/2026</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>30/03/2026 18:38:29</td>
            <td>Revisão para RJM </td>
            <td><button onclick="delete_lancamento_mtd(738654)">Apagar</button></td>
        </tr><tr id="mtd_190204" role="row" class="even">
            <td>11</td>
            <td>7</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>23/10/2023</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>23/10/2023 20:35:24</td>
            <td></td>
            <td><button onclick="delete_lancamento_mtd(190204)">Apagar</button></td>
        </tr>"#,
        );

        let result = parse_method_lessons_body(response_body);

        assert_eq!(
            result,
            vec![
                MtdLesson {
                    id: Some("738654".to_string()),
                    pages: Some("00".to_string()),
                    lesson: Some("00".to_string()),
                    method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_string()),
                    date: Some("24/03/2026".to_string()),
                    authorizer: Some("MURILO FAGNER CARDOSO".to_string()),
                    registration_date: Some("30/03/2026 18:38:29".to_string()),
                    observations: Some("Revisão para RJM".to_string()),
                },
                MtdLesson {
                    id: Some("190204".to_string()),
                    pages: Some("11".to_string()),
                    lesson: Some("7".to_string()),
                    method: Some("MÉTODO CCB - SCHIMOLL - VIOLINO".to_string()),
                    date: Some("23/10/2023".to_string()),
                    authorizer: Some("MURILO FAGNER CARDOSO".to_string()),
                    registration_date: Some("23/10/2023 20:35:24".to_string()),
                    observations: None,
                },
            ]
        );
    }

    #[test]
    fn given_row_with_every_field_absent_should_still_be_returned() {
        let response_body: &str = &mtd_lessons_page(
            "<tr><td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td></tr>",
        );

        let result = parse_method_lessons_body(response_body);

        assert_eq!(
            result,
            vec![MtdLesson::default()],
            "Rows without any field must not be dropped"
        );
    }

    #[test]
    fn given_row_with_empty_table_should_return_no_lessons() {
        let result = parse_method_lessons_body(&mtd_lessons_page(""));

        assert_eq!(result, vec![]);
    }

    #[test]
    fn given_html_without_mtd_table_should_return_empty_list_not_error() {
        let result = parse_method_lessons_body("<html><body><h1>Informação não encontrada</h1></body></html>",
        );

        assert_eq!(result, vec![]);
    }

    #[test]
    fn given_empty_body_should_return_no_lessons() {
        for empty_body in ["", "   "] {
            let result = parse_method_lessons_body(empty_body);

            assert_eq!(
                result,
                vec![],
                "Expected no lessons for '{empty_body}'"
            );
        }
    }

}
