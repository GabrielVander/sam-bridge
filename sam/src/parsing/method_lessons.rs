use anyhow::bail;

#[derive(Debug, PartialEq, Clone)]
pub struct MtdLesson {
    pub id: String,
    pub pages: String,
    pub lesson: Option<String>,
    pub method: String,
    pub date: String,
    pub authorizer: String,
    pub registration_date: String,
    pub observations: Option<String>,
}

pub(crate) fn parse_method_lessons(
    response_status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Result<Vec<MtdLesson>> {
    if response_status != reqwest::StatusCode::OK {
        bail!("Unexpected status for method lessons response: {response_status:?}");
    }

    if body.trim().is_empty() {
        return Ok(Vec::new());
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let selectors: &Selectors = selectors();

    let mtd_table: scraper::ElementRef = document
        .select(&selectors.mtd_table)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No MTD lessons table found"))?;

    mtd_table
        .select(&selectors.body_row)
        .map(|row| parse_mtd_lesson_row(row, &selectors.cell))
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

fn parse_mtd_lesson_row(
    row: scraper::ElementRef,
    cell_selector: &scraper::Selector,
) -> anyhow::Result<MtdLesson> {
    let id: String = row
        .value()
        .attr("id")
        .and_then(|value| value.strip_prefix("mtd_"))
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("MTD lesson row is missing a valid 'id'"))?;

    let mut cells: scraper::element_ref::Select = row.select(cell_selector);

    let pages: String = required_cell(&mut cells, "pages")?;
    let lesson: Option<String> = optional_cell(&mut cells);
    let method: String = required_cell(&mut cells, "method")?;
    let date: String = required_cell(&mut cells, "date")?;
    let authorizer: String = required_cell(&mut cells, "authorizer")?;
    let registration_date: String = required_cell(&mut cells, "registration_date")?;
    let observations: Option<String> = optional_cell(&mut cells);

    Ok(MtdLesson {
        id,
        pages,
        lesson,
        method,
        date,
        authorizer,
        registration_date,
        observations,
    })
}

fn required_cell(cells: &mut scraper::element_ref::Select, field: &str) -> anyhow::Result<String> {
    match cells.next().map(extract_cell_text) {
        Some(text) if !text.is_empty() => Ok(text),
        _ => bail!("MTD lesson row is missing '{field}'"),
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
    use super::{MtdLesson, parse_method_lessons};
    use reqwest::StatusCode;

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
            <td><button type="button" class="btn btn-danger btn-sm" onclick="delete_lancamento_mtd(738654)">Apagar</button></td>
        </tr><tr id="mtd_214020" role="row" class="even">
            <td>00</td>
            <td>00</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>04/12/2023</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>04/12/2023 21:17:17</td>
            <td>Postura do violino </td>
            <td><button type="button" class="btn btn-danger btn-sm" onclick="delete_lancamento_mtd(214020)">Apagar</button></td>
        </tr><tr id="mtd_190204" role="row" class="even">
            <td>11</td>
            <td>7</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>23/10/2023</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>23/10/2023 20:35:24</td>
            <td></td>
            <td><button type="button" class="btn btn-danger btn-sm" onclick="delete_lancamento_mtd(190204)">Apagar</button></td>
        </tr>"#,
        );

        let result = parse_method_lessons(StatusCode::OK, response_body);

        assert_eq!(
            result.expect("Parsing should succeed"),
            vec![
                MtdLesson {
                    id: "738654".to_string(),
                    pages: "00".to_string(),
                    lesson: Some("00".to_string()),
                    method: "MÉTODO CCB - SCHIMOLL - VIOLINO".to_string(),
                    date: "24/03/2026".to_string(),
                    authorizer: "MURILO FAGNER CARDOSO".to_string(),
                    registration_date: "30/03/2026 18:38:29".to_string(),
                    observations: Some("Revisão para RJM".to_string()),
                },
                MtdLesson {
                    id: "214020".to_string(),
                    pages: "00".to_string(),
                    lesson: Some("00".to_string()),
                    method: "MÉTODO CCB - SCHIMOLL - VIOLINO".to_string(),
                    date: "04/12/2023".to_string(),
                    authorizer: "MURILO FAGNER CARDOSO".to_string(),
                    registration_date: "04/12/2023 21:17:17".to_string(),
                    observations: Some("Postura do violino".to_string()),
                },
                MtdLesson {
                    id: "190204".to_string(),
                    pages: "11".to_string(),
                    lesson: Some("7".to_string()),
                    method: "MÉTODO CCB - SCHIMOLL - VIOLINO".to_string(),
                    date: "23/10/2023".to_string(),
                    authorizer: "MURILO FAGNER CARDOSO".to_string(),
                    registration_date: "23/10/2023 20:35:24".to_string(),
                    observations: None,
                },
            ]
        );
    }

    #[test]
    fn given_row_with_empty_table_should_return_no_lessons() {
        let result = parse_method_lessons(StatusCode::OK, &mtd_lessons_page(""));

        assert_eq!(result.expect("Parsing should succeed"), vec![]);
    }

    #[test]
    fn given_missing_required_cell_should_fail_loudly() {
        let missing_registration_date_row: &str = r#"<tr id="mtd_738654">
            <td>00</td>
            <td>00</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>24/03/2026</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td></td>
            <td>Revisão para RJM </td>
        </tr>"#;

        let result = parse_method_lessons(
            StatusCode::OK,
            &mtd_lessons_page(missing_registration_date_row),
        );

        assert!(
            result.is_err(),
            "Expected a row without a registration_date to fail but got {:#?}",
            result
        );
        assert!(
            result.unwrap_err().to_string().contains("missing"),
            "Expected a loud error naming the missing field"
        );
    }

    #[test]
    fn given_rows_missing_each_leading_required_field_should_fail_naming_it() {
        let missing_pages_row: &str = r#"<tr id="mtd_1"></tr>"#;
        let missing_method_row: &str = r#"<tr id="mtd_2"><td>00</td><td></td></tr>"#;
        let missing_date_row: &str =
            r#"<tr id="mtd_3"><td>00</td><td></td><td>MÉTODO CCB - SCHIMOLL - VIOLINO</td></tr>"#;
        let missing_authorizer_row: &str = r#"<tr id="mtd_4"><td>00</td><td></td><td>MÉTODO CCB - SCHIMOLL - VIOLINO</td><td>24/03/2026</td></tr>"#;
        let missing_registration_date_row: &str = r#"<tr id="mtd_5"><td>00</td><td></td><td>MÉTODO CCB - SCHIMOLL - VIOLINO</td><td>24/03/2026</td><td>MURILO FAGNER CARDOSO</td></tr>"#;

        let error_test_cases: Vec<(&str, &str)> = vec![
            (missing_pages_row, "missing 'pages'"),
            (missing_method_row, "missing 'method'"),
            (missing_date_row, "missing 'date'"),
            (missing_authorizer_row, "missing 'authorizer'"),
            (missing_registration_date_row, "missing 'registration_date'"),
        ];

        for (row_html, expected_error) in error_test_cases {
            let result = parse_method_lessons(StatusCode::OK, &mtd_lessons_page(row_html));

            assert!(
                result.is_err(),
                "Expected '{row_html}' to fail but got {:#?}",
                result
            );
            assert!(
                result.unwrap_err().to_string().contains(expected_error),
                "Expected error naming {expected_error}"
            );
        }
    }

    #[test]
    fn given_row_without_valid_id_should_fail_loudly() {
        let row_without_id: &str = r#"<tr class="odd">
            <td>00</td>
            <td>00</td>
            <td>MÉTODO CCB - SCHIMOLL - VIOLINO</td>
            <td>24/03/2026</td>
            <td>MURILO FAGNER CARDOSO</td>
            <td>30/03/2026 18:38:29</td>
            <td>Revisão.</td>
        </tr>"#;

        let result = parse_method_lessons(StatusCode::OK, &mtd_lessons_page(row_without_id));

        assert!(
            result.is_err(),
            "Expected a row without an id to fail but got {:#?}",
            result
        );
    }

    #[test]
    fn given_html_without_mtd_table_should_fail_with_table_error() {
        let result = parse_method_lessons(
            StatusCode::OK,
            "<html><body><h1>Lições de Método</h1></body></html>",
        );

        assert!(
            result.is_err(),
            "Expected parsing without an MTD table to fail but got {:#?}",
            result
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("No MTD lessons table found"),
            "Expected a missing-table error"
        );
    }

    #[test]
    fn given_empty_body_should_return_no_lessons() {
        for empty_body in ["", "   "] {
            let result = parse_method_lessons(StatusCode::OK, empty_body);

            assert_eq!(
                result.expect("Parsing should succeed"),
                vec![],
                "Expected no lessons for '{empty_body}'"
            );
        }
    }

    #[test]
    fn given_unexpected_response_status_should_fail_with_status_error() {
        let result = parse_method_lessons(
            StatusCode::TEMPORARY_REDIRECT,
            &mtd_lessons_page(
                "<tr id=\"mtd_1\"><td>pg</td><td>lsn</td><td>mtd</td><td>dt</td><td>auth</td><td>reg</td><td>obs</td></tr>",
            ),
        );

        assert!(result.is_err(), "Expected an Err but got {:#?}", result);
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("Unexpected status for method lessons response: 307"),
            "Expected an unexpected-status error"
        );
    }
}

