use student_management_sam_adapter::{
    Authenticated, SamClient,
    gateways::{SamLessonsGateway, SamRosterGateway},
};

#[derive(Clone)]
pub struct AppGateways {
    pub roster: SamRosterGateway<SamClient<Authenticated>>,
    pub lessons: SamLessonsGateway<SamClient<Authenticated>>,
}

impl AppGateways {
    pub fn from_session(client: &SamClient<Authenticated>) -> Self {
        Self {
            roster: SamRosterGateway::new(client.clone()),
            lessons: SamLessonsGateway::new(client.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use student_management::api::application::StudentsRetrievalGateway;

    #[test]
    fn given_authenticated_session_should_build_all_gateways() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/painel"))
                .respond_with(wiremock::ResponseTemplate::new(200))
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/alunos/listagem"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200).set_body_string(
                        r#"{"draw":"1","recordsTotal":0,"recordsFiltered":0,"data":[]}"#,
                    ),
                )
                .mount(&mock_server)
                .await;
            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/licoes/index/1"))
                .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(""))
                .mount(&mock_server)
                .await;

            let client = student_management_sam_adapter::gateways::login(
                SamClient::new(mock_server.uri()).expect("Client should be created"),
                "u".to_owned(),
                "p".to_owned(),
            )
            .await
            .expect("Login should succeed");

            let gateways = AppGateways::from_session(&client);

            assert!(gateways.roster.get_avaliable_records().await.is_ok());
            assert!(
                student_management::api::application::StudentLessonsGateway::get_all_for_student_with_id(
                    &gateways.lessons, "1"
                )
                .await
                .is_ok()
            );
        });
    }
}
