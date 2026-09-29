use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use gui_application::api::roster::{
    InstrumentDto, RetrieveAllAvailableStudentsOutcomeDto, StudentPositionDto, StudentSummaryDto,
};
use pretty_assertions::assert_eq;
use student::application::gateways::{FailureKind, StudentGatewayError};
use student::domain::entities::{
    Instrument, MusicianLevel, OrganistLevel, Student, StudentId, StudentPosition,
};

#[path = "support/helpers.rs"]
mod support;
use support::FakeSam;

fn student(position: StudentPosition) -> Student {
    Student {
        id: StudentId::new("1".to_owned()),
        name: "Someone".to_owned(),
        position,
        location: "Somewhere".to_owned(),
    }
}

const fn musician(level: MusicianLevel) -> StudentPosition {
    StudentPosition::Musician {
        level,
        instrument: None,
    }
}

const fn musician_playing(instrument: Instrument) -> StudentPosition {
    StudentPosition::Musician {
        level: MusicianLevel::YouthService,
        instrument: Some(instrument),
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

fn listed_instruments(positions: Vec<StudentPosition>) -> Option<Vec<Option<InstrumentDto>>> {
    match summaries_of(positions) {
        RetrieveAllAvailableStudentsOutcomeDto::Success { students } => Some(
            students
                .into_iter()
                .map(|summary| summary.instrument)
                .collect(),
        ),
        RetrieveAllAvailableStudentsOutcomeDto::Failure { .. } => None,
    }
}

#[test]
fn available_students_are_summarized_with_the_instrument_of_musicians() {
    let result = summaries_of(vec![musician_playing(Instrument::TenorSaxophone)]);

    assert_eq!(
        result,
        RetrieveAllAvailableStudentsOutcomeDto::Success {
            students: vec![StudentSummaryDto {
                id: "1".to_owned(),
                name: "Someone".to_owned(),
                position: StudentPositionDto::YouthService,
                location: "Somewhere".to_owned(),
                instrument: Some(InstrumentDto::TenorSaxophone),
            }]
        }
    );
}

#[test]
fn every_instrument_crosses_the_bridge() {
    let instruments = [
        (Instrument::Violin, InstrumentDto::Violin),
        (Instrument::Viola, InstrumentDto::Viola),
        (Instrument::Cello, InstrumentDto::Cello),
        (Instrument::Flute, InstrumentDto::Flute),
        (Instrument::Oboe, InstrumentDto::Oboe),
        (Instrument::Bassoon, InstrumentDto::Bassoon),
        (Instrument::Clarinet, InstrumentDto::Clarinet),
        (Instrument::AltoClarinet, InstrumentDto::AltoClarinet),
        (Instrument::BassClarinet, InstrumentDto::BassClarinet),
        (Instrument::AltoSaxophone, InstrumentDto::AltoSaxophone),
        (
            Instrument::CurvedSopranoSaxophone,
            InstrumentDto::CurvedSopranoSaxophone,
        ),
        (
            Instrument::StraightSopranoSaxophone,
            InstrumentDto::StraightSopranoSaxophone,
        ),
        (Instrument::TenorSaxophone, InstrumentDto::TenorSaxophone),
        (Instrument::Trumpet, InstrumentDto::Trumpet),
        (Instrument::Cornet, InstrumentDto::Cornet),
        (Instrument::Flugelhorn, InstrumentDto::Flugelhorn),
        (Instrument::FrenchHorn, InstrumentDto::FrenchHorn),
        (Instrument::Trombone, InstrumentDto::Trombone),
        (Instrument::Euphonium, InstrumentDto::Euphonium),
        (Instrument::Tuba, InstrumentDto::Tuba),
        (Instrument::EnglishHorn, InstrumentDto::EnglishHorn),
        (Instrument::ContraltoViolin, InstrumentDto::ContraltoViolin),
        (
            Instrument::Unknown("BANDOLIM".to_owned()),
            InstrumentDto::Unknown {
                raw: "BANDOLIM".to_owned(),
            },
        ),
    ];

    let summarized = listed_instruments(
        instruments
            .iter()
            .map(|(instrument, _)| musician_playing(instrument.clone()))
            .collect(),
    );

    assert_eq!(
        summarized,
        Some(instruments.into_iter().map(|(_, dto)| Some(dto)).collect())
    );
}

#[test]
fn only_musicians_have_an_instrument() {
    let instruments = listed_instruments(vec![
        musician(MusicianLevel::Practice),
        StudentPosition::Organist {
            level: OrganistLevel::Practice,
        },
        StudentPosition::GemSecretary,
        StudentPosition::Unknown("Avocado".to_owned()),
    ]);

    assert_eq!(instruments, Some(vec![None, None, None, None]));
}

#[test]
fn every_musician_level_is_a_position() {
    let positions = listed_positions(vec![
        musician(MusicianLevel::Candidate),
        musician(MusicianLevel::Practice),
        musician(MusicianLevel::YouthService),
        musician(MusicianLevel::YouthServicePractice),
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
            StudentPositionDto::YouthServicePractice,
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
        OrganistLevel::OfficialService,
        OrganistLevel::YouthServiceHalfHour,
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
            StudentPositionDto::OfficialService,
            StudentPositionDto::YouthServiceHalfHour,
            StudentPositionDto::Invalid {
                raw: "Peanuts".to_owned()
            },
        ])
    );
}

#[test]
fn a_gem_secretary_is_a_position() {
    let positions = listed_positions(vec![StudentPosition::GemSecretary]);

    assert_eq!(positions, Some(vec![StudentPositionDto::GemSecretary]));
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
