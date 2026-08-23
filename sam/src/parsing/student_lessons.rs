use anyhow::bail;

#[derive(Debug, PartialEq)]
pub struct MsaLesson {
    pub id: String,
    pub date: String,
    pub phases: String,
    pub pages: String,
    pub lessons: Option<String>,
    pub clefs: Option<String>,
    pub description: Option<String>,
    pub authorizer: String,
}

pub(crate) fn parse_student_lessons(
    response_status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Result<Vec<MsaLesson>> {
    if response_status != reqwest::StatusCode::OK {
        bail!("Unexpected status for student lessons response: {response_status:?}");
    }

    if body.trim().is_empty() {
        return Ok(Vec::new());
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let selectors: &Selectors = selectors();

    let msa_table: scraper::ElementRef = document
        .select(&selectors.msa_table)
        .next()
        .ok_or_else(|| anyhow::anyhow!("No MSA lessons table found"))?;

    msa_table
        .select(&selectors.body_row)
        .map(|row| parse_msa_lesson_row(row, &selectors.cell))
        .collect()
}

struct Selectors {
    msa_table: scraper::Selector,
    body_row: scraper::Selector,
    cell: scraper::Selector,
}

fn selectors() -> &'static Selectors {
    static SELECTORS: std::sync::OnceLock<Selectors> = std::sync::OnceLock::new();

    SELECTORS.get_or_init(|| Selectors {
        msa_table: scraper::Selector::parse("div#msa table")
            .expect("The MSA lessons table selector must be valid"),
        body_row: scraper::Selector::parse("tbody tr")
            .expect("The body row selector must be valid"),
        cell: scraper::Selector::parse("td").expect("The cell selector must be valid"),
    })
}

fn parse_msa_lesson_row(
    row: scraper::ElementRef,
    cell_selector: &scraper::Selector,
) -> anyhow::Result<MsaLesson> {
    let id: String = row
        .value()
        .attr("id")
        .and_then(|value| value.strip_prefix("msa_"))
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("MSA lesson row is missing a valid 'id'"))?;

    let mut cells: scraper::element_ref::Select = row.select(cell_selector);

    let date: String = required_cell(&mut cells, "date")?;
    let phases: String = required_cell(&mut cells, "phases")?;
    let pages: String = required_cell(&mut cells, "pages")?;
    let lessons: Option<String> = optional_cell(&mut cells);
    let clefs: Option<String> = optional_cell(&mut cells);
    let description: Option<String> = optional_cell(&mut cells);
    let authorizer: String = required_cell(&mut cells, "authorizer")?;

    Ok(MsaLesson {
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

fn required_cell(cells: &mut scraper::element_ref::Select, field: &str) -> anyhow::Result<String> {
    match cells.next().map(extract_cell_text) {
        Some(text) if !text.is_empty() => Ok(text),
        _ => bail!("MSA lesson row is missing '{field}'"),
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
mod student_lessons_tests {
    use super::{MsaLesson, parse_student_lessons};
    use reqwest::StatusCode;

    fn msa_lessons_page(rows_html: &str) -> String {
        format!(
            r#"<html><body><div id="msa"><table id="datatable1" class="table table-striped table-bordered table-hover table-responsive dataTable no-footer" role="grid">
    <thead>
        <tr class="active" role="row"><th>Data da Lição</th><th>Fases</th><th>Paginas</th><th>Lições</th><th>Claves</th><th>Observações</th><th>Autorizante</th><th>Ações</th></tr>
    </thead>
    <tbody>{rows_html}</tbody>
</table></div></body></html>"#
        )
    }

    #[test]
    fn given_full_msa_table_should_return_all_rows() {
        let response_body: &str = &msa_lessons_page(
            r#"<tr id="msa_538784" role="row" class="odd">
            <td>19/08/2025</td>
            <td>3.4 - 4.1</td>
            <td>30 - 34</td>
            <td></td>
            <td></td>
            <td>Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32. </td>
            <td>ELIAS BRANDE</td>
            <td><button onclick="delete_lancamento_msa(538784)">Apagar</button></td>
        </tr><tr id="msa_559783" role="row" class="even">
            <td>09/09/2025</td>
            <td>4.5 - 4.5</td>
            <td>38 - 38</td>
            <td>7 - 8</td>
            <td>Sol</td>
            <td>Passou lições 7 e 8, estudar próximas lições.</td>
            <td>MARCOS ROGÉRIO COSME</td>
            <td><button onclick="delete_lancamento_msa(559783)">Apagar</button></td>
        </tr>"#,
        );

        let result = parse_student_lessons(StatusCode::OK, response_body);

        assert_eq!(
            result.expect("Parsing should succeed"),
            vec![
                MsaLesson {
                    id: "538784".to_string(),
                    date: "19/08/2025".to_string(),
                    phases: "3.4 - 4.1".to_string(),
                    pages: "30 - 34".to_string(),
                    lessons: None,
                    clefs: None,
                    description: Some(
                        "Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32.".to_string()
                    ),
                    authorizer: "ELIAS BRANDE".to_string(),
                },
                MsaLesson {
                    id: "559783".to_string(),
                    date: "09/09/2025".to_string(),
                    phases: "4.5 - 4.5".to_string(),
                    pages: "38 - 38".to_string(),
                    lessons: Some("7 - 8".to_string()),
                    clefs: Some("Sol".to_string()),
                    description: Some("Passou lições 7 e 8, estudar próximas lições.".to_string()),
                    authorizer: "MARCOS ROGÉRIO COSME".to_string(),
                },
            ]
        );
    }

    #[test]
    fn given_row_with_empty_table_should_return_no_lessons() {
        let result = parse_student_lessons(StatusCode::OK, &msa_lessons_page(""));

        assert_eq!(result.expect("Parsing should succeed"), vec![]);
    }

    #[test]
    fn given_missing_required_cell_should_fail_loudly() {
        let missing_authorizer_row: &str = r#"<tr id="msa_538784">
            <td>19/08/2025</td>
            <td>3.4 - 4.1</td>
            <td>30 - 34</td>
            <td></td>
            <td></td>
            <td>Revisão: Ligaduras.</td>
        </tr>"#;

        let result =
            parse_student_lessons(StatusCode::OK, &msa_lessons_page(missing_authorizer_row));

        assert!(
            result.is_err(),
            "Expected a row without an authorizer to fail but got {:#?}",
            result
        );
        assert!(
            result.unwrap_err().to_string().contains("missing"),
            "Expected a loud error naming the missing field"
        );
    }

    #[test]
    fn given_rows_missing_each_leading_required_field_should_fail_naming_it() {
        let missing_date_row: &str = r#"<tr id="msa_1"></tr>"#;
        let missing_phases_row: &str = r#"<tr id="msa_2"><td>19/08/2025</td></tr>"#;
        let missing_pages_row: &str =
            r#"<tr id="msa_3"><td>19/08/2025</td><td>3.4 - 4.1</td></tr>"#;

        let error_test_cases: Vec<(&str, &str)> = vec![
            (missing_date_row, "missing 'date'"),
            (missing_phases_row, "missing 'phases'"),
            (missing_pages_row, "missing 'pages'"),
        ];

        for (row_html, expected_error) in error_test_cases {
            let result = parse_student_lessons(StatusCode::OK, &msa_lessons_page(row_html));

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
            <td>19/08/2025</td>
            <td>3.4 - 4.1</td>
            <td>30 - 34</td>
            <td></td>
            <td></td>
            <td>Revisão.</td>
            <td>ELIAS BRANDE</td>
        </tr>"#;

        let result = parse_student_lessons(StatusCode::OK, &msa_lessons_page(row_without_id));

        assert!(
            result.is_err(),
            "Expected a row without an id to fail but got {:#?}",
            result
        );
    }

    #[test]
    fn given_html_without_msa_table_should_fail_with_table_error() {
        let result = parse_student_lessons(
            StatusCode::OK,
            "<html><body><h1>Lições Aprovadas</h1></body></html>",
        );

        assert!(
            result.is_err(),
            "Expected parsing without an MSA table to fail but got {:#?}",
            result
        );
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("No MSA lessons table found"),
            "Expected a missing-table error"
        );
    }

    #[test]
    fn given_empty_body_should_return_no_lessons() {
        for empty_body in ["", "   "] {
            let result = parse_student_lessons(StatusCode::OK, empty_body);

            assert_eq!(
                result.expect("Parsing should succeed"),
                vec![],
                "Expected no lessons for '{empty_body}'"
            );
        }
    }

    #[test]
    fn given_unexpected_response_status_should_fail_with_status_error() {
        let result = parse_student_lessons(
            StatusCode::TEMPORARY_REDIRECT,
            &msa_lessons_page(
                "<tr id=\"msa_1\"><td>d</td><td>p</td><td>pg</td><td></td><td></td><td>obs</td><td>a</td></tr>",
            ),
        );

        assert!(result.is_err(), "Expected an Err but got {:#?}", result);
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("Unexpected status for student lessons response: 307"),
            "Expected an unexpected-status error"
        );
    }
}
