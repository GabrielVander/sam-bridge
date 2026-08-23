use crate::view_models::StudentListItem;
use student_management::api::application::StudentsRetrievalGateway;

pub async fn load(gateway: &(dyn StudentsRetrievalGateway + Send + Sync)) -> anyhow::Result<Vec<StudentListItem>> {
    let students = gateway.get_avaliable_records().await?;
    Ok(students.iter().map(StudentListItem::from).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use student_management::api::domain::{MusicianLevel, Region, Student, StudentPosition};

    struct FakeRosterGateway {
        students: Vec<Student>,
        fail: bool,
    }

    #[async_trait]
    impl StudentsRetrievalGateway for FakeRosterGateway {
        async fn get_avaliable_records(&self) -> anyhow::Result<Vec<Student>> {
            if self.fail {
                anyhow::bail!("Session invalid or expired");
            }
            Ok(self.students.clone())
        }
    }

    fn student(id: &str) -> Student {
        Student {
            id: id.to_owned(),
            name: format!("ALUNO {id}"),
            position: StudentPosition::Musician {
                level: MusicianLevel::Candidate,
            },
            location: "BAIRRO".to_owned(),
            region: Region::AraraquaraSaoCarlos,
        }
    }

    #[test]
    fn given_students_should_map_to_list_items() {
        smol::block_on(async {
            let gateway = FakeRosterGateway {
                students: vec![student("1"), student("2")],
                fail: false,
            };

            let items = load(&gateway).await.expect("Should load");

            assert_eq!(items.len(), 2);
            assert_eq!(items[0].id, "1");
            assert_eq!(items[0].name, "ALUNO 1");
            assert_eq!(items[0].position, "Músico · Candidato(a)");
            assert_eq!(items[0].location, "BAIRRO");
        });
    }

    #[test]
    fn given_empty_roster_should_return_empty() {
        smol::block_on(async {
            let gateway = FakeRosterGateway {
                students: vec![],
                fail: false,
            };

            let items = load(&gateway).await.expect("Should load");

            assert!(items.is_empty());
        });
    }

    #[test]
    fn given_gateway_failure_should_propagate_error() {
        smol::block_on(async {
            let gateway = FakeRosterGateway {
                students: vec![],
                fail: true,
            };

            let result = load(&gateway).await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Session"));
        });
    }
}
