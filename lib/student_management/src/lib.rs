pub(crate) mod features;

pub mod api {

    pub mod application {
        pub use crate::features::{
            authentication::application::gateways::AuthGateway,
            authentication::application::use_cases::LoginUseCase,
            student_lessons::application::gateways::StudentLessonsGateway,
            student_lessons::application::use_cases::RetrieveStudentLessonsUseCase,
            student_roster::application::gateways::StudentsRetrievalGateway,
            student_roster::application::use_cases::RetrieveStudentsUseCase,
        };
    }

    pub mod domain {
        pub use crate::features::{
            student_lessons::domain::entities::{Clef, Lesson, Range},
            student_roster::domain::entities::{
                MusicianLevel, OrganistLevel, Region, SecretaryType, Student, StudentPosition,
            },
        };
    }
}
