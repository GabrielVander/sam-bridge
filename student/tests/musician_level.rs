use student::domain::entities::MusicianLevel;

#[test]
fn levels_rank_in_the_order_a_musician_progresses_through_them() {
    let journey: [MusicianLevel; 5] = [
        MusicianLevel::Candidate,
        MusicianLevel::Practice,
        MusicianLevel::YouthService,
        MusicianLevel::OfficialService,
        MusicianLevel::Officialized,
    ];

    for (lower, higher) in journey.iter().zip(journey.iter().skip(1)) {
        assert!(
            lower.rank() < higher.rank(),
            "{lower:?} should rank below {higher:?}"
        );
    }
}

#[test]
fn an_unknown_level_ranks_above_every_known_one() {
    let unknown: MusicianLevel = MusicianLevel::Unknown("EXÓTICO".to_owned());

    assert!(MusicianLevel::Officialized.rank() < unknown.rank());
}
