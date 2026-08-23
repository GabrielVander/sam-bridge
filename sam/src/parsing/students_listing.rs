use anyhow::{Context, bail};

#[derive(Debug, PartialEq)]
pub struct SamStudent {
    pub id: String,
    pub name: String,
    pub location: String,
    pub role: String,
    pub instrument: String,
    pub level: String,
}

pub(crate) fn parse_students_listing(
    response_status: reqwest::StatusCode,
    body: &str,
) -> anyhow::Result<Vec<SamStudent>> {
    if response_status != reqwest::StatusCode::OK {
        bail!("Unexpected status for student listing response: {response_status:?}");
    }

    let response: SamStudentsJsonResponse =
        serde_json::from_str(body).context("Unable to decode student listing JSON response")?;

    Ok(response.data.iter().map(SamStudent::from).collect())
}

#[derive(serde::Deserialize, Debug)]
struct SamStudentsJsonResponse {
    data: Vec<SamSingleStudentJsonResponse>,
}

type SamSingleStudentJsonResponse = Vec<String>;

impl From<&SamSingleStudentJsonResponse> for SamStudent {
    fn from(value: &SamSingleStudentJsonResponse) -> Self {
        let id: String = value
            .first()
            .map(|i| i.to_string())
            .unwrap_or("".to_string());
        let name: String = value
            .get(1)
            .map(|i| i.to_string())
            .unwrap_or("".to_string());
        let location: String = value
            .get(2)
            .map(|i| i.to_string())
            .unwrap_or("".to_string());
        let role: String = value
            .get(3)
            .map(|i| i.to_string())
            .unwrap_or("".to_string());
        let instrument: String = value
            .get(4)
            .map(|i| i.to_string())
            .unwrap_or("".to_string());
        let level: String = value
            .get(5)
            .map(|i| i.to_string())
            .unwrap_or("".to_string());

        SamStudent {
            id,
            name,
            location,
            role,
            instrument,
            level,
        }
    }
}

#[cfg(test)]
mod students_listing_tests {
    use super::SamStudent;
    use super::parse_students_listing;
    use reqwest::StatusCode;

    #[test]
    fn given_unexpected_response_status_students_listing_should_fail_with_status_error() {
        let result = parse_students_listing(
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
        );

        assert!(result.is_err(), "Expected an Err but got {:#?}", result);
        assert!(
            result
                .unwrap_err()
                .to_string()
                .starts_with("Unexpected status for student listing response: 500"),
            "Expected an unexpected-status error"
        );
    }

    #[test]
    fn given_ok_response_status_students_listing_should_be_parsed() {
        let result = parse_students_listing(
            StatusCode::OK,
            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
        );

        assert_eq!(result.expect("Parsing should succeed"), vec![]);
    }

    #[test]
    fn given_empty_data_should_return_no_students() {
        let result = parse_students_listing(
            StatusCode::OK,
            r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
        );

        assert_eq!(result.expect("Parsing should succeed"), vec![]);
    }

    #[test]
    fn given_valid_row_should_return_mapped_student() {
        let result = parse_students_listing(
            StatusCode::OK,
            r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["99999","CARLOS ALBERTO DE NOBREGA","JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","A DEFINIR","CANDIDATO(A)","99999","0"]]}"#,
        );

        assert_eq!(
            result.expect("Parsing should succeed"),
            vec![SamStudent {
                id: "99999".to_string(),
                name: "CARLOS ALBERTO DE NOBREGA".to_string(),
                location:
                    "JARDIM PALMARES DO SUL <span class='m-r-10'></span> | <span class='m-r-10'></span> BR-SP-ARARAQUARA-SÃO CARLOS"
                        .to_string(),
                role: "MÚSICO".to_string(),
                instrument: "A DEFINIR".to_string(),
                level: "CANDIDATO(A)".to_string(),
            }]
        );
    }

    #[test]
    fn given_unknown_fields_and_extra_row_columns_they_should_be_ignored() {
        let result = parse_students_listing(
            StatusCode::OK,
            r#"{"other":"field","data":[["1","N","L","R","I","LV","EXTRA-1","EXTRA-2"]]}"#,
        );

        assert_eq!(
            result.expect("Parsing should succeed"),
            vec![SamStudent {
                id: "1".to_string(),
                name: "N".to_string(),
                location: "L".to_string(),
                role: "R".to_string(),
                instrument: "I".to_string(),
                level: "LV".to_string(),
            }]
        );
    }

    #[test]
    fn given_rows_with_missing_trailing_columns_they_should_default_to_empty_strings() {
        let result = parse_students_listing(StatusCode::OK, r#"{"data":[["1","NAME"]]}"#);

        assert_eq!(
            result.expect("Parsing should succeed"),
            vec![SamStudent {
                id: "1".to_string(),
                name: "NAME".to_string(),
                location: "".to_string(),
                role: "".to_string(),
                instrument: "".to_string(),
                level: "".to_string(),
            }]
        );
    }

    #[test]
    fn given_malformed_json_should_fail_with_decoding_error() {
        let error_test_cases: Vec<&str> = vec![
            "",
            "{}",
            r#"{"something": "else"}"#,
            r#"{"data": [["1", 2, "3", "4", "5", "6"]]}"#,
        ];

        for response_body in error_test_cases {
            let result = parse_students_listing(StatusCode::OK, response_body);

            assert!(
                result.is_err(),
                "Expected parsing of '{response_body}' to fail but got {:#?}",
                result
            );
            assert!(
                result
                    .unwrap_err()
                    .to_string()
                    .starts_with("Unable to decode student listing JSON response"),
                "Expected a decoding error for '{response_body}'"
            );
        }
    }
}
