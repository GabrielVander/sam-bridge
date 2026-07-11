#[derive(Debug)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub position: StudentPosition,
    pub location: String,
    pub region: Region,
}

#[derive(Debug)]
pub enum StudentPosition {
    Musician { level: MusicianLevel },
    Organist { level: OrganistLevel },
    Secretary { r#type: SecretaryType },
    Unknown(String),
}

#[derive(Debug)]
pub enum MusicianLevel {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Unknown(String),
}

#[derive(Debug)]
pub enum OrganistLevel {
    Candidate,
    Practice,
    YouthService,
    HafHour,
    OfficialService,
    YouthServiceHafHour,
    YouthServicePractice,
    YouthServiceOfficialService,
    YouthServiceOfficialized,
    Unknown(String),
}

#[derive(Debug)]
pub enum Region {
    AraraquaraSaoCarlos,
    AraraquaraItirapina,
    Other(String),
}

#[derive(Debug)]
pub enum SecretaryType {
    Gem,
    Music,
}

#[derive(Debug)]
pub struct Lesson {
    pub id: String,
    pub date: String,
    pub phase: Option<Range>,
    pub page: Option<Range>,
    pub lesson: Option<Range>,
    pub clef: Option<Clef>,
    pub description: Option<String>,
    pub instructor: Option<String>,
}

#[derive(Debug)]
pub struct Range {
    pub from: String,
    pub to: String,
}

#[derive(Debug)]
pub enum Clef {
    G,
    C,
    F,
}
