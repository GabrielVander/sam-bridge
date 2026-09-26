use std::sync::Arc;
use student::application::gateways::{MusicianProfileGateway, MusicianProfileGatewayError};
use student::domain::entities::{MusicianProfile, StudentId};

use crate::client::{SamClient, SamStudent};
use crate::diagnostics::{error_chain, failure_kind};
use crate::shared::musicians::{MUSICIAN_ROLE, parse_instrument, parse_musician_level};

pub struct MusicianProfileGatewaySamImpl {
    client: Arc<dyn SamClient + Send + Sync>,
}

impl MusicianProfileGatewaySamImpl {
    pub fn new(client: Arc<dyn SamClient + Send + Sync>) -> Self {
        Self { client }
    }
}
impl MusicianProfileGateway for MusicianProfileGatewaySamImpl {
    fn get_by_id(&self, id: &StudentId) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        let sam_students: Vec<SamStudent> = self.client.students().map_err(|error| {
            MusicianProfileGatewayError::UnableToPerformOperation {
                kind: failure_kind(&error),
                details: error_chain(&error),
            }
        })?;

        let sam_student: SamStudent = sam_students
            .into_iter()
            .find(|student| student.id == id.as_str())
            .ok_or(MusicianProfileGatewayError::NotFound)?;

        if sam_student.role != MUSICIAN_ROLE {
            return Err(MusicianProfileGatewayError::NotAMusician);
        }

        Ok(MusicianProfile {
            level: parse_musician_level(&sam_student.level),
            instrument: parse_instrument(&sam_student.instrument),
        })
    }
}
