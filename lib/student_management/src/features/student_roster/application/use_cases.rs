use crate::features::student_roster::domain::{
    entities::Student, gateways::StudentsRetrievalGateway,
};

pub struct RetrieveStudentsUseCase<'a, T: StudentsRetrievalGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentsRetrievalGateway> RetrieveStudentsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self) -> Result<Vec<Student>, RetrieveStudentsError> {
        self.gateway
            .get_avaliable_records()
            .await
            .map_err(RetrieveStudentsError::from)
    }
}

#[derive(Debug)]
pub enum RetrieveStudentsError {
    Generic(String),
}

impl From<String> for RetrieveStudentsError {
    fn from(value: String) -> RetrieveStudentsError {
        RetrieveStudentsError::Generic(value)
    }
}
