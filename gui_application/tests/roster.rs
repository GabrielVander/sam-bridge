use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use gui_application::api::roster::{
    RetrieveAllAvailableStudentsOutcomeDto, StudentPositionDto, StudentSummaryDto,
};
use pretty_assertions::assert_eq;
use student::application::gateways::{FailureKind, StudentGatewayError};
use student::domain::entities::{
    Instrument, MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentId,
    StudentPosition,
};

mod support;
use support::FakeSam;

fn student(position: StudentPosition) -> Student {
    Student {
        id: StudentId::new("1".to_owned()),
        name: "Someone".to_owned(),
        position,
        location: "Somewhere".to_owned(),
        region: Region::AraraquaraSaoCarlos,
    }
}

const fn musician(level: MusicianLevel) -> StudentPosition {
    StudentPosition::Musician {
        level,
        instrument: None,
        instrument_name: None,
    }
}

fn summaries_of(positions: Vec<StudentPosition>) -> RetrieveAllAvailableStudentsOutcomeDto {
    FakeSam::default()
        .listing(Ok(positions.into_iter().map(student).collect()))
        .build()
        .retrieve_all_available_students()
}

fn listed_positions(positions: Vec<StudentPosition>) -> Option<Vec<StudentPositionDto>> {
    match summaries_of(positions) {
        RetrieveAllAvailableStudentsOutcomeDto::Success { students } => Some(
            students
                .into_iter()
                .map(|summary| summary.position)
                .collect(),
        ),
        RetrieveAllAvailableStudentsOutcomeDto::Failure { .. } => None,
    }
}

#[test]
fn available_students_are_summarized_with_the_instrument_name_of_musicians() {
    let result = summaries_of(vec![StudentPosition::Musician {
        level: MusicianLevel::YouthService,
        instrument: Some(Instrument::Saxophone),
        instrument_name: Some("SAXOFONE TENOR".to_owned()),
    }]);

    assert_eq!(
        result,
        RetrieveAllAvailableStudentsOutcomeDto::Success {
            students: vec![StudentSummaryDto {
                id: "1".to_owned(),
                name: "Someone".to_owned(),
                position: StudentPositionDto::YouthService,
                location: "Somewhere".to_owned(),
                instrument_name: Some("SAXOFONE TENOR".to_owned()),
            }]
        }
    );
}

#[test]
fn only_musicians_have_an_instrument_name() {
    let result = summaries_of(vec![
        StudentPosition::Organist {
            level: OrganistLevel::Practice,
        },
        StudentPosition::Secretary {
            r#type: SecretaryType::Gem,
        },
        StudentPosition::Unknown("Avocado".to_owned()),
    ]);

    let RetrieveAllAvailableStudentsOutcomeDto::Success { students } = result else {
        panic!("the students should be listed, got {result:?}");
    };
    assert!(students.iter().all(|s| s.instrument_name.is_none()));
}

#[test]
fn every_musician_level_is_a_position() {
    let positions = listed_positions(vec![
        musician(MusicianLevel::Candidate),
        musician(MusicianLevel::Practice),
        musician(MusicianLevel::YouthService),
        musician(MusicianLevel::OfficialService),
        musician(MusicianLevel::Officialized),
        musician(MusicianLevel::Unknown("Strawberry".to_owned())),
    ]);

    assert_eq!(
        positions,
        Some(vec![
            StudentPositionDto::Candidate,
            StudentPositionDto::Practice,
            StudentPositionDto::YouthService,
            StudentPositionDto::OfficialService,
            StudentPositionDto::Officialized,
            StudentPositionDto::Invalid {
                raw: "Strawberry".to_owned()
            },
        ])
    );
}

#[test]
fn every_organist_level_is_a_position() {
    let levels = [
        OrganistLevel::Candidate,
        OrganistLevel::Practice,
        OrganistLevel::YouthService,
        OrganistLevel::HalfHour,
        OrganistLevel::OfficialService,
        OrganistLevel::YouthServiceHalfHour,
        OrganistLevel::YouthServicePractice,
        OrganistLevel::YouthServiceOfficialService,
        OrganistLevel::YouthServiceOfficialized,
        OrganistLevel::Unknown("Peanuts".to_owned()),
    ];

    let positions = listed_positions(
        levels
            .into_iter()
            .map(|level| StudentPosition::Organist { level })
            .collect(),
    );

    assert_eq!(
        positions,
        Some(vec![
            StudentPositionDto::Candidate,
            StudentPositionDto::Practice,
            StudentPositionDto::YouthService,
            StudentPositionDto::HalfHour,
            StudentPositionDto::OfficialService,
            StudentPositionDto::YouthServiceHalfHour,
            StudentPositionDto::YouthServicePractice,
            StudentPositionDto::YouthServiceOfficialService,
            StudentPositionDto::YouthServiceOfficialized,
            StudentPositionDto::Invalid {
                raw: "Peanuts".to_owned()
            },
        ])
    );
}

#[test]
fn every_secretary_type_is_a_position() {
    let positions = listed_positions(vec![
        StudentPosition::Secretary {
            r#type: SecretaryType::Gem,
        },
        StudentPosition::Secretary {
            r#type: SecretaryType::Music,
        },
    ]);

    assert_eq!(
        positions,
        Some(vec![
            StudentPositionDto::GemSecretary,
            StudentPositionDto::MusicSecretary,
        ])
    );
}

#[test]
fn an_unknown_position_keeps_what_sam_wrote() {
    let positions = listed_positions(vec![StudentPosition::Unknown("Avocado".to_owned())]);

    assert_eq!(
        positions,
        Some(vec![StudentPositionDto::Invalid {
            raw: "Avocado".to_owned()
        }])
    );
}

#[test]
fn every_kind_of_listing_failure_is_reported_with_its_kind_and_details() {
    for (kind, expected) in [
        (FailureKind::Transient, ErrorKindDto::Network),
        (FailureKind::Unexpected, ErrorKindDto::UnexpectedResponse),
        (FailureKind::SessionExpired, ErrorKindDto::SessionExpired),
        (FailureKind::Unclassified, ErrorKindDto::Unknown),
    ] {
        let facade = FakeSam::default()
            .listing(Err(StudentGatewayError::UnableToPerformOperation {
                kind,
                details: "Unable to decode student listing JSON response".to_owned(),
            }))
            .build();

        let result = facade.retrieve_all_available_students();

        assert_eq!(
            result,
            RetrieveAllAvailableStudentsOutcomeDto::Failure {
                report: ErrorReportDto {
                    kind: expected,
                    details: "Unable to decode student listing JSON response".to_owned(),
                }
            }
        );
    }
}
