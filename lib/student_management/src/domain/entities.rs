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
