use sam::client::{Authenticated, SamClient, SamCredentials, SamStudent, StudentLessonsPage, Unauthenticated};

pub trait AuthSource: Send + Sync {
    fn login(&self, username: &str, password: &str) -> anyhow::Result<SamClient<Authenticated>>;
}

pub trait RosterSource: Send + Sync {
    fn students(&self) -> anyhow::Result<Vec<SamStudent>>;
}

pub trait LessonsSource: Send + Sync {
    fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage>;
}

impl AuthSource for SamClient<Unauthenticated> {
    fn login(&self, username: &str, password: &str) -> anyhow::Result<SamClient<Authenticated>> {
        self.clone().login(&SamCredentials {
            login: username.to_owned(),
            password: password.to_owned(),
        })
    }
}

impl RosterSource for SamClient<Authenticated> {
    fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
        SamClient::students(self)
    }
}

impl LessonsSource for SamClient<Authenticated> {
    fn student_lessons(&self, student_id: &str) -> anyhow::Result<StudentLessonsPage> {
        SamClient::student_lessons(self, student_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::{MsaLesson, MtdLesson};

    async fn logged_in_client(uri: &str) -> SamClient<Authenticated> {
        SamClient::new(uri)
            .expect("Client should be created")
            .login(&SamCredentials {
                login: "u".to_owned(),
                password: "p".to_owned(),
            })
            .expect("Login should succeed")
    }

    async fn mock_sam_server() -> wiremock::MockServer {
        let server = wiremock::MockServer::start().await;

        wiremock::Mock::given(wiremock::matchers::method("POST"))
            .and(wiremock::matchers::path("/autenticar"))
            .respond_with(wiremock::ResponseTemplate::new(303))
            .mount(&server)
            .await;
        wiremock::Mock::given(wiremock::matchers::method("GET"))
            .and(wiremock::matchers::path("/painel"))
            .respond_with(wiremock::ResponseTemplate::new(200))
            .mount(&server)
            .await;

        server
    }

    fn lessons_page() -> String {
        format!(
            "<html><body>{}{}</body></html>",
            r#"<div id="msa"><table id="datatable1"><tbody><tr id="msa_559783"><td>09/09/2025</td><td>4.5 - 4.5</td><td>38 - 38</td><td>7 - 8</td><td>Sol</td><td>obs</td><td>MARCOS ROGÉRIO COSME</td></tr></tbody></table></div>"#,
            r#"<table id="datatable3"><tbody><tr id="mtd_214020"><td>00</td><td>00</td><td>MÉTODO CCB - SCHIMOLL - VIOLINO</td><td>04/12/2023</td><td>MURILO FAGNER CARDOSO</td><td>04/12/2023 21:17:17</td><td>Postura do violino</td></tr></tbody></table>"#
        )
    }

    #[test]
    fn given_unauthenticated_client_auth_source_should_login_and_return_authenticated() {
        smol::block_on(async {
            let mock_server = wiremock::MockServer::start().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/autenticar"))
                .respond_with(wiremock::ResponseTemplate::new(303))
                .mount(&mock_server)
                .await;

            let client = SamClient::<Unauthenticated>::new(mock_server.uri())
                .expect("Client should be created");

            let authenticated =
                AuthSource::login(&client, "u", "p").expect("Login should succeed");

            let _: SamClient<Authenticated> = authenticated;
        });
    }

    #[test]
    fn given_real_sam_client_roster_source_should_delegate_to_client() {
        smol::block_on(async {
            let mock_server = mock_sam_server().await;

            wiremock::Mock::given(wiremock::matchers::method("POST"))
                .and(wiremock::matchers::path("/alunos/listagem"))
                .respond_with(wiremock::ResponseTemplate::new(200).set_body_string(
                    r#"{"draw":"1","recordsTotal":1,"recordsFiltered":1,"data":[["99998","PEDRO ÁLVARES CABRAL","JARDIM <span class='m-r-10'></span> | BR-SP-ARARAQUARA-SÃO CARLOS","MÚSICO","VIOLINO","CANDIDATO(A)"]]}"#,
                ))
                .mount(&mock_server)
                .await;

            let client = logged_in_client(&mock_server.uri()).await;
            let students: Vec<SamStudent> =
                RosterSource::students(&client).expect("Delegation should succeed");

            assert_eq!(students.len(), 1);
            assert_eq!(students[0].id, "99998");
        });
    }

    #[test]
    fn given_real_sam_client_lessons_source_should_delegate_to_client() {
        smol::block_on(async {
            let mock_server = mock_sam_server().await;

            wiremock::Mock::given(wiremock::matchers::method("GET"))
                .and(wiremock::matchers::path("/licoes/index/500132"))
                .respond_with(
                    wiremock::ResponseTemplate::new(200).set_body_string(lessons_page()),
                )
                .mount(&mock_server)
                .await;

            let client = logged_in_client(&mock_server.uri()).await;
            let page: StudentLessonsPage =
                LessonsSource::student_lessons(&client, "500132")
                    .expect("Delegation should succeed");

            assert_eq!(page.msa.len(), 1);
            assert_eq!(page.msa[0].id.as_deref(), Some("559783"));
            assert_eq!(page.method.len(), 1);
            assert_eq!(page.method[0].id.as_deref(), Some("214020"));
        });
    }

    #[test]
    fn dto_defaults_should_exist_for_stubs() {
        let _: Vec<MsaLesson> = Vec::new();
        let _: Vec<MtdLesson> = Vec::new();
    }
}