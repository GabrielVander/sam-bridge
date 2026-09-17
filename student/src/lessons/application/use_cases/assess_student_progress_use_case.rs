use crate::lessons::{
    application::gateways::{
        MusicianProfileGateway, MusicianProfileGatewayError, StudentLessonsGateway,
        StudentLessonsGatewayError,
    },
    domain::entities::{AssessError, ProgressAssessment, assess},
};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AssessStudentProgressError {
    #[error(transparent)]
    Profile(#[from] MusicianProfileGatewayError),
    #[error(transparent)]
    Lessons(#[from] StudentLessonsGatewayError),
    #[error("student has no instrument assigned yet")]
    NoInstrumentAssigned,
    #[error(transparent)]
    Assessment(#[from] AssessError),
}

pub struct AssessStudentProgressUseCase<
    'a,
    P: MusicianProfileGateway + ?Sized,
    L: StudentLessonsGateway + ?Sized,
> {
    profile_gateway: &'a P,
    lessons_gateway: &'a L,
}

impl<'a, P: MusicianProfileGateway + ?Sized, L: StudentLessonsGateway + ?Sized>
    AssessStudentProgressUseCase<'a, P, L>
{
    pub const fn new(profile_gateway: &'a P, lessons_gateway: &'a L) -> Self {
        Self {
            profile_gateway,
            lessons_gateway,
        }
    }

    pub async fn execute(
        &self,
        student_id: &str,
    ) -> Result<ProgressAssessment, AssessStudentProgressError> {
        let profile = self.profile_gateway.get_by_id(student_id).await?;
        let Some(instrument) = profile.instrument else {
            return Err(AssessStudentProgressError::NoInstrumentAssigned);
        };

        let lessons = self
            .lessons_gateway
            .get_all_for_student_with_id(student_id)
            .await?;

        Ok(assess(
            &profile.level,
            instrument,
            &lessons.approved,
            &lessons.method,
        )?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lessons::domain::entities::{Lesson, MusicianProfile, Range, StudentLessons};
    use crate::shared::domain::entities::{Instrument, MusicianLevel};
    use async_trait::async_trait;

    struct FakeMusicianProfileGateway {
        profile: Result<MusicianProfile, MusicianProfileGatewayError>,
    }

    #[async_trait]
    impl MusicianProfileGateway for FakeMusicianProfileGateway {
        async fn get_by_id(
            &self,
            _id: &str,
        ) -> Result<MusicianProfile, MusicianProfileGatewayError> {
            self.profile.clone()
        }
    }

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

    fn profile_gateway(
        level: MusicianLevel,
        instrument: Option<Instrument>,
    ) -> FakeMusicianProfileGateway {
        FakeMusicianProfileGateway {
            profile: Ok(MusicianProfile { level, instrument }),
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
    fn assesses_progress_from_the_gateways_profile_and_lessons() {
        smol::block_on(async {
            let profiles = profile_gateway(MusicianLevel::Candidate, Some(Instrument::Violin));
            let lessons = FakeStudentLessonsGateway {
                bundle: ready_to_advance_bundle(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let report = use_case.execute("500132").await.expect("should succeed");

            let youth_service = report
                .checkpoints
                .iter()
                .find(|c| c.level == MusicianLevel::YouthService)
                .expect("youth service checkpoint exists");
            assert!(youth_service.ready_to_advance);
        });
    }

    #[test]
    fn propagates_profile_gateway_errors() {
        smol::block_on(async {
            let profiles = FakeMusicianProfileGateway {
                profile: Err(MusicianProfileGatewayError::NotFound),
            };
            let lessons = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let result = use_case.execute("500132").await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Profile(
                    MusicianProfileGatewayError::NotFound
                ))
            );
        });
    }

    #[test]
    fn student_with_no_instrument_assigned_is_reported_without_fetching_lessons() {
        smol::block_on(async {
            let profiles = profile_gateway(MusicianLevel::Candidate, None);
            let lessons = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let result = use_case.execute("500132").await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::NoInstrumentAssigned)
            );
        });
    }

    #[test]
    fn propagates_lessons_gateway_errors() {
        smol::block_on(async {
            let profiles = profile_gateway(MusicianLevel::Candidate, Some(Instrument::Violin));
            let lessons = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: true,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let result = use_case.execute("500132").await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Lessons(
                    StudentLessonsGatewayError::UnableToPerformOperation
                ))
            );
        });
    }

    #[test]
    fn propagates_unknown_level_assessment_errors() {
        smol::block_on(async {
            let profiles = profile_gateway(
                MusicianLevel::Unknown("EXÓTICO".to_owned()),
                Some(Instrument::Violin),
            );
            let lessons = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let result = use_case.execute("500132").await;

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
            let profiles =
                profile_gateway(MusicianLevel::Candidate, Some(Instrument::AltoClarinet));
            let lessons = FakeStudentLessonsGateway {
                bundle: StudentLessons::default(),
                fail: false,
            };
            let use_case = AssessStudentProgressUseCase::new(&profiles, &lessons);

            let result = use_case.execute("500132").await;

            assert_eq!(
                result,
                Err(AssessStudentProgressError::Assessment(
                    AssessError::UnpublishedRequirements(Instrument::AltoClarinet)
                ))
            );
        });
    }
}
