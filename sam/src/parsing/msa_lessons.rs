
/// All fields are optional: SAM data is assumed to be potentially absent.
#[derive(Debug, PartialEq, Clone, Default)]
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

pub(crate) fn parse_msa_lessons_body(body: &str) -> Vec<MsaLesson> {
    if body.trim().is_empty() {
        return Vec::new();
    }

    let document: scraper::Html = scraper::Html::parse_document(body);
    let selectors: &Selectors = selectors();

    // A missing table means SAM has no lessons to show (e.g. its
    // "information not found" page) — that is an empty list, not an error.
    let Some(msa_table): Option<scraper::ElementRef> = document.select(&selectors.msa_table).next()
    else {
        return Vec::new();
    };

    msa_table
        .select(&selectors.body_row)
        .map(|row| parse_row(row, &selectors.cell))
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

fn parse_row(row: scraper::ElementRef, cell_selector: &scraper::Selector) -> MsaLesson {
    let id = row
        .value()
        .attr("id")
        .and_then(|value| value.strip_prefix("msa_"))
        .filter(|value| !value.is_empty())
        .map(str::to_owned);

    let mut cells: scraper::element_ref::Select = row.select(cell_selector);

    MsaLesson {
        id,
        date: optional_cell(&mut cells),
        phases: optional_cell(&mut cells),
        pages: optional_cell(&mut cells),
        lessons: optional_cell(&mut cells),
        clefs: optional_cell(&mut cells),
        description: optional_cell(&mut cells),
        authorizer: optional_cell(&mut cells),
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
mod msa_lessons_tests {
    use super::{MsaLesson, parse_msa_lessons_body};

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

        let result = parse_msa_lessons_body(response_body);

        assert_eq!(
            result,
            vec![
                MsaLesson {
                    id: Some("538784".to_string()),
                    date: Some("19/08/2025".to_string()),
                    phases: Some("3.4 - 4.1".to_string()),
                    pages: Some("30 - 34".to_string()),
                    lessons: None,
                    clefs: None,
                    description: Some(
                        "Revisão: Ligaduras. Estudar exercícios 1 e 2, página 32.".to_string()
                    ),
                    authorizer: Some("ELIAS BRANDE".to_string()),
                },
                MsaLesson {
                    id: Some("559783".to_string()),
                    date: Some("09/09/2025".to_string()),
                    phases: Some("4.5 - 4.5".to_string()),
                    pages: Some("38 - 38".to_string()),
                    lessons: Some("7 - 8".to_string()),
                    clefs: Some("Sol".to_string()),
                    description: Some("Passou lições 7 e 8, estudar próximas lições.".to_string()),
                    authorizer: Some("MARCOS ROGÉRIO COSME".to_string()),
                },
            ]
        );
    }

    #[test]
    fn given_row_with_every_field_absent_should_still_be_returned() {
        let response_body: &str = &msa_lessons_page(
            r#"<tr>
            <td></td><td></td><td></td><td></td><td></td><td></td><td></td><td></td>
        </tr><tr id="msa_1"></tr>"#,
        );

        let result = parse_msa_lessons_body(response_body);

        assert_eq!(
            result,
            vec![MsaLesson::default(), MsaLesson { id: Some("1".to_owned()), ..Default::default() }],
            "Rows without authorizer or any other field must not be dropped"
        );
    }

    #[test]
    fn given_row_with_empty_table_should_return_no_lessons() {
        let result = parse_msa_lessons_body(&msa_lessons_page(""));

        assert_eq!(result, vec![]);
    }

    #[test]
    fn given_html_without_msa_table_should_return_empty_list_not_error() {
        let result = parse_msa_lessons_body("<html><body><h1>Informação não encontrada</h1></body></html>",
        );

        assert_eq!(result, vec![]);
    }

    #[test]
    fn given_empty_body_should_return_no_lessons() {
        for empty_body in ["", "   "] {
            let result = parse_msa_lessons_body(empty_body);

            assert_eq!(
                result,
                vec![],
                "Expected no lessons for '{empty_body}'"
            );
        }
    }

}
