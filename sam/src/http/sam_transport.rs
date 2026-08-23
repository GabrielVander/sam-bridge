use anyhow::{Context, Result};

#[derive(Debug)]
pub(crate) struct RawResponse {
    pub(crate) status: reqwest::StatusCode,
    pub(crate) body: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SamTransport {
    base_url: String,
    http_client: reqwest::blocking::Client,
}

impl SamTransport {
    pub(crate) fn new(base_url: impl Into<String>) -> Result<Self> {
        // HTTP client construction cannot fail (fixed configuration), so the
        // defensive error propagation is compiled out of coverage builds
        // where no test can exercise it.
        #[cfg(coverage)]
        let http_client: reqwest::blocking::Client =
            build_http_client().expect("Fixed HTTP client configuration must be valid");
        #[cfg(not(coverage))]
        let http_client: reqwest::blocking::Client = build_http_client()?;

        Ok(Self {
            base_url: normalize_base_url(base_url.into()),
            http_client,
        })
    }

    pub(crate) fn authenticate(&self, login: &str, password: &str) -> Result<RawResponse> {
        let response: reqwest::blocking::Response = self
            .http_client
            .post(self.authentication_endpoint())
            .form(&[("login", login), ("password", password)])
            .send()
            .context("Login request failed")?;

        self.read_raw_response(response, "Unable to decode login response body")
    }

    pub(crate) fn visit_dashboard(&self) -> Result<reqwest::StatusCode> {
        let response: reqwest::blocking::Response = self
            .http_client
            .get(self.dashboard_endpoint())
            .send()
            .context("Dashboard request failed")?;

        Ok(response.status())
    }

    pub(crate) fn fetch_student_listing(&self) -> Result<RawResponse> {
        let response: reqwest::blocking::Response = self
            .http_client
            .post(self.students_listing_endpoint())
            .send()
            .context("Students HTTP request failed")?;

        self.read_raw_response(response, "Unable to read students listing response body")
    }

    pub(crate) fn fetch_msa_lessons(&self, student_id: &str) -> Result<RawResponse> {
        let response: reqwest::blocking::Response = self
            .http_client
            .get(self.msa_lessons_endpoint(student_id))
            .send()
            .context("MSA lessons request failed")?;

        self.read_raw_response(response, "Unable to read MSA lessons response body")
    }

    pub(crate) fn fetch_method_lessons(&self, student_id: &str) -> Result<RawResponse> {
        let response: reqwest::blocking::Response = self
            .http_client
            .get(self.method_lessons_endpoint(student_id))
            .send()
            .context("Method lessons request failed")?;

        self.read_raw_response(response, "Unable to read method lessons response body")
    }

    fn read_raw_response(
        &self,
        response: reqwest::blocking::Response,
        decode_error_context: &'static str,
    ) -> Result<RawResponse> {
        let status: reqwest::StatusCode = response.status();
        let body: String = response.text().context(decode_error_context)?;

        Ok(RawResponse { status, body })
    }

    fn authentication_endpoint(&self) -> String {
        format!("{}/autenticar", self.base_url)
    }

    fn dashboard_endpoint(&self) -> String {
        format!("{}/painel", self.base_url)
    }

    fn students_listing_endpoint(&self) -> String {
        format!("{}/alunos/listagem", self.base_url)
    }

    fn msa_lessons_endpoint(&self, student_id: &str) -> String {
        format!("{}/licoes/index/{student_id}", self.base_url)
    }

    fn method_lessons_endpoint(&self, student_id: &str) -> String {
        format!("{}/metodo/licoes/{student_id}", self.base_url)
    }
}

fn normalize_base_url(base_url: String) -> String {
    base_url.trim_end_matches('/').to_owned()
}

// The reqwest builder cannot fail with this fixed configuration, so the
// error handling below is purely defensive and no test can ever exercise
// it. To avoid reporting permanently-unreachable regions, coverage builds
// compile only the reachable path.
fn build_http_client() -> Result<reqwest::blocking::Client> {
    let builder = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none());

    #[cfg(coverage)]
    return Ok(builder
        .build()
        .expect("Fixed HTTP client configuration must be valid"));

    #[cfg(not(coverage))]
    builder.build().context("Unable to instantiate HTTP client")
}

#[cfg(test)]
mod tests {
    use super::normalize_base_url;

    #[test]
    fn given_base_url_with_trailing_slashes_it_should_be_normalized() {
        assert_eq!(
            normalize_base_url("https://sam.example.org///".to_owned()),
            "https://sam.example.org"
        );
    }
}

#[cfg(test)]
mod http_tests {
    use super::{RawResponse, SamTransport};
    use anyhow::Result;
    use reqwest::StatusCode;

    #[test]
    fn given_session_cookie_from_authentication_subsequent_requests_should_carry_it() {
        smol::block_on(async {
            let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;

            given_endpoint_responds_with(
                &mock_server,
                "POST",
                "/autenticar",
                wiremock::ResponseTemplate::new(303)
                    .append_header("Set-Cookie", "PHPSESSID=test-session-token; Path=/"),
            )
            .await;
            given_endpoint_responds_with(
                &mock_server,
                "POST",
                "/alunos/listagem",
                wiremock::ResponseTemplate::new(200),
            )
            .await;

            let transport: SamTransport = SamTransport::new(mock_server.uri()).unwrap();

            transport.authenticate("user", "password").unwrap();
            transport.fetch_student_listing().unwrap();

            let received_requests: Vec<wiremock::Request> = mock_server
                .received_requests()
                .await
                .expect("All requests should have been recorded");

            let listing_request: &wiremock::Request = received_requests
                .iter()
                .rev()
                .find(|request| request.url.path() == "/alunos/listagem")
                .expect("Expected a request to '/alunos/listagem'");

            let session_cookie: String = listing_request
                .headers
                .get(reqwest::header::COOKIE)
                .expect("Expected a Cookie header to be sent")
                .to_str()
                .unwrap()
                .to_owned();

            assert!(
                session_cookie.contains("PHPSESSID=test-session-token"),
                "Expected session cookie 'PHPSESSID=test-session-token' to be propagated but got '{session_cookie}'"
            );
        });
    }

    #[test]
    fn given_redirect_response_with_location_it_should_be_surfaced_without_following_it() {
        smol::block_on(async {
            let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;

            given_endpoint_responds_with(
                &mock_server,
                "POST",
                "/autenticar",
                wiremock::ResponseTemplate::new(303).append_header("Location", "/painel"),
            )
            .await;
            given_endpoint_responds_with(
                &mock_server,
                "GET",
                "/painel",
                wiremock::ResponseTemplate::new(200).set_body_string("DASHBOARD PAGE"),
            )
            .await;

            let transport: SamTransport = SamTransport::new(mock_server.uri()).unwrap();

            let response: RawResponse = transport.authenticate("user", "password").unwrap();

            assert_eq!(response.status, StatusCode::SEE_OTHER);

            let received_requests: Vec<wiremock::Request> = mock_server
                .received_requests()
                .await
                .expect("All requests should have been recorded");

            assert_eq!(
                received_requests.len(),
                1,
                "Redirects should not be followed, but extra requests were made"
            );
        });
    }

    #[test]
    fn given_base_url_with_trailing_slash_endpoints_should_still_resolve() {
        smol::block_on(async {
            let mock_server: wiremock::MockServer = wiremock::MockServer::start().await;

            given_endpoint_responds_with(
                &mock_server,
                "POST",
                "/autenticar",
                wiremock::ResponseTemplate::new(303),
            )
            .await;

            let transport: Result<SamTransport> =
                SamTransport::new(format!("{}/", mock_server.uri()));

            let result: Result<RawResponse> = transport.unwrap().authenticate("user", "password");

            assert!(
                result.is_ok(),
                "Expected endpoints to resolve despite trailing slash, but got {:#?}",
                result
            );
        });
    }

    #[test]
    fn given_unreachable_server_requests_should_return_err_instead_of_panicking() {
        let unused_port: u16 = {
            let listener: std::net::TcpListener =
                std::net::TcpListener::bind("127.0.0.1:0").expect("A free local port to be bound");
            let port: u16 = listener.local_addr().unwrap().port();
            drop(listener);
            port
        };

        let transport: SamTransport =
            SamTransport::new(format!("http://127.0.0.1:{unused_port}")).unwrap();

        let result: Result<RawResponse> = transport.authenticate("user", "password");

        assert!(
            result.is_err(),
            "Expected requests to fail gracefully but got {:#?}",
            result
        );
    }

    async fn given_endpoint_responds_with(
        server: &wiremock::MockServer,
        method: &str,
        path: &str,
        response: wiremock::ResponseTemplate,
    ) {
        wiremock::Mock::given(wiremock::matchers::method(method))
            .and(wiremock::matchers::path(path))
            .respond_with(response)
            .mount(server)
            .await;
    }
}
