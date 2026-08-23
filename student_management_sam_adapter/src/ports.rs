use sam::client::{Authenticated, MsaLesson, MtdLesson, SamClient};

pub trait MsaSource: Send + Sync {
    fn msa_lessons(&self, student_id: &str) -> anyhow::Result<Vec<MsaLesson>>;
}

pub trait MethodSource: Send + Sync {
    fn method_lessons(&self, student_id: &str) -> anyhow::Result<Vec<MtdLesson>>;
}

impl MsaSource for SamClient<Authenticated> {
    fn msa_lessons(&self, student_id: &str) -> anyhow::Result<Vec<MsaLesson>> {
        SamClient::msa_lessons(self, student_id)
    }
}

impl MethodSource for SamClient<Authenticated> {
    fn method_lessons(&self, student_id: &str) -> anyhow::Result<Vec<MtdLesson>> {
        SamClient::method_lessons(self, student_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::SamCredentials;

    async fn logged_in_client(uri: &str) -> SamClient<Authenticated> {
        SamClient::new(uri)
            .expect("Client should be created")
            .login(&SamCredentials {
                login: "u".to_owned(),
                password: "p".to_owned(),
            })
            .expect("Login should succeed")
    }

    #[test]
    fn given_real_sam_client_msa_source_should_delegate_to_client() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/licoes/index/500132"))
                .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                    r#"<html><body><div id="msa"><table id="datatable1"><tbody>
                        <tr id="msa_559783"><td>09/09/2025</td><td>4.5 - 4.5</td><td>38 - 38</td><td>7 - 8</td><td>Sol</td><td>obs</td><td>MARCOS ROGÉRIO COSME</td></tr>
                    </tbody></table></div></body></html>"#,
                ))
                .mount(&mock_server)
                .await;

            let client = logged_in_client(&mock_server.uri()).await;
            let lessons: Vec<MsaLesson> =
                MsaSource::msa_lessons(&client, "500132").expect("Delegation should succeed");

            assert_eq!(lessons.len(), 1);
            assert_eq!(lessons[0].id, "559783");
        });
    }

    #[test]
    fn given_real_sam_client_method_source_should_delegate_to_client() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/metodo/licoes/500132"))
                .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                    r#"<html><body><table id="datatable3"><tbody>
                        <tr id="mtd_214020"><td>00</td><td>00</td><td>MÉTODO CCB - SCHIMOLL - VIOLINO</td><td>04/12/2023</td><td>MURILO FAGNER CARDOSO</td><td>04/12/2023 21:17:17</td><td>Postura do violino</td></tr>
                    </tbody></table></body></html>"#,
                ))
                .mount(&mock_server)
                .await;

            let client = logged_in_client(&mock_server.uri()).await;
            let lessons: Vec<MtdLesson> =
                MethodSource::method_lessons(&client, "500132").expect("Delegation should succeed");

            assert_eq!(lessons.len(), 1);
            assert_eq!(lessons[0].id, "214020");
        });
    }
}
