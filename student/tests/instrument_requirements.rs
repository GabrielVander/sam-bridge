use pretty_assertions::assert_eq;
use student::domain::entities::{
    Instrument, InstrumentRequirements, MethodMilestone, MusicianLevel, TestRequirement,
};

#[test]
fn violin_matches_known_youth_and_official_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Violin)
            .expect("violin requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.theory.msa_phase, 12);
    assert_eq!(youth.method_alternatives.len(), 3);
    assert_eq!(
        youth.method_alternatives[1].components[0].milestone,
        MethodMilestone::PageAndLesson {
            page: 46,
            lesson: 113
        }
    );

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(official.theory.msa_phase, 16);
    assert_eq!(
        official.method_alternatives[1].components[0].milestone,
        MethodMilestone::PageAndLesson {
            page: 67,
            lesson: 162
        }
    );

    let officialized: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::Officialized)
        .expect("officialization requirement exists");

    assert_eq!(officialized.theory.msa_phase, 16);
    assert_eq!(
        officialized.theory.note,
        Some("Com repasse na leitura métrica a partir da lição 56")
    );
}

#[test]
fn viola_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Viola)
            .expect("viola requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.theory.msa_phase, 12);
    assert_eq!(
        youth.method_alternatives[0].components[1].milestone,
        MethodMilestone::Page(31)
    );

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(official.theory.msa_phase, 16);
    assert_eq!(
        official.method_alternatives[0].components[0].milestone,
        MethodMilestone::Page(62)
    );
}

#[test]
fn cello_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Cello)
            .expect("cello requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(
        youth.method_alternatives[0].components[1].milestone,
        MethodMilestone::PageAndLesson {
            page: 34,
            lesson: 80
        }
    );

    let officialized: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::Officialized)
        .expect("officialization requirement exists");

    assert_eq!(
        officialized.method_alternatives[0].components[1].milestone,
        MethodMilestone::PageAndLesson {
            page: 19,
            lesson: 154
        }
    );
}

#[test]
fn flute_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Flute)
            .expect("flute requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.method_alternatives.len(), 3);
    assert_eq!(
        youth.method_alternatives[2].components[0].milestone,
        MethodMilestone::Phase(13)
    );

    let officialized: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::Officialized)
        .expect("officialization requirement exists");

    assert!(
        officialized
            .method_alternatives
            .iter()
            .all(|alt| alt.components[0].milestone == MethodMilestone::Complete)
    );
}

#[test]
fn oboe_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Oboe)
            .expect("oboe requirements are published");

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(
        official.method_alternatives[0].components[0].milestone,
        MethodMilestone::Page(16)
    );
    assert_eq!(
        official.method_alternatives[1].components[0].milestone,
        MethodMilestone::Page(30)
    );
}

#[test]
fn bassoon_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Bassoon)
            .expect("bassoon requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(
        youth.method_alternatives[1].components[0].milestone,
        MethodMilestone::Module(12)
    );

    let officialized: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::Officialized)
        .expect("officialization requirement exists");

    assert_eq!(
        officialized.method_alternatives[1].components[0].milestone,
        MethodMilestone::Module(22)
    );
}

#[test]
fn clarinet_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Clarinet)
            .expect("clarinet requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.method_alternatives.len(), 3);

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(official.method_alternatives.len(), 4);
    assert_eq!(
        official.method_alternatives[2].components[0].milestone,
        MethodMilestone::Lesson(36)
    );
    assert_eq!(
        official.method_alternatives[3].components[1].milestone,
        MethodMilestone::Page(18)
    );
}

#[test]
fn bass_clarinet_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::BassClarinet)
            .expect("bass clarinet requirements are published");

    let officialized: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::Officialized)
        .expect("officialization requirement exists");

    assert_eq!(
        officialized.method_alternatives[0].components[0].milestone,
        MethodMilestone::Complete
    );
    assert_eq!(
        officialized.method_alternatives[1].components[1].milestone,
        MethodMilestone::Page(29)
    );
}

#[test]
fn saxophone_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Saxophone)
            .expect("saxophone requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.method_alternatives.len(), 3);
    assert_eq!(
        youth.method_alternatives[2].components[0].milestone,
        MethodMilestone::Phase(13)
    );

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(
        official.method_alternatives[1].components[0].milestone,
        MethodMilestone::Page(40)
    );
}

#[test]
fn trumpet_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Trumpet)
            .expect("trumpet requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.method_alternatives.len(), 1);
    assert_eq!(
        youth.method_alternatives[0].components[0].milestone,
        MethodMilestone::Complete
    );

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(
        official.method_alternatives[0].components[0].milestone,
        MethodMilestone::ExerciseRange { from: 65, to: 94 }
    );
}

#[test]
fn tuba_matches_known_thresholds() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Tuba)
            .expect("tuba requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(
        youth.method_alternatives[0].components[0].milestone,
        MethodMilestone::Page(24)
    );

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(
        official.method_alternatives[1].components[0].milestone,
        MethodMilestone::Phase(25)
    );
}

#[test]
fn french_horn_has_a_single_mandatory_combined_alternative() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::FrenchHorn)
            .expect("french horn requirements are published");

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.method_alternatives.len(), 1);
    assert_eq!(youth.method_alternatives[0].components.len(), 2);
}

#[test]
fn trombone_and_euphonium_share_the_same_requirement_shape() {
    let trombone: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Trombone)
            .expect("trombone requirements are published");

    let euphonium: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Euphonium)
            .expect("euphonium requirements are published");

    assert_eq!(trombone.tests, euphonium.tests);
    assert_eq!(trombone.instrument, Instrument::Trombone);
    assert_eq!(euphonium.instrument, Instrument::Euphonium);
}

#[test]
fn alto_clarinet_has_no_published_requirements_yet() {
    assert_eq!(
        InstrumentRequirements::for_instrument(&Instrument::AltoClarinet),
        None
    );
}

#[test]
fn real_sam_instrument_categories_without_a_published_sheet_entry_have_no_requirements() {
    assert_eq!(
        InstrumentRequirements::for_instrument(&Instrument::EnglishHorn),
        None
    );
    assert_eq!(
        InstrumentRequirements::for_instrument(&Instrument::ContraltoViolin),
        None
    );
    assert_eq!(
        InstrumentRequirements::for_instrument(&Instrument::Unknown("BANDOLIM".to_owned())),
        None
    );
}

#[test]
fn requirement_for_unpublished_level_is_none() {
    let requirements: InstrumentRequirements =
        InstrumentRequirements::for_instrument(&Instrument::Violin)
            .expect("violin requirements are published");

    assert_eq!(
        requirements.requirement_for(&MusicianLevel::Candidate),
        None
    );
}

#[test]
fn youth_service_reads_hymns_431_to_480_and_every_other_level_the_whole_hymnal() {
    let requirements: InstrumentRequirements = InstrumentRequirements::violin();

    let youth: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::YouthService)
        .expect("youth service requirement exists");

    assert_eq!(youth.metric_reading, "Hinos 431 a 480");
    assert_eq!(youth.hymnal, "431 a 480 - Voz principal + Voz alternativa");

    let official: &TestRequirement = requirements
        .requirement_for(&MusicianLevel::OfficialService)
        .expect("official service requirement exists");

    assert_eq!(official.metric_reading, "Todos os Hinos");
    assert_eq!(
        official.hymnal,
        "Completo - Voz principal + Voz alternativa"
    );
}
