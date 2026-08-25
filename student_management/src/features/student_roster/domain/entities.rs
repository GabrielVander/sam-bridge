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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn musician_level_name_and_parse() {
        assert_eq!(MusicianLevel::Candidate.name(), "Candidate");
        assert_eq!(MusicianLevel::parse_named("Candidate"), MusicianLevel::Candidate);
        assert_eq!(MusicianLevel::parse_named("UnknownX"), MusicianLevel::Unknown("UnknownX".to_owned()));
        assert!(MusicianLevel::Unknown("x".to_owned()).is_unknown());
        assert_eq!(MusicianLevel::Candidate.unknown_raw(), None);
        assert_eq!(MusicianLevel::Unknown("raw".to_owned()).unknown_raw(), Some("raw"));
    }

    #[test]
    fn musician_level_rank() {
        assert_eq!(MusicianLevel::Candidate.rank(), 0);
        assert_eq!(MusicianLevel::Officialized.rank(), 4);
        assert_eq!(MusicianLevel::Unknown("x".to_owned()).rank(), u8::MAX);
        assert_eq!(MusicianLevel::Unknown("x".to_owned()).rank_opt(), None);
        assert_eq!(MusicianLevel::Candidate.rank_opt(), Some(0));
    }

    #[test]
    fn student_clone_and_position() {
        let s = Student {
            id: "1".to_owned(),
            name: "A".to_owned(),
            position: StudentPosition::Musician { level: MusicianLevel::Candidate },
            location: "LOC".to_owned(),
            region: Region::AraraquaraSaoCarlos,
        };
        assert_eq!(s.clone().id, "1");
        let o = StudentPosition::Organist { level: OrganistLevel::HalfHour };
        assert_eq!(o.clone(), o);
        let sec = StudentPosition::Secretary { r#type: SecretaryType::Gem };
        assert!(matches!(sec, StudentPosition::Secretary { r#type: SecretaryType::Gem }));
        let unk = StudentPosition::Unknown("x".to_owned());
        assert_eq!(unk.clone(), unk);
        assert_eq!(Region::Other("x".to_owned()).clone(), Region::Other("x".to_owned()));
        assert_eq!(OrganistLevel::YouthServiceHalfHour.clone(), OrganistLevel::YouthServiceHalfHour);
        assert_eq!(SecretaryType::Music.clone(), SecretaryType::Music);
    }
}
