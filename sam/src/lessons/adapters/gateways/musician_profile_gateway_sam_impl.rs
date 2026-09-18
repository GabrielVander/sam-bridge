use std::sync::Arc;
use student::application::gateways::{MusicianProfileGateway, MusicianProfileGatewayError};
use student::domain::entities::{MusicianProfile, Student, StudentPosition};

use crate::client::{SamClient, SamStudent};

pub struct MusicianProfileGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl MusicianProfileGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}
impl MusicianProfileGateway for MusicianProfileGatewaySamImpl {
    fn get_by_id(&self, id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        let sam_students: Vec<SamStudent> = self
            .client
            .students()
            .map_err(|_| MusicianProfileGatewayError::UnableToPerformOperation)?;

        let sam_student = sam_students
            .into_iter()
            .find(|student| student.id == id)
            .ok_or(MusicianProfileGatewayError::NotFound)?;

        match Student::from(sam_student).position {
            StudentPosition::Musician {
                level, instrument, ..
            } => Ok(MusicianProfile { level, instrument }),
            StudentPosition::Organist { .. }
            | StudentPosition::Secretary { .. }
            | StudentPosition::Unknown(_) => Err(MusicianProfileGatewayError::NotAMusician),
        }
    }
}
