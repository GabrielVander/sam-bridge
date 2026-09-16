use std::sync::Arc;

use async_trait::async_trait;
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

#[async_trait]
impl MusicianProfileGateway for MusicianProfileGatewaySamImpl {
    async fn get_by_id(&self, id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        let sam_students: Vec<SamStudent> = self
            .client
            .students()
            .map_err(|_| MusicianProfileGatewayError::UnableToPerformOperation)?;

        let sam_student = sam_students
            .into_iter()
            .find(|student| student.id == id)
            .ok_or(MusicianProfileGatewayError::NotFound)?;

        match Student::from(sam_student).position {
            StudentPosition::Musician { level, instrument } => {
                Ok(MusicianProfile { level, instrument })
            }
            StudentPosition::Organist { .. }
            | StudentPosition::Secretary { .. }
            | StudentPosition::Unknown(_) => Err(MusicianProfileGatewayError::NotAMusician),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::{SamClientError, SamCredentials, StudentLessonsPage};
    use student::domain::entities::{Instrument, MusicianLevel};

    struct FakeSamClient {
        students: Vec<SamStudent>,
        fail: bool,
    }

    impl SamClient for FakeSamClient {
        fn login(&self, _credentials: &SamCredentials) -> Result<(), SamClientError> {
            Err(SamClientError::InvalidCredentials)
        }

        fn students(&self) -> Result<Vec<SamStudent>, SamClientError> {
            if self.fail {
                return Err(SamClientError::InvalidCredentials);
            }
            Ok(self.students.clone())
        }

        fn student_lessons(&self, _student_id: &str) -> Result<StudentLessonsPage, SamClientError> {
            Err(SamClientError::InvalidCredentials)
        }
    }

    fn sam_student(id: &str, role: &str, level: &str, instrument: &str) -> SamStudent {
        SamStudent {
            id: id.to_owned(),
            name: "Someone".to_owned(),
            location: "Somewhere".to_owned(),
            role: role.to_owned(),
            level: level.to_owned(),
            instrument: instrument.to_owned(),
        }
    }

    fn gateway(students: Vec<SamStudent>) -> MusicianProfileGatewaySamImpl {
        MusicianProfileGatewaySamImpl::new(Arc::new(FakeSamClient {
            students,
            fail: false,
        }))
    }

    #[test]
    fn returns_the_musicians_level_and_instrument() {
        smol::block_on(async {
            let gateway = gateway(vec![sam_student("1", "MÚSICO", "RJM", "VIOLINO")]);

            let profile = gateway.get_by_id("1").await.expect("should succeed");

            assert_eq!(profile.level, MusicianLevel::YouthService);
            assert_eq!(profile.instrument, Some(Instrument::Violin));
        });
    }

    #[test]
    fn student_without_an_assigned_instrument_has_none() {
        smol::block_on(async {
            let gateway = gateway(vec![sam_student(
                "1",
                "MÚSICO",
                "CANDIDATO(A)",
                "A DEFINIR",
            )]);

            let profile = gateway.get_by_id("1").await.expect("should succeed");

            assert_eq!(profile.instrument, None);
        });
    }

    #[test]
    fn unknown_id_is_not_found() {
        smol::block_on(async {
            let gateway = gateway(vec![sam_student("1", "MÚSICO", "RJM", "VIOLINO")]);

            let result = gateway.get_by_id("does-not-exist").await;

            assert_eq!(result, Err(MusicianProfileGatewayError::NotFound));
        });
    }

    #[test]
    fn non_musician_is_reported_as_not_a_musician() {
        smol::block_on(async {
            let gateway = gateway(vec![sam_student("1", "ORGANISTA", "RJM", "A DEFINIR")]);

            let result = gateway.get_by_id("1").await;

            assert_eq!(result, Err(MusicianProfileGatewayError::NotAMusician));
        });
    }

    #[test]
    fn client_failure_is_reported_as_unable_to_perform_operation() {
        smol::block_on(async {
            let gateway = MusicianProfileGatewaySamImpl::new(Arc::new(FakeSamClient {
                students: Vec::new(),
                fail: true,
            }));

            let result = gateway.get_by_id("1").await;

            assert_eq!(
                result,
                Err(MusicianProfileGatewayError::UnableToPerformOperation)
            );
        });
    }
}
