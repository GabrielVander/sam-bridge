use std::sync::Arc;

use pretty_assertions::assert_eq;
use student::application::gateways::{
    FailureKind, MusicianProfileGatewayError, StudentLessonsGatewayError,
};
use student::application::use_cases::{AssessStudentProgressError, AssessStudentProgressUseCase};
use student::domain::entities::{
    AssessError, CheckpointStatus, Instrument, MusicianLevel, MusicianProfile, ProgressAssessment,
    StudentId, StudentLessons,
};

#[path = "support/helpers.rs"]
mod support;
use support::{
    FakeMusicianProfileGateway, FakeStudentLessonsGateway, checkpoint, method_lesson, msa_lesson,
};

#[test]
fn assesses_progress_from_the_gateways_profile_and_lessons() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Ok(musician(MusicianLevel::Candidate, Some(Instrument::Violin))),
        Ok(StudentLessons {
            msa: vec![msa_lesson("12", "12")],
            method: vec![method_lesson("46", "113")],
        }),
    );

    let assessment: ProgressAssessment = use_case
        .execute(&StudentId::new("500132".to_owned()))
        .expect("should succeed");

    let youth_service: &CheckpointStatus =
        checkpoint(&assessment, &MusicianLevel::YouthService).expect("youth service checkpoint");

    assert!(youth_service.ready_to_advance);
}

#[test]
fn propagates_profile_gateway_errors() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Err(MusicianProfileGatewayError::NotFound),
        Ok(StudentLessons::default()),
    );

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(
        result,
        Err(AssessStudentProgressError::Profile(
            MusicianProfileGatewayError::NotFound
        ))
    );
}

#[test]
fn propagates_profile_gateway_failures_with_their_kind_and_details() {
    let failure: MusicianProfileGatewayError =
        MusicianProfileGatewayError::UnableToPerformOperation {
            kind: FailureKind::SessionExpired,
            details: "Session expired".to_owned(),
        };

    let use_case: AssessStudentProgressUseCase =
        use_case(Err(failure.clone()), Ok(StudentLessons::default()));

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(result, Err(AssessStudentProgressError::Profile(failure)));
}

#[test]
fn student_with_no_instrument_assigned_is_reported_without_fetching_lessons() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Ok(musician(MusicianLevel::Candidate, None)),
        Err(lessons_failure()),
    );

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(
        result,
        Err(AssessStudentProgressError::NoInstrumentAssigned)
    );
}

#[test]
fn propagates_lessons_gateway_errors() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Ok(musician(MusicianLevel::Candidate, Some(Instrument::Violin))),
        Err(lessons_failure()),
    );

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(
        result,
        Err(AssessStudentProgressError::Lessons(lessons_failure()))
    );
}

#[test]
fn propagates_unknown_level_assessment_errors() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Ok(musician(
            MusicianLevel::Unknown("EXÓTICO".to_owned()),
            Some(Instrument::Violin),
        )),
        Ok(StudentLessons::default()),
    );

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(
        result,
        Err(AssessStudentProgressError::Assessment(
            AssessError::UnknownLevel("EXÓTICO".to_owned())
        ))
    );
}

#[test]
fn propagates_unpublished_requirements_assessment_errors() {
    let use_case: AssessStudentProgressUseCase = use_case(
        Ok(musician(
            MusicianLevel::Candidate,
            Some(Instrument::AltoClarinet),
        )),
        Ok(StudentLessons::default()),
    );

    let result: Result<ProgressAssessment, AssessStudentProgressError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(
        result,
        Err(AssessStudentProgressError::Assessment(
            AssessError::UnpublishedRequirements(Instrument::AltoClarinet)
        ))
    );
}

fn use_case(
    profile: Result<MusicianProfile, MusicianProfileGatewayError>,
    lessons: Result<StudentLessons, StudentLessonsGatewayError>,
) -> AssessStudentProgressUseCase {
    AssessStudentProgressUseCase::new(
        Arc::new(FakeMusicianProfileGateway::returning(profile)),
        Arc::new(FakeStudentLessonsGateway::returning(lessons)),
    )
}

const fn musician(level: MusicianLevel, instrument: Option<Instrument>) -> MusicianProfile {
    MusicianProfile { level, instrument }
}

fn lessons_failure() -> StudentLessonsGatewayError {
    StudentLessonsGatewayError::UnableToPerformOperation {
        kind: FailureKind::Transient,
        details: "connection refused".to_owned(),
    }
}
