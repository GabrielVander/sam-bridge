/// Instruments as SAM's own roster actually classifies them, confirmed
/// against the live students listing (`role = "MÚSICO"`).
///
/// A few real SAM categories have no published test requirements in
/// Formulário M09 (`AltoClarinet`, `EnglishHorn`, `ContraltoViolin`) and some
/// are aliases the requirements sheet explicitly folds into another
/// instrument's method track (see `also_accepted_as`) - both are still
/// identities SAM assigns to real students, so they're represented here even
/// though `InstrumentRequirements` (in the `lessons` bounded context) has no
/// catalog entry for the first group.
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
    /// Corne Inglês: an oboe-family instrument SAM tracks as its own
    /// category. Not in Formulário M09's instrument list.
    EnglishHorn,
    /// Violino Contralto: tracked by SAM as distinct from `Viola`. Not in
    /// Formulário M09's instrument list.
    ContraltoViolin,
    /// A raw SAM instrument string we don't yet recognize, kept verbatim
    /// rather than guessed at - mirrors `MusicianLevel::Unknown`.
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

    /// Parses back the exact string `name()` produces (round-trip pair, the
    /// same pattern as `MusicianLevel::parse_named`). Used to carry an
    /// instrument through a boundary that only understands plain strings
    /// (e.g. across the Flutter bridge) without inventing a translation
    /// layer for names Formulário M09 and SAM's own roster already give us.
    #[must_use]
    pub fn parse_named(raw: &str) -> Self {
        match raw {
            "Violino" => Self::Violin,
            "Viola" => Self::Viola,
            "Violoncelo" => Self::Cello,
            "Flauta" => Self::Flute,
            "Oboé" => Self::Oboe,
            "Fagote" => Self::Bassoon,
            "Clarinete" => Self::Clarinet,
            "Clarinete Alto" => Self::AltoClarinet,
            "Clarinete Baixo" => Self::BassClarinet,
            "Saxofone" => Self::Saxophone,
            "Trompete" => Self::Trumpet,
            "Trompa" => Self::FrenchHorn,
            "Trombone" => Self::Trombone,
            "Eufônio" => Self::Euphonium,
            "Tuba" => Self::Tuba,
            "Corne Inglês" => Self::EnglishHorn,
            "Violino Contralto" => Self::ContraltoViolin,
            other => Self::Unknown(other.to_owned()),
        }
    }

    /// Other instrument names the sheet (or SAM's own roster) accepts as
    /// equivalent for testing purposes (e.g. a Cornet player is evaluated
    /// against Trumpet requirements).
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

    #[test]
    fn name_and_parse_named_round_trip_for_every_variant() {
        let variants = [
            Instrument::Violin,
            Instrument::Viola,
            Instrument::Cello,
            Instrument::Flute,
            Instrument::Oboe,
            Instrument::Bassoon,
            Instrument::Clarinet,
            Instrument::AltoClarinet,
            Instrument::BassClarinet,
            Instrument::Saxophone,
            Instrument::Trumpet,
            Instrument::FrenchHorn,
            Instrument::Trombone,
            Instrument::Euphonium,
            Instrument::Tuba,
            Instrument::EnglishHorn,
            Instrument::ContraltoViolin,
        ];

        for variant in variants {
            assert_eq!(Instrument::parse_named(&variant.name()), variant);
        }
    }

    #[test]
    fn parse_named_falls_through_to_unknown() {
        assert_eq!(
            Instrument::parse_named("BANDOLIM"),
            Instrument::Unknown("BANDOLIM".to_owned())
        );
    }
}
