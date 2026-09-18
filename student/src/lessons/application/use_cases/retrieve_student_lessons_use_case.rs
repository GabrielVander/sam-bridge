use crate::lessons::{
    application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError},
    domain::entities::StudentLessons,
};

pub struct RetrieveStudentLessonsUseCase<'a, T: StudentLessonsGateway + ?Sized> {
    gateway: &'a T,
}

impl<'a, T: StudentLessonsGateway + ?Sized> RetrieveStudentLessonsUseCase<'a, T> {
    pub const fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub fn execute(&self, student_id: &str) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.gateway.get_all_for_student_with_id(student_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::lessons::domain::entities::{Clef, Lesson, Range};

    use super::*;
    use chrono::NaiveDate;

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
                return Err(StudentLessonsGatewayError::UnableToPerformOperation);
            }
            Ok(self.bundle.clone())
        }
    }

    fn fixture_bundle() -> StudentLessons {
        StudentLessons {
            approved: vec![Lesson {
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
        let use_case = RetrieveStudentLessonsUseCase::new(&gateway);

        let result = use_case.execute("500132").expect("should succeed");

        assert_eq!(result, bundle);
    }

    #[test]
    fn propagates_gateway_errors() {
        let gateway = FakeStudentLessonsGateway {
            bundle: StudentLessons::default(),
            fail: true,
        };
        let use_case = RetrieveStudentLessonsUseCase::new(&gateway);

        let result = use_case.execute("500132");

        assert_eq!(
            result,
            Err(StudentLessonsGatewayError::UnableToPerformOperation)
        );
    }
}
