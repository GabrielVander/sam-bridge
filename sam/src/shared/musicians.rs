use student::domain::entities::{Instrument, MusicianLevel};

pub const MUSICIAN_ROLE: &str = "MÚSICO";

pub fn parse_musician_level(level: &str) -> MusicianLevel {
    match level {
        "CANDIDATO(A)" => MusicianLevel::Candidate,
        "ENSAIO" => MusicianLevel::Practice,
        "RJM" => MusicianLevel::YouthService,
        "RJM / ENSAIO" => MusicianLevel::YouthServicePractice,
        "CULTO OFICIAL" => MusicianLevel::OfficialService,
        other => MusicianLevel::Unknown(other.to_owned()),
    }
}

pub fn parse_instrument(instrument: &str) -> Option<Instrument> {
    match instrument.trim() {
        "" | "A DEFINIR" => None,
        "VIOLINO" => Some(Instrument::Violin),
        "VIOLA" => Some(Instrument::Viola),
        "VIOLONCELO" => Some(Instrument::Cello),
        "FLAUTA" => Some(Instrument::Flute),
        "OBOÉ" => Some(Instrument::Oboe),
        "FAGOTE" => Some(Instrument::Bassoon),
        "CLARINETE" => Some(Instrument::Clarinet),
        "CLARINETE ALTO" => Some(Instrument::AltoClarinet),
        "CLARINETE BAIXO" => Some(Instrument::BassClarinet),
        "SAXOFONE ALTO" => Some(Instrument::AltoSaxophone),
        "SAXOFONE SOPRANO CUR" => Some(Instrument::CurvedSopranoSaxophone),
        "SAXOFONE SOPRANO RET" => Some(Instrument::StraightSopranoSaxophone),
        "SAXOFONE TENOR" => Some(Instrument::TenorSaxophone),
        "TROMPETE" => Some(Instrument::Trumpet),
        "CORNET" => Some(Instrument::Cornet),
        "FLUGELHORN" => Some(Instrument::Flugelhorn),
        "TROMPA" => Some(Instrument::FrenchHorn),
        "TROMBONE" => Some(Instrument::Trombone),
        "EUPHONIUM" => Some(Instrument::Euphonium),
        "TUBA" => Some(Instrument::Tuba),
        "CORNE INGLÊS" => Some(Instrument::EnglishHorn),
        "VIOLINO CONTRALTO" => Some(Instrument::ContraltoViolin),
        other => Some(Instrument::Unknown(other.to_owned())),
    }
}
