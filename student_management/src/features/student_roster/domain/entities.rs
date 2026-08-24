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
    Officialized,
    Unknown(String),
}

impl MusicianLevel {
    /// Ordinal rank for level comparison. Higher = more advanced.
    /// Canonical name for serialization / lookup round-trips.
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

    /// Parses a canonical name back into a MusicianLevel.
    /// Unknown names are preserved as `Unknown(raw)` and must be handled
    /// explicitly — they carry no rank and should not be used for progress.
    pub fn parse_named(raw: &str) -> Self {
        match raw {
            "Candidate" => Self::Candidate,
            "Practice" => Self::Practice,
            "YouthService" => Self::YouthService,
            "OfficialService" => Self::OfficialService,
            "Officialized" => Self::Officialized,
            other => Self::Unknown(other.to_owned()),
        }
    }

    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }

    pub fn unknown_raw(&self) -> Option<&str> {
        if let Self::Unknown(raw) = self { Some(raw) } else { None }
    }

    pub fn rank(&self) -> u8 {
        match self {
            Self::Candidate => 0,
            Self::Practice => 1,
            Self::YouthService => 2,
            Self::OfficialService => 3,
            Self::Officialized => 4,
            Self::Unknown(_) => u8::MAX,
        }
    }

    pub fn rank_opt(&self) -> Option<u8> {
        match self {
            Self::Unknown(_) => None,
            _ => Some(self.rank()),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum OrganistLevel {
    Candidate,
    Practice,
    YouthService,
    #[deprecated(note = "typo: use HalfHour")]
    HafHour,
    HalfHour,
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
