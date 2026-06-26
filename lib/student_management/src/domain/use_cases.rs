use std::future::Future;

use super::entities::Student;

async fn retrieve_students<GetAllStudents, GetAllStudentsFuture>(
    get_all_students_op: GetAllStudents,
) -> Result<Vec<Student>, RetrieveStudentError>
where
    GetAllStudents: Fn() -> GetAllStudentsFuture,
    GetAllStudentsFuture: Future<Output = Result<Vec<Student>, String>>,
{
    get_all_students_op()
        .await
        .map_err(RetrieveStudentError::from)
}

enum RetrieveStudentError {
    Generic(String),
}

impl From<String> for RetrieveStudentError {
    fn from(value: String) -> RetrieveStudentError {
        RetrieveStudentError::Generic(format!(
            "Students retrieval failed unexpectedly: {:?}",
            value
        ))
    }
}
