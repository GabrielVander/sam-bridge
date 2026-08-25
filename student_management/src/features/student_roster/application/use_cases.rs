#![allow(non_snake_case)]
use std::sync::Arc;

use crate::features::student_roster::application::dto::StudentSummaryDto;
use crate::features::student_roster::application::gateways::StudentsRetrievalGateway;

pub struct RetrieveStudentsUseCase {
    gateway: Arc<dyn StudentsRetrievalGateway>,
}

impl RetrieveStudentsUseCase {
    pub fn new(gateway: Arc<dyn StudentsRetrievalGateway>) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<StudentSummaryDto>> {
        let students = self.gateway.get_available_records().await?;
        Ok(students.iter().map(mapStudent).collect())
    }
}

fn mapStudent(s: &crate::features::student_roster::domain::entities::Student) -> StudentSummaryDto {
    StudentSummaryDto {
        id: s.id.clone(),
        name: s.name.clone(),
        location: s.location.clone(),
        position: s.position.clone(),
        region: s.region.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::domain::{MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition};
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
                            level: OrganistLevel::HalfHour,
                        },
                    ),
                ],
                fail: false,
            };
            let use_case = RetrieveStudentsUseCase::new(Arc::new(gateway));

            let dtos = use_case.execute().await.expect("should succeed");

            assert_eq!(dtos.len(), 2);
            assert_eq!(dtos[0].id, "1");
            assert_eq!(dtos[1].id, "2");
            assert_eq!(dtos[0].position, StudentPosition::Musician { level: MusicianLevel::Candidate });
            assert_eq!(dtos[1].position, StudentPosition::Organist { level: OrganistLevel::HalfHour });
        });
    }

    #[test]
    fn propagates_gateway_errors() {
        smol::block_on(async {
            let gateway = FakeStudentsRetrievalGateway {
                students: vec![],
                fail: true,
            };
            let use_case = RetrieveStudentsUseCase::new(Arc::new(gateway));

            let result = use_case.execute().await;

            assert!(result.is_err());
            assert!(result.unwrap_err().to_string().contains("Session"));
        });
    }

    #[test]
    fn maps_all_position_and_region_variants() {
        smol::block_on(async {
            let variants = vec![
                Student {
                    id: "1".to_owned(),
                    name: "A".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::Candidate },
                    location: "X".to_owned(),
                    region: Region::AraraquaraItirapina,
                },
                Student {
                    id: "1b".to_owned(),
                    name: "A".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::Practice },
                    location: "X".to_owned(),
                    region: Region::AraraquaraItirapina,
                },
                Student {
                    id: "1c".to_owned(),
                    name: "A".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::YouthService },
                    location: "X".to_owned(),
                    region: Region::AraraquaraItirapina,
                },
                Student {
                    id: "1d".to_owned(),
                    name: "A".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::OfficialService },
                    location: "X".to_owned(),
                    region: Region::AraraquaraItirapina,
                },
                Student {
                    id: "1e".to_owned(),
                    name: "A".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::Officialized },
                    location: "X".to_owned(),
                    region: Region::AraraquaraItirapina,
                },
                Student {
                    id: "2".to_owned(),
                    name: "B".to_owned(),
                    position: StudentPosition::Musician { level: MusicianLevel::Unknown("X".to_owned()) },
                    location: "Y".to_owned(),
                    region: Region::Other("OUTRA".to_owned()),
                },
                Student {
                    id: "3".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::Candidate },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3b".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::Practice },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3c".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::YouthService },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3d".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::HalfHour },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3e".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::OfficialService },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3f".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::YouthServiceHalfHour },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3g".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::YouthServicePractice },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3h".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::YouthServiceOfficialService },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3i".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::YouthServiceOfficialized },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "3j".to_owned(),
                    name: "C".to_owned(),
                    position: StudentPosition::Organist { level: OrganistLevel::Unknown("X".to_owned()) },
                    location: "Z".to_owned(),
                    region: Region::Other("".to_owned()),
                },
                Student {
                    id: "4".to_owned(),
                    name: "D".to_owned(),
                    position: StudentPosition::Secretary { r#type: SecretaryType::Gem },
                    location: "".to_owned(),
                    region: Region::AraraquaraSaoCarlos,
                },
                Student {
                    id: "4b".to_owned(),
                    name: "D".to_owned(),
                    position: StudentPosition::Secretary { r#type: SecretaryType::Music },
                    location: "".to_owned(),
                    region: Region::AraraquaraSaoCarlos,
                },
                Student {
                    id: "5".to_owned(),
                    name: "E".to_owned(),
                    position: StudentPosition::Unknown("REGENTE".to_owned()),
                    location: "".to_owned(),
                    region: Region::Other("".to_owned()),
                },
            ];
            let gateway = FakeStudentsRetrievalGateway { students: variants, fail: false };
            let use_case = RetrieveStudentsUseCase::new(Arc::new(gateway));
            let dtos = use_case.execute().await.unwrap();
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::Candidate }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::Practice }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::YouthService }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::OfficialService }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::Officialized }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Musician { level: MusicianLevel::Unknown("X".to_owned()) }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Organist { level: OrganistLevel::Candidate }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Organist { level: OrganistLevel::HalfHour }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Secretary { r#type: SecretaryType::Gem }));
            assert!(dtos.iter().any(|d| d.position == StudentPosition::Unknown("REGENTE".to_owned())));
            assert!(dtos.iter().any(|d| d.region == Region::AraraquaraSaoCarlos));
            assert!(dtos.iter().any(|d| d.region == Region::AraraquaraItirapina));
        });
    }
}