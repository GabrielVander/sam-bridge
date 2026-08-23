#[derive(Debug, PartialEq, Clone)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub position: StudentPosition,
    pub location: String,
    pub region: Region,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StudentPosition {
    Musician { level: MusicianLevel },
    Organist { level: OrganistLevel },
    Secretary { r#type: SecretaryType },
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum MusicianLevel {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Unknown(String),
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Region {
    AraraquaraSaoCarlos,
    AraraquaraItirapina,
    Other(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SecretaryType {
    Gem,
    Music,
}
