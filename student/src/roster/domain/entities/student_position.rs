use crate::shared::domain::entities::{Instrument, MusicianLevel};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StudentPosition {
    Musician {
        level: MusicianLevel,
        instrument: Option<Instrument>,
    },
    Organist {
        level: OrganistLevel,
    },
    Secretary {
        r#type: SecretaryType,
    },
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum OrganistLevel {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    YouthServiceHalfHour,
    Unknown(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SecretaryType {
    Gem,
}
