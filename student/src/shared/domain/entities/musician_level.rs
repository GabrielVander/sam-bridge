#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MusicianLevel {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Officialized,
    Unknown(String),
}

impl MusicianLevel {
    pub const ASCENDING: [Self; 5] = [
        Self::Candidate,
        Self::Practice,
        Self::YouthService,
        Self::OfficialService,
        Self::Officialized,
    ];

    #[must_use]
    pub const fn rank(&self) -> u8 {
        match self {
            Self::Candidate => 0,
            Self::Practice => 1,
            Self::YouthService => 2,
            Self::OfficialService => 3,
            Self::Officialized => 4,
            Self::Unknown(_) => u8::MAX,
        }
    }
}
