use crate::domain::gateway::StudentGateway;

use super::entities::Student;

pub struct RetrieveStudentsImpl<'a, T: StudentGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentGateway> RetrieveStudentsImpl<'a, T> {
    pub async fn retrieve_students(&self) -> Result<Vec<Student>, RetrieveStudentsError> {
        self.gateway
            .get_avaliable_records()
            .await
            .map_err(RetrieveStudentsError::from)
    }
}

pub enum RetrieveStudentsError {
    Generic(String),
}

impl From<String> for RetrieveStudentsError {
    fn from(value: String) -> RetrieveStudentsError {
        RetrieveStudentsError::Generic(value)
    }
}
