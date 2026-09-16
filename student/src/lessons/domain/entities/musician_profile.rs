use crate::shared::domain::entities::{Instrument, MusicianLevel};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MusicianProfile {
    pub level: MusicianLevel,
    pub instrument: Option<Instrument>,
}
