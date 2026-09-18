use std::sync::Arc;

use async_trait::async_trait;
use student::{
    application::{
        dto::{StudentPositionDto, StudentSummaryDto},
        gateways::{StudentGateway, StudentGatewayError},
        use_cases::{RetrieveAllAvailableStudentsResult, RetrieveAllAvailableStudentsUseCase},
    },
    domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

use pretty_assertions::assert_eq;

#[test]
fn assert_students_map() {
    smol::block_on(async {
        let students: [Student; 3] = [
            Student {
                id: "1".to_string(),
                name: "Student A".to_string(),
                position: StudentPosition::Musician {
                    level: MusicianLevel::Candidate,
                    instrument: Some(Instrument::Violin),
                    instrument_name: Some("VIOLINO".to_string()),
                },
                location: "Location A".to_string(),
                region: Region::AraraquaraSaoCarlos,
            },
            Student {
                id: "2".to_string(),
                name: "Student B".to_string(),
                position: StudentPosition::Secretary {
                    r#type: SecretaryType::Music,
                },
                location: "Location B".to_string(),
                region: Region::AraraquaraSaoCarlos,
            },
            Student {
                id: "3".to_string(),
                name: "Student C".to_string(),
                position: StudentPosition::Organist {
                    level: OrganistLevel::YouthServiceHalfHour,
                },
                location: "Location A".to_string(),
                region: Region::AraraquaraSaoCarlos,
            },
        ];

        let expected: RetrieveAllAvailableStudentsResult =
            RetrieveAllAvailableStudentsResult::Success(
                [
                    StudentSummaryDto {
                        id: "1".to_string(),
                        name: "Student A".to_string(),
                        position: StudentPositionDto::Candidate,
                        location: "Location A".to_string(),
                        instrument_name: Some("VIOLINO".to_string()),
                    },
                    StudentSummaryDto {
                        id: "2".to_string(),
                        name: "Student B".to_string(),
                        position: StudentPositionDto::MusicSecretary,
                        location: "Location B".to_owned(),
                        instrument_name: None,
                    },
                    StudentSummaryDto {
                        id: "3".to_string(),
                        name: "Student C".to_string(),
                        position: StudentPositionDto::YouthServiceHalfHour,
                        location: "Location A".to_string(),
                        instrument_name: None,
                    },
                ]
                .to_vec(),
            );

        let gateway: FakeSuccessGateway = FakeSuccessGateway {
            students: students.to_vec(),
        };

        let use_case: RetrieveAllAvailableStudentsUseCase =
            RetrieveAllAvailableStudentsUseCase::new(Arc::new(gateway));

        let result: RetrieveAllAvailableStudentsResult = use_case.execute().await;

        assert_eq!(result, expected);
    });
}

#[test]
fn propagates_gateway_errors() {
    smol::block_on(async {
        let gateway: FakeFailureGateway = FakeFailureGateway;

        let use_case: RetrieveAllAvailableStudentsUseCase =
            RetrieveAllAvailableStudentsUseCase::new(Arc::new(gateway));

        let result: RetrieveAllAvailableStudentsResult = use_case.execute().await;

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsResult::Failure(
                StudentGatewayError::UnableToPerformOperation
            )
        );
    });
}

struct FakeSuccessGateway {
    students: Vec<Student>,
}

#[async_trait]
impl StudentGateway for FakeSuccessGateway {
    async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        Ok(self.students.clone())
    }
}

struct FakeFailureGateway;

#[async_trait]
impl StudentGateway for FakeFailureGateway {
    async fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        Err(StudentGatewayError::UnableToPerformOperation)
    }
}
