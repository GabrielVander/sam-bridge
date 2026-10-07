#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MusicianLevel {
    Candidate,
    Practice,
    YouthService,
    YouthServicePractice,
    OfficialService,
    Officialized,
    Unknown(String),
}
