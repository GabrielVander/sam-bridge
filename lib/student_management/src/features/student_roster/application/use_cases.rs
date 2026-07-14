use crate::api::{application::StudentsRetrievalGateway, domain::Student};

pub struct RetrieveStudentsUseCase<'a, T: StudentsRetrievalGateway> {
    gateway: &'a T,
}

impl<'a, T: StudentsRetrievalGateway> RetrieveStudentsUseCase<'a, T> {
    pub fn new(gateway: &'a T) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self) -> anyhow::Result<Vec<Student>> {
        self.gateway.get_avaliable_records().await
    }
}
