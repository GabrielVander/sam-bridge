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
    HalfHour,
    OfficialService,
    YouthServiceHalfHour,
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
