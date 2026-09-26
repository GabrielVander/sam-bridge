use std::sync::Arc;

use chrono::NaiveDate;
use pretty_assertions::assert_eq;
use student::application::gateways::{FailureKind, StudentLessonsGatewayError};
use student::application::use_cases::RetrieveStudentLessonsUseCase;
use student::domain::entities::{Clef, Lesson, Range, StudentId, StudentLessons};

mod support;
use support::FakeStudentLessonsGateway;

#[test]
fn returns_the_lessons_from_the_gateway() {
    let lessons: StudentLessons = StudentLessons {
        msa: vec![Lesson {
            id: Some("559783".to_owned()),
            date: NaiveDate::from_ymd_opt(2025, 9, 9),
            phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
            page: None,
            lesson: None,
            clef: Some(Clef::G),
            description: None,
            instructor: Some("MARCOS ROGÉRIO COSME".to_owned()),
            method: None,
        }],
        method: vec![Lesson::default()],
    };

    let use_case: RetrieveStudentLessonsUseCase = RetrieveStudentLessonsUseCase::new(Arc::new(
        FakeStudentLessonsGateway::returning(Ok(lessons.clone())),
    ));

    let result: Result<StudentLessons, StudentLessonsGatewayError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(result, Ok(lessons));
}

#[test]
fn propagates_gateway_errors_with_their_kind_and_details() {
    let failure: StudentLessonsGatewayError =
        StudentLessonsGatewayError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        };

    let use_case: RetrieveStudentLessonsUseCase = RetrieveStudentLessonsUseCase::new(Arc::new(
        FakeStudentLessonsGateway::returning(Err(failure.clone())),
    ));

    let result: Result<StudentLessons, StudentLessonsGatewayError> =
        use_case.execute(&StudentId::new("500132".to_owned()));

    assert_eq!(result, Err(failure));
}
