use crate::api::{application::StudentsRetrievalGateway, domain::Student};

pub struct RetrieveStudentsUseCase<'a, T: StudentsRetrievalGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentsRetrievalGateway> RetrieveStudentsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<Student>> {
        self.gateway.get_available_records().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::domain::{MusicianLevel, OrganistLevel, Region, StudentPosition};
    use async_trait::async_trait;

    struct FakeStudentsRetrievalGateway {
        students: Vec<Student>,
        fail: bool,
    }

    #[async_trait]
    impl StudentsRetrievalGateway for FakeStudentsRetrievalGateway {
        async fn get_available_records(&self) -> anyhow::Result<Vec<Student>> {
            if self.fail {
                anyhow::bail!("Session invalid or expired");
            }
            Ok(self.students.clone())
        }
    }

    fn student(id: &str, position: StudentPosition) -> Student {
        Student {
            id: id.to_owned(),
            name: format!("ALUNO {id}"),
            position,
            location: "BAIRRO".to_owned(),
            region: Region::AraraquaraSaoCarlos,
        }
    }

    #[test]
    fn returns_the_students_from_the_gateway() {
        smol::block_on(async {
            let gateway = FakeStudentsRetrievalGateway {
                students: vec![
                    student(
                        "1",
                        StudentPosition::Musician {
                            level: MusicianLevel::Candidate,
                        },
                    ),
                    student(
                        "2",
                        StudentPosition::Organist {
                            level: OrganistLevel::HafHour,
                        },
                    ),
                ],
                fail: false,
            };
            let use_case = RetrieveStudentsUseCase::new(&gateway);

            let students = use_case.execute().await.expect("should succeed");

            assert_eq!(students.len(), 2);
            assert_eq!(students[0].id, "1");
            assert_eq!(students[1].id, "2");
        });
    }

    #[test]
    fn propagates_gateway_errors() {
        smol::block_on(async {
            let gateway = FakeStudentsRetrievalGateway {
                students: vec![],
                fail: true,
            };
            let use_case = RetrieveStudentsUseCase::new(&gateway);

            let result = use_case.execute().await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Session"));
        });
    }
}
