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
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::Candidate => "Candidate".to_owned(),
            Self::Practice => "Practice".to_owned(),
            Self::YouthService => "YouthService".to_owned(),
            Self::OfficialService => "OfficialService".to_owned(),
            Self::Officialized => "Officialized".to_owned(),
            Self::Unknown(raw) => raw.clone(),
        }
    }

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

#[cfg(test)]
mod tests {
    use super::MusicianLevel;

    #[test]
    fn name_matches_every_known_variant() {
        assert_eq!(MusicianLevel::Candidate.name(), "Candidate");
        assert_eq!(MusicianLevel::Practice.name(), "Practice");
        assert_eq!(MusicianLevel::YouthService.name(), "YouthService");
        assert_eq!(MusicianLevel::OfficialService.name(), "OfficialService");
        assert_eq!(MusicianLevel::Officialized.name(), "Officialized");
        assert_eq!(
            MusicianLevel::Unknown("Strawberry".to_owned()).name(),
            "Strawberry"
        );
    }

    #[test]
    fn rank_orders_every_known_variant_below_unknown() {
        assert_eq!(MusicianLevel::Candidate.rank(), 0);
        assert_eq!(MusicianLevel::Practice.rank(), 1);
        assert_eq!(MusicianLevel::YouthService.rank(), 2);
        assert_eq!(MusicianLevel::OfficialService.rank(), 3);
        assert_eq!(MusicianLevel::Officialized.rank(), 4);
        assert_eq!(MusicianLevel::Unknown("x".to_owned()).rank(), u8::MAX);
    }
}
