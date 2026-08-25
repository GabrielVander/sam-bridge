pub(crate) mod features;

pub mod api {

    pub mod application {
        pub use crate::features::{
            authentication::application::dto::{LoginInput, LoginOutput},
            authentication::application::gateways::AuthGateway,
            authentication::application::use_cases::LoginUseCase,
            student_lessons::application::dto::{LessonDto, StudentLessonsDto},
            student_lessons::application::gateways::StudentLessonsGateway,
            student_lessons::application::use_cases::RetrieveStudentLessonsUseCase,
            student_roster::application::dto::StudentSummaryDto,
            student_roster::application::gateways::StudentsRetrievalGateway,
            student_roster::application::use_cases::RetrieveStudentsUseCase,
        };
    }

    pub mod domain {
        pub use crate::features::{
            student_lessons::domain::entities::{Clef, Lesson, Range, StudentLessons},
            student_lessons::domain::progress::{
                CheckpointStatus, MethodProfile, ProgressAssessment, UnknownLevel,
                assess as calculate_progress_fn, violin_schmoll_profile,
            },
            student_roster::domain::entities::{
                MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
            },
        };
    }
}
