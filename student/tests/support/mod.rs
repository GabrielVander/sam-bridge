#![allow(dead_code)]

use student::application::gateways::{
    MusicianProfileGateway, MusicianProfileGatewayError, StudentLessonsGateway,
    StudentLessonsGatewayError,
};
use student::domain::entities::{
    CheckpointStatus, Lesson, MusicianLevel, MusicianProfile, ProgressAssessment, Range,
    StudentLessons,
};

pub struct FakeStudentLessonsGateway {
    result: Result<StudentLessons, StudentLessonsGatewayError>,
}

impl FakeStudentLessonsGateway {
    pub const fn returning(result: Result<StudentLessons, StudentLessonsGatewayError>) -> Self {
        Self { result }
    }
}

impl StudentLessonsGateway for FakeStudentLessonsGateway {
    fn get_all_for_student_with_id(
        &self,
        _id: &str,
    ) -> Result<StudentLessons, StudentLessonsGatewayError> {
        self.result.clone()
    }
}

pub struct FakeMusicianProfileGateway {
    result: Result<MusicianProfile, MusicianProfileGatewayError>,
}

impl FakeMusicianProfileGateway {
    pub const fn returning(result: Result<MusicianProfile, MusicianProfileGatewayError>) -> Self {
        Self { result }
    }
}

impl MusicianProfileGateway for FakeMusicianProfileGateway {
    fn get_by_id(&self, _id: &str) -> Result<MusicianProfile, MusicianProfileGatewayError> {
        self.result.clone()
    }
}

pub fn msa_lesson(from: &str, to: &str) -> Lesson {
    Lesson {
        phase: Some(Range::new(from.to_owned(), to.to_owned())),
        ..Lesson::default()
    }
}

pub fn method_lesson(page: &str, lesson: &str) -> Lesson {
    Lesson {
        page: Some(Range::single(page.to_owned())),
        lesson: Some(Range::single(lesson.to_owned())),
        ..Lesson::default()
    }
}

pub fn method_phase_lesson(phase: &str) -> Lesson {
    Lesson {
        phase: Some(Range::single(phase.to_owned())),
        ..Lesson::default()
    }
}

pub fn checkpoint<'a>(
    assessment: &'a ProgressAssessment,
    level: &MusicianLevel,
) -> Option<&'a CheckpointStatus> {
    assessment
        .checkpoints
        .iter()
        .find(|checkpoint| &checkpoint.level == level)
}
