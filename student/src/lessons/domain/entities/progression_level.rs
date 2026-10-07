#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProgressionLevel {
    Candidate,
    Practice,
    YouthService,
    OfficialService,
    Officialized,
}

impl ProgressionLevel {
    pub const ASCENDING: [Self; 5] = [
        Self::Candidate,
        Self::Practice,
        Self::YouthService,
        Self::OfficialService,
        Self::Officialized,
    ];
}
