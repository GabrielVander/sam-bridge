use crate::lessons::{
    application::gateways::{StudentLessonsGateway, StudentLessonsGatewayError},
    domain::entities::{AssessError, ProgressAssessment, assess},
};
use crate::shared::domain::entities::{Instrument, MusicianLevel};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssessStudentProgressError {
    #[error(transparent)]
    Gateway(#[from] StudentLessonsGatewayError),
    #[error(transparent)]
    Assessment(#[from] AssessError),
}

/// Fetches a student's recorded lessons and assesses their progress towards
/// the next test-eligibility checkpoint for their instrument.
///
/// The student's assigned level and instrument aren't sourced from a gateway
/// here: the roster domain doesn't track instrument at all yet, and SAM's
/// musician level vocabulary is only partially confirmed (see
/// `sam::roster::adapters::gateways::student_gateway_sam_impl::parse_musician_level`),
/// so for now the caller (who already has this from the student roster) is
/// expected to supply both.
pub struct AssessStudentProgressUseCase<'a, T: StudentLessonsGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentLessonsGateway> AssessStudentProgressUseCase<'a, T> {
    pub const fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(
        &self,
        student_id: &str,
        assigned_level: &MusicianLevel,
        instrument: Instrument,
    ) -> Result<ProgressAssessment, AssessStudentProgressError> {
        let lessons = self.gateway.get_all_for_student_with_id(student_id).await?;

        Ok(assess(
            assigned_level,
            instrument,
            &lessons.approved,
            &lessons.method,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::domain::entities::{Lesson, Range, StudentLessons};
    use async_trait::async_trait;

    struct FakeStudentLessonsGateway {
        bundle: StudentLessons,
        fail: bool,
    }

    #[async_trait]
    impl StudentLessonsGateway for FakeStudentLessonsGateway {
        async fn get_all_for_student_with_id(
            &self,
            _student_id: &str,
        ) -> Result<StudentLessons, StudentLessonsGatewayError> {
            if self.fail {
                return Err(StudentLessonsGatewayError::UnableToPerformOperation);
            }
            Ok(self.bundle.clone())
        }
    }

    fn ready_to_advance_bundle() -> StudentLessons {
        StudentLessons {
            approved: vec![Lesson {
                phase: Some(Range::new("12".to_owned(), "12".to_owned())),
                ..Default::default()
            }],
            method: vec![Lesson {
                page: Some(Range::new("46".to_owned(), "46".to_owned())),
                lesson: Some(Range::new("113".to_owned(), "113".to_owned())),
                ..Default::default()
            }],
        }
    }

    #[test]
    fn assesses_progress_from_the_gateways_lessons() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: ready_to_advance_bundle(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&gateway);

            let report = use_case
                .execute("500132", &MusicianLevel::Candidate, Instrument::Violin)
                .await
                .expect("should succeed");

            let youth_service = report
                .checkpoints
                .iter()
                .find(|c| c.level == MusicianLevel::YouthService)
                .expect("youth service checkpoint exists");
            assert!(youth_service.ready_to_advance);
        });
    }

    #[test]
    fn propagates_gateway_errors_from_assessment() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };
            let use_case = AssessStudentProgressUseCase::new(&gateway);

            let result = use_case
                .execute("500132", &MusicianLevel::Candidate, Instrument::Violin)
                .await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Gateway(
                    StudentLessonsGatewayError::UnableToPerformOperation
                ))
            );
        });
    }

    #[test]
    fn propagates_unknown_level_assessment_errors() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&gateway);

            let result = use_case
                .execute(
                    "500132",
                    &MusicianLevel::Unknown("EXÓTICO".to_owned()),
                    Instrument::Violin,
                )
                .await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Assessment(
                    AssessError::UnknownLevel("EXÓTICO".to_owned())
                ))
            );
        });
    }

    #[test]
    fn propagates_unpublished_requirements_assessment_errors() {
        smol::block_on(async {
            let gateway = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&gateway);

            let result = use_case
                .execute(
                    "500132",
                    &MusicianLevel::Candidate,
                    Instrument::AltoClarinet,
                )
                .await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Assessment(
                    AssessError::UnpublishedRequirements(Instrument::AltoClarinet)
                ))
            );
        });
    }
}
