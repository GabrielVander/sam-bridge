//! Fakes for the `student` ports. Each answers every request with a fixed
//! result, whatever it is asked for.

use student::application::gateways::{
    MusicianProfileGateway, MusicianProfileGatewayError, StudentGateway, StudentGatewayError,
    StudentLessonsGateway, StudentLessonsGatewayError,
};
use student::domain::entities::{MusicianProfile, Student, StudentLessons};

pub struct FakeStudentGateway {
    result: Result<Vec<Student>, StudentGatewayError>,
}

impl FakeStudentGateway {
    #[must_use]
    pub const fn new(result: Result<Vec<Student>, StudentGatewayError>) -> Self {
        Self { result }
    }
}

impl StudentGateway for FakeStudentGateway {
    fn get_available_records(&self) -> Result<Vec<Student>, StudentGatewayError> {
        self.result.clone()
    }
}

pub struct FakeMusicianProfileGateway {
    result: Result<MusicianProfile, MusicianProfileGatewayError>,
}

impl FakeMusicianProfileGateway {
    #[must_use]
    pub const fn new(result: Result<MusicianProfile, MusicianProfileGatewayError>) -> Self {
        Self { result }
    }
}

impl MusicianProfileGateway for FakeMusicianProfileGateway {
    fn get_by_id(&self, _id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        self.result.clone()
    }
}

pub struct FakeStudentLessonsGateway {
    result: Result<StudentLessons, StudentLessonsGatewayError>,
}

impl FakeStudentLessonsGateway {
    #[must_use]
    pub const fn new(result: Result<StudentLessons, StudentLessonsGatewayError>) -> Self {
        Self { result }
    }
}

impl StudentLessonsGateway for FakeStudentLessonsGateway {
    fn get_all_for_student_with_id(
        &self,
        _student_id: &str,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.result.clone()
    }
}
