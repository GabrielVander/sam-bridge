use student::application::dto as student_dto;
use student::domain::entities::Clef;

pub struct RangeDto {
    pub from: String,
    pub to: String,
}

pub enum ClefDto {
    G,
    C,
    F,
}

pub struct LessonDto {
    pub id: Option<String>,
    pub date: Option<String>,
    pub phase: Option<RangeDto>,
    pub page: Option<RangeDto>,
    pub lesson: Option<RangeDto>,
    pub clef: Option<ClefDto>,
    pub description: Option<String>,
    pub instructor: Option<String>,
    pub method: Option<String>,
}

pub struct StudentLessonsDto {
    pub approved: Vec<LessonDto>,
    pub method: Vec<LessonDto>,
}

pub enum RetrieveStudentLessonsOutcome {
    Success(StudentLessonsDto),
    Failure(String),
}

impl From<student_dto::StudentLessonsDto> for StudentLessonsDto {
    fn from(dto: student_dto::StudentLessonsDto) -> Self {
        Self {
            approved: dto.approved.into_iter().map(LessonDto::from).collect(),
            method: dto.method.into_iter().map(LessonDto::from).collect(),
        }
    }
}

impl From<student_dto::LessonDto> for LessonDto {
    fn from(dto: student_dto::LessonDto) -> Self {
        Self {
            id: dto.id,
            date: dto.date.map(|d| d.format("%d/%m/%Y").to_string()),
            phase: dto.phase.map(RangeDto::from),
            page: dto.page.map(RangeDto::from),
            lesson: dto.lesson.map(RangeDto::from),
            clef: dto.clef.map(ClefDto::from),
            description: dto.description,
            instructor: dto.instructor,
            method: dto.method,
        }
    }
}

impl From<student::domain::entities::Range> for RangeDto {
    fn from(range: student::domain::entities::Range) -> Self {
        Self {
            from: range.from,
            to: range.to,
        }
    }
}

impl From<Clef> for ClefDto {
    fn from(clef: Clef) -> Self {
        match clef {
            Clef::G => Self::G,
            Clef::C => Self::C,
            Clef::F => Self::F,
        }
    }
}
