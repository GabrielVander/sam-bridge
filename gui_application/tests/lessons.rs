use gui_application::api::error_report::{ErrorKindDto, ErrorReportDto};
use gui_application::api::lessons::{
    ClefDto, DateDto, LessonDto, RangeDto, RetrieveStudentLessonsOutcomeDto, StudentLessonsDto,
};
use pretty_assertions::assert_eq;
use student::application::gateways::{FailureKind, StudentLessonsGatewayError};
use student::domain::entities::{Clef, Lesson, Range, StudentLessons};

mod support;
use support::FakeSam;

fn lessons_of(lessons: StudentLessons) -> RetrieveStudentLessonsOutcomeDto {
    FakeSam::default()
        .teaching(Ok(lessons))
        .build()
        .retrieve_student_lessons("1".to_owned())
}

fn msa_lesson_with_clef(clef: Clef) -> Lesson {
    Lesson {
        clef: Some(clef),
        ..Lesson::default()
    }
}

#[test]
fn every_field_of_a_lesson_is_carried() {
    let result = lessons_of(StudentLessons {
        msa: vec![Lesson {
            id: Some("1".to_owned()),
            date: chrono::NaiveDate::from_ymd_opt(2025, 9, 9),
            phase: Some(Range::new("4.5".to_owned(), "4.5".to_owned())),
            page: Some(Range::new("30".to_owned(), "34".to_owned())),
            lesson: Some(Range::new("7".to_owned(), "8".to_owned())),
            clef: Some(Clef::G),
            description: Some("desc".to_owned()),
            instructor: Some("instructor".to_owned()),
            method: Some("method".to_owned()),
        }],
        method: vec![Lesson::default()],
    });

    assert_eq!(
        result,
        RetrieveStudentLessonsOutcomeDto::Success {
            lessons: StudentLessonsDto {
                msa: vec![LessonDto {
                    id: Some("1".to_owned()),
                    date: Some(DateDto {
                        year: 2025,
                        month: 9,
                        day: 9,
                    }),
                    phase: Some(RangeDto {
                        from: "4.5".to_owned(),
                        to: "4.5".to_owned(),
                    }),
                    page: Some(RangeDto {
                        from: "30".to_owned(),
                        to: "34".to_owned(),
                    }),
                    lesson: Some(RangeDto {
                        from: "7".to_owned(),
                        to: "8".to_owned(),
                    }),
                    clef: Some(ClefDto::G),
                    description: Some("desc".to_owned()),
                    instructor: Some("instructor".to_owned()),
                    method: Some("method".to_owned()),
                }],
                method: vec![LessonDto {
                    id: None,
                    date: None,
                    phase: None,
                    page: None,
                    lesson: None,
                    clef: None,
                    description: None,
                    instructor: None,
                    method: None,
                }],
            }
        }
    );
}

#[test]
fn every_clef_is_carried() {
    let result = lessons_of(StudentLessons {
        msa: vec![
            msa_lesson_with_clef(Clef::G),
            msa_lesson_with_clef(Clef::C),
            msa_lesson_with_clef(Clef::F),
        ],
        method: Vec::new(),
    });

    let RetrieveStudentLessonsOutcomeDto::Success { lessons } = result else {
        panic!("the lessons should be retrieved, got {result:?}");
    };
    let clefs: Vec<Option<ClefDto>> = lessons.msa.into_iter().map(|l| l.clef).collect();
    assert_eq!(
        clefs,
        vec![Some(ClefDto::G), Some(ClefDto::C), Some(ClefDto::F)]
    );
}

#[test]
fn a_lessons_failure_is_reported_with_its_kind_and_details() {
    let facade = FakeSam::default()
        .teaching(Err(StudentLessonsGatewayError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "Request failed for operation 'student_lessons'".to_owned(),
        }))
        .build();

    let result = facade.retrieve_student_lessons("1".to_owned());

    assert_eq!(
        result,
        RetrieveStudentLessonsOutcomeDto::Failure {
            report: ErrorReportDto {
                kind: ErrorKindDto::Network,
                details: "Request failed for operation 'student_lessons'".to_owned(),
            }
        }
    );
}
