use async_trait::async_trait;
use student_management::api::{
    application::StudentsRetrievalGateway,
    domain::Student,
};

use crate::{mapping::roster as roster_mapping, ports::RosterSource};

#[derive(Clone)]
pub struct SamRosterGateway<S: RosterSource> {
    source: S,
}

impl<S: RosterSource> SamRosterGateway<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

#[async_trait]
impl<S> StudentsRetrievalGateway for SamRosterGateway<S>
where
    S: RosterSource + Clone + Send + Sync + 'static,
{
    async fn get_avaliable_records(&self) -> anyhow::Result<Vec<Student>> {
        let source = self.source.clone();
        // sam is blocking — offload to thread pool so we don't block the async executor
        let dtos = smol::unblock(move || source.students()).await?;
        dtos.iter().map(roster_mapping::map).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sam::client::SamStudent;
    use student_management::api::{
        application::StudentsRetrievalGateway,
        domain::{MusicianLevel, OrganistLevel, Region, StudentPosition},
    };

    use crate::ports::RosterSource;

    #[derive(Clone)]
    struct StubRosterSource {
        students: Vec<SamStudent>,
    }

    impl RosterSource for StubRosterSource {
        fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
            Ok(self.students.clone())
        }
    }

    #[derive(Clone)]
    struct FailingRosterSource;

    impl RosterSource for FailingRosterSource {
        fn students(&self) -> anyhow::Result<Vec<SamStudent>> {
            anyhow::bail!("Students HTTP request failed")
        }
    }

    fn student_dto(id: &str, name: &str, role: &str, level: &str) -> SamStudent {
        SamStudent {
            id: id.to_owned(),
            name: name.to_owned(),
            location: "BAIRRO <span></span> | BR-SP-ARARAQUARA-SÃO CARLOS".to_owned(),
            role: role.to_owned(),
            instrument: "VIOLINO".to_owned(),
            level: level.to_owned(),
        }
    }

    #[test]
    fn given_roster_gateway_should_map_dtos_to_domain_students() {
        smol::block_on(async {
            let stub = StubRosterSource {
                students: vec![
                    student_dto("1", "ALUNA A", "MÚSICO", "CANDIDATO(A)"),
                    student_dto("2", "ALUNO B", "ORGANISTA", "MEIA HORA"),
                ],
            };
            let gateway = SamRosterGateway::new(stub);

            let students = gateway.get_avaliable_records().await.expect("Should succeed");

            assert_eq!(students.len(), 2);
            assert_eq!(students[0].id, "1");
            assert_eq!(
                students[0].position,
                StudentPosition::Musician {
                    level: MusicianLevel::Candidate
                }
            );
            assert_eq!(students[0].region, Region::AraraquaraSaoCarlos);
            assert_eq!(
                students[1].position,
                StudentPosition::Organist {
                    level: OrganistLevel::HafHour
                }
            );
        });
    }

    #[test]
    fn given_roster_gateway_with_empty_source_should_return_empty() {
        smol::block_on(async {
            let gateway = SamRosterGateway::new(StubRosterSource { students: vec![] });

            let students = gateway.get_avaliable_records().await.expect("Should succeed");

            assert!(students.is_empty());
        });
    }

    #[test]
    fn given_roster_gateway_when_source_fails_should_propagate_error() {
        smol::block_on(async {
            let gateway = SamRosterGateway::new(FailingRosterSource);

            let result = gateway.get_avaliable_records().await;

            assert!(result.is_err());
            assert!(result
                .unwrap_err()
                .to_string()
                .contains("Students HTTP request failed"));
        });
    }

    #[test]
    fn given_unmappable_student_should_still_be_mapped_tolerantly() {
        smol::block_on(async {
            let mut nameless = student_dto("3", "ALUNA C", "MÚSICO", "CANDIDATO(A)");
            nameless.name = " ".to_owned();

            let gateway = SamRosterGateway::new(StubRosterSource {
                students: vec![nameless],
            });

            let students = gateway.get_avaliable_records().await.expect("Tolerance keeps rows");

            assert_eq!(students.len(), 1);
            assert_eq!(students[0].name, "", "Absent name flows through as empty");
        });
    }
}
