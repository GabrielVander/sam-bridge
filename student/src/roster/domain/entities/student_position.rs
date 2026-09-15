use crate::shared::domain::entities::MusicianLevel;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StudentPosition {
    Musician { level: MusicianLevel },
    Organist { level: OrganistLevel },
    Secretary { r#type: SecretaryType },
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrganistLevel {
    Candidate,
    Practice,
    YouthService,
    HalfHour,
    OfficialService,
    YouthServiceHalfHour,
    YouthServicePractice,
    YouthServiceOfficialService,
    YouthServiceOfficialized,
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SecretaryType {
    Gem,
    Music,
}
