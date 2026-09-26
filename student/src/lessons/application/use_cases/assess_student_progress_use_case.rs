use std::sync::Arc;

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

#[derive(Clone)]
pub struct AssessStudentProgressUseCase {
    profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync>,
    lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync>,
}

impl AssessStudentProgressUseCase {
    pub fn new(
        profile_gateway: Arc<dyn MusicianProfileGateway + Send + Sync>,
        lessons_gateway: Arc<dyn StudentLessonsGateway + Send + Sync>,
    ) -> Self {
        Self {
            profile_gateway,
            lessons_gateway,
        }
    }

    pub fn execute(
        &self,
        student_id: &str,
    ) -> Result<ProgressAssessment, AssessStudentProgressError> {
        let profile = self.profile_gateway.get_by_id(student_id)?;
        let Some(instrument) = profile.instrument else {
            return Err(AssessStudentProgressError::NoInstrumentAssigned);
        };

        let lessons = self
            .lessons_gateway
            .get_all_for_student_with_id(student_id)?;

        Ok(assess(
            &profile.level,
            instrument,
            &lessons.msa,
            &lessons.method,
        )?)
    }
}
