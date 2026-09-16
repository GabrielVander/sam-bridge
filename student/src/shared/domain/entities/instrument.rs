#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Instrument {
    Violin,
    Viola,
    Cello,
    Flute,
    Oboe,
    Bassoon,
    Clarinet,
    AltoClarinet,
    BassClarinet,
    Saxophone,
    Trumpet,
    FrenchHorn,
    Trombone,
    Euphonium,
    Tuba,
    EnglishHorn,
    ContraltoViolin,
    Unknown(String),
}

impl Instrument {
    #[must_use]
    pub fn name(&self) -> String {
        match self {
            Self::Violin => "Violino".to_owned(),
            Self::Viola => "Viola".to_owned(),
            Self::Cello => "Violoncelo".to_owned(),
            Self::Flute => "Flauta".to_owned(),
            Self::Oboe => "Oboé".to_owned(),
            Self::Bassoon => "Fagote".to_owned(),
            Self::Clarinet => "Clarinete".to_owned(),
            Self::AltoClarinet => "Clarinete Alto".to_owned(),
            Self::BassClarinet => "Clarinete Baixo".to_owned(),
            Self::Saxophone => "Saxofone".to_owned(),
            Self::Trumpet => "Trompete".to_owned(),
            Self::FrenchHorn => "Trompa".to_owned(),
            Self::Trombone => "Trombone".to_owned(),
            Self::Euphonium => "Eufônio".to_owned(),
            Self::Tuba => "Tuba".to_owned(),
            Self::EnglishHorn => "Corne Inglês".to_owned(),
            Self::ContraltoViolin => "Violino Contralto".to_owned(),
            Self::Unknown(raw) => raw.clone(),
        }
    }

    #[must_use]
    pub const fn also_accepted_as(&self) -> &'static [&'static str] {
        match self {
            Self::Trumpet => &["Cornet", "Flugelhorn"],
            Self::Trombone | Self::Euphonium => &["Trombonito"],
            _ => &[],
        }
    }

    #[must_use]
    pub const fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown(_))
    }
}

#[cfg(test)]
mod tests {
    use super::Instrument;

    #[test]
    fn trumpet_also_accepts_cornet_and_flugelhorn() {
        assert_eq!(
            Instrument::Trumpet.also_accepted_as(),
            &["Cornet", "Flugelhorn"]
        );
        assert_eq!(Instrument::Trombone.also_accepted_as(), &["Trombonito"]);
        let empty: &[&str] = &[];
        assert_eq!(Instrument::Violin.also_accepted_as(), empty);
    }

    #[test]
    fn unknown_instrument_keeps_raw_name() {
        let instrument = Instrument::Unknown("BANDOLIM".to_owned());
        assert!(instrument.is_unknown());
        assert_eq!(instrument.name(), "BANDOLIM");
        assert!(!Instrument::Violin.is_unknown());
    }
}
