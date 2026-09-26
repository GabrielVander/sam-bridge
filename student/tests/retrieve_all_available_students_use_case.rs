use std::sync::Arc;
use student::domain::entities::StudentId;
use student::{
    application::{
        gateways::{FailureKind, StudentGateway, StudentGatewayError},
        use_cases::RetrieveAllAvailableStudentsUseCase,
    },
    domain::entities::{
        Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
    },
};

use pretty_assertions::assert_eq;

struct FakeStudentGateway {
    result: Result<Vec<Student>, StudentGatewayError>,
}

impl FakeStudentGateway {
    const fn new(result: Result<Vec<Student>, StudentGatewayError>) -> Self {
        Self { result }
    }
}

impl StudentGateway for FakeStudentGateway {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        self.result.clone()
    }
}

#[test]
fn returns_every_available_student() {
    let students: [Student; 3] = [
        Student {
            id: StudentId::new("1".to_owned()),
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
            id: StudentId::new("2".to_owned()),
            name: "Student B".to_string(),
            position: StudentPosition::Secretary {
                r#type: SecretaryType::Music,
            },
            location: "Location B".to_string(),
            region: Region::AraraquaraSaoCarlos,
        },
        Student {
            id: StudentId::new("3".to_owned()),
            name: "Student C".to_string(),
            position: StudentPosition::Organist {
                level: OrganistLevel::YouthServiceHalfHour,
            },
            location: "Location A".to_string(),
            region: Region::AraraquaraSaoCarlos,
        },
    ];

    let gateway: FakeStudentGateway = FakeStudentGateway::new(Ok(students.to_vec()));

    let use_case: RetrieveAllAvailableStudentsUseCase =
        RetrieveAllAvailableStudentsUseCase::new(Arc::new(gateway));

    let result: Result<Vec<Student>, StudentGatewayError> = use_case.execute();

    assert_eq!(result, Ok(students.to_vec()));
}

#[test]
fn propagates_gateway_errors_with_their_kind_and_details() {
    let error: StudentGatewayError = StudentGatewayError::UnableToPerformOperation {
        kind: FailureKind::Unexpected,
        details: "missing table".to_owned(),
    };
    let gateway: FakeStudentGateway = FakeStudentGateway::new(Err(error.clone()));

    let use_case: RetrieveAllAvailableStudentsUseCase =
        RetrieveAllAvailableStudentsUseCase::new(Arc::new(gateway));

    let result: Result<Vec<Student>, StudentGatewayError> = use_case.execute();

    assert_eq!(result, Err(error));
}
