use crate::shared::domain::entities::{Instrument, MusicianLevel};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StudentPosition {
    Musician {
        level: MusicianLevel,
        /// `None` when SAM has the student marked "A DEFINIR" (instrument
        /// not yet assigned).
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
