use anyhow::bail;

pub(crate) fn parse_session_status(dashboard_status: reqwest::StatusCode) -> anyhow::Result<()> {
    if dashboard_status != reqwest::StatusCode::OK {
        bail!(
            "Session invalid or expired. Expected dashboard to return 200 OK but got {dashboard_status}"
        );
    }

    Ok(())
}

#[cfg(test)]
mod session_tests {
    use super::parse_session_status;
    use reqwest::StatusCode;

    #[test]
    fn given_ok_dashboard_session_should_be_active() {
        let result = parse_session_status(StatusCode::OK);

        result.expect("Session should be active");
    }

    #[test]
    fn given_non_ok_dashboard_session_should_fail_with_session_error() {
        let error_test_cases: Vec<u16> = vec![302, 404, 503];

        for status_code in error_test_cases {
            let result = parse_session_status(
                StatusCode::from_u16(status_code).expect("A valid status code"),
            );

            assert!(
                result.is_err(),
                "Expected session check of {status_code} to fail but got {:#?}",
                result
            );
            assert!(
                result.unwrap_err().to_string().contains("Session invalid or expired"),
                "Expected a session-related error for {status_code}"
            );
        }
    }
}
