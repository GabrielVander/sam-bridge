use std::sync::Arc;

use crate::lessons::{
    application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError},
    domain::entities::StudentLessons,
};

#[derive(Clone)]
pub struct RetrieveStudentLessonsUseCase {
    gateway: Arc<dyn StudentLessonsGateway + Send + Sync>,
}

impl RetrieveStudentLessonsUseCase {
    pub fn new(gateway: Arc<dyn StudentLessonsGateway + Send + Sync>) -> Self {
        Self { gateway }
    }

    pub fn execute(&self, student_id: &str) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.gateway.get_all_for_student_with_id(student_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::lessons::domain::entities::{Clef, Lesson, Range};
    use shared_kernel::failure_kind::FailureKind;

    use super::*;
    use chrono::NaiveDate;

    fn failure() -> StudentLessonsGatewayError {
        StudentLessonsGatewayError::UnableToPerformOperation {
            kind: FailureKind::Transient,
            details: "connection refused".to_owned(),
        }
    }

    struct FakeStudentLessonsGateway {
        bundle: StudentLessons,
        fail: bool,
    }
    impl StudentLessonsGateway for FakeStudentLessonsGateway {
        fn get_all_for_student_with_id(
            &self,
            _student_id: &str,
        ) -> Result<StudentLessons, StudentLessonsGatewayError> {
            if self.fail {
                return Err(failure());
            }
            Ok(self.bundle.clone())
        }
    }

    fn fixture_bundle() -> StudentLessons {
        StudentLessons {
            msa: vec![Lesson {
                id: Some("559783".to_owned()),
                date: Some(NaiveDate::from_ymd_opt(2025, 9, 9).expect("valid date")),
                phase: Some(Range {
                    from: "4.5".to_owned(),
                    to: "4.5".to_owned(),
                }),
                page: None,
                lesson: None,
                clef: Some(Clef::G),
                description: None,
                instructor: Some("MARCOS ROGÉRIO COSME".to_owned()),
                method: None,
            }],
            method: vec![Lesson::default()],
        }
    }

    #[test]
    fn returns_the_bundle_from_the_gateway() {
        let bundle = fixture_bundle();
        let gateway = FakeStudentLessonsGateway {
            bundle: bundle.clone(),
            fail: false,
        };
        let use_case = RetrieveStudentLessonsUseCase::new(Arc::new(gateway));

        let result = use_case.execute("500132").expect("should succeed");

        assert_eq!(result, bundle);
    }

    #[test]
    fn propagates_gateway_errors_with_their_kind_and_details() {
        let gateway = FakeStudentLessonsGateway {
            bundle: StudentLessons::default(),
            fail: true,
        };
        let use_case = RetrieveStudentLessonsUseCase::new(Arc::new(gateway));

        let result = use_case.execute("500132");

        assert_eq!(result, Err(failure()));
    }
}
