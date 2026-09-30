use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use gui_application::api::progress::{
    AssessStudentProgressOutcomeDto, CheckpointDto, CheckpointStatusDto, MusicianLevelDto,
    ProgressAssessmentDto, RequirementStatusDto,
};
use pretty_assertions::assert_eq;
use student::application::gateways::{
    FailureKind, MusicianProfileGatewayError, StudentLessonsGatewayError,
};
use student::domain::entities::{
    Instrument, Lesson, MethodBook, MusicianLevel, MusicianProfile, Range, StudentLessons,
};

#[path = "support/helpers.rs"]
mod support;
use support::FakeSam;

#[test]
fn an_assessment_carries_every_checkpoint_and_percentage() {
    let facade = FakeSam::default()
        .profiling(Ok(musician(
            MusicianLevel::Candidate,
            Some(Instrument::Violin),
        )))
        .teaching(Ok(StudentLessons {
            msa: vec![Lesson {
                phase: Some(Range::new("6".to_owned(), "6".to_owned())),
                ..Lesson::default()
            }],
            method: Vec::new(),
        }))
        .build();

    let result = facade.assess_student_progress("1".to_owned());

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::Success {
            assessment: ProgressAssessmentDto {
                checkpoints: vec![
                    checkpoint(
                        MusicianLevelDto::Candidate,
                        CheckpointStatusDto::Achieved,
                        true
                    ),
                    checkpoint(
                        MusicianLevelDto::Practice,
                        CheckpointStatusDto::Pending,
                        true
                    ),
                    checkpoint(
                        MusicianLevelDto::YouthService,
                        CheckpointStatusDto::Pending,
                        false
                    ),
                    checkpoint(
                        MusicianLevelDto::OfficialService,
                        CheckpointStatusDto::Pending,
                        false
                    ),
                    checkpoint(
                        MusicianLevelDto::Officialized,
                        CheckpointStatusDto::Pending,
                        false
                    ),
                ],
                msa_relative_percent: 50.0,
                method_relative_percent: 0.0,
                combined_percent: 25.0,
                overall_checkpoint_percent: 25.0,
                next_level: Some(MusicianLevelDto::YouthService),
            }
        }
    );
}

#[test]
fn an_officialized_musician_has_no_next_level() {
    let result = assessment_of(Ok(musician(
        MusicianLevel::Officialized,
        Some(Instrument::Violin),
    )));

    let AssessStudentProgressOutcomeDto::Success { assessment } = result else {
        panic!("the progress should be assessed, got {result:?}");
    };
    assert_eq!(assessment.next_level, None);
}

#[test]
fn a_level_whose_requirements_are_met_is_ready_for_the_exam() {
    let facade = FakeSam::default()
        .profiling(Ok(musician(
            MusicianLevel::Candidate,
            Some(Instrument::Violin),
        )))
        .teaching(Ok(StudentLessons {
            msa: vec![Lesson {
                phase: Some(Range::single("12".to_owned())),
                ..Lesson::default()
            }],
            method: vec![Lesson {
                page: Some(Range::single("35".to_owned())),
                method_books: vec![MethodBook::Laoureux1],
                ..Lesson::default()
            }],
        }))
        .build();

    let AssessStudentProgressOutcomeDto::Success { assessment } =
        facade.assess_student_progress("1".to_owned())
    else {
        panic!("the progress should be assessed");
    };
    let youth_service: Option<CheckpointStatusDto> = assessment
        .checkpoints
        .into_iter()
        .find(|checkpoint| checkpoint.level == MusicianLevelDto::YouthService)
        .map(|checkpoint| checkpoint.status);
    assert_eq!(youth_service, Some(CheckpointStatusDto::ReadyForExam));
}

#[test]
fn rehearsing_at_the_youth_meeting_is_the_youth_meeting_on_the_progress_journey() {
    assert_eq!(
        MusicianLevelDto::from(MusicianLevel::YouthServicePractice),
        MusicianLevelDto::YouthService
    );
}

#[test]
fn a_student_without_an_instrument_is_reported_as_such() {
    let result = assessment_of(Ok(musician(MusicianLevel::Candidate, None)));

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::NoInstrumentAssigned
    );
}

#[test]
fn an_unknown_level_is_reported_with_what_sam_wrote() {
    let result = assessment_of(Ok(musician(
        MusicianLevel::Unknown("EXÓTICO".to_owned()),
        Some(Instrument::Violin),
    )));

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::UnknownLevel {
            raw_level: "EXÓTICO".to_owned()
        }
    );
}

#[test]
fn a_non_musician_is_reported_without_calling_it_a_failure() {
    let result = assessment_of(Err(MusicianProfileGatewayError::NotAMusician));

    assert_eq!(result, AssessStudentProgressOutcomeDto::NotAMusician);
}

#[test]
fn a_student_sam_does_not_list_is_an_unexpected_response() {
    let result = assessment_of(Err(MusicianProfileGatewayError::NotFound));

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::UnexpectedResponse,
                details: "no student found with the given id".to_owned(),
            }
        }
    );
}

#[test]
fn an_instrument_without_published_requirements_is_an_unknown_failure_with_the_reason() {
    let result = assessment_of(Ok(musician(
        MusicianLevel::Candidate,
        Some(Instrument::AltoClarinet),
    )));

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::Unknown,
                details: "no published test requirements for AltoClarinet".to_owned(),
            }
        }
    );
}

#[test]
fn a_profile_failure_is_reported_with_its_kind_and_details() {
    let result = assessment_of(Err(MusicianProfileGatewayError::UnableToPerformOperation {
        kind: FailureKind::SessionExpired,
        details: "Session expired".to_owned(),
    }));

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::SessionExpired,
                details: "Session expired".to_owned(),
            }
        }
    );
}

#[test]
fn a_lessons_failure_is_reported_with_its_kind_and_details() {
    let facade = FakeSam::default()
        .profiling(Ok(musician(
            MusicianLevel::Candidate,
            Some(Instrument::Violin),
        )))
        .teaching(Err(StudentLessonsGatewayError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        }))
        .build();

    let result = facade.assess_student_progress("1".to_owned());

    assert_eq!(
        result,
        AssessStudentProgressOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::Network,
                details: "connection refused".to_owned(),
            }
        }
    );
}

fn assessment_of(
    musician_profile: Result<MusicianProfile, MusicianProfileGatewayError>,
) -> AssessStudentProgressOutcomeDto {
    FakeSam::default()
        .profiling(musician_profile)
        .build()
        .assess_student_progress("1".to_owned())
}

const fn musician(level: MusicianLevel, instrument: Option<Instrument>) -> MusicianProfile {
    MusicianProfile { level, instrument }
}

const fn checkpoint(
    level: MusicianLevelDto,
    status: CheckpointStatusDto,
    requirement_met: bool,
) -> CheckpointDto {
    CheckpointDto {
        level,
        status,
        requirement: RequirementStatusDto {
            msa_met: requirement_met,
            method_met: requirement_met,
        },
    }
}
