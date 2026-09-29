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
    GemSecretary,
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
