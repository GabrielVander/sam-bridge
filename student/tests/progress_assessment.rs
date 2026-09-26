use pretty_assertions::assert_eq;
use student::domain::entities::{
    AssessError, CheckpointStatus, Instrument, Lesson, MusicianLevel, ProgressAssessment, assess,
};

#[path = "support/helpers.rs"]
mod support;
use support::{checkpoint, method_lesson, method_phase_lesson, msa_lesson};

#[test]
fn a_candidate_without_lessons_has_only_the_first_checkpoint() {
    let assessment: ProgressAssessment =
        assess(&MusicianLevel::Candidate, Instrument::Violin, &[], &[]).unwrap();

    assert!(
        checkpoint(&assessment, &MusicianLevel::Candidate)
            .unwrap()
            .achieved
    );
    assert!(!assessment.checkpoints.iter().any(|c| c.ready_to_advance));
    assert!((assessment.overall_checkpoint_percent - 20.0).abs() < 0.1);
    assert!(
        is_about(assessment.msa_relative_percent, 0.0),
        "got {}",
        assessment.msa_relative_percent
    );
    assert!(
        is_about(assessment.method_relative_percent, 0.0),
        "got {}",
        assessment.method_relative_percent
    );
}

#[test]
fn the_assigned_level_and_every_level_below_it_are_achieved() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::YouthService,
        Instrument::Violin,
        &[msa_lesson("3", "3")],
        &[method_lesson("10", "20")],
    )
    .unwrap();

    let achieved: Vec<(MusicianLevel, bool)> = assessment
        .checkpoints
        .iter()
        .map(|c| (c.level.clone(), c.achieved))
        .collect();

    assert_eq!(
        achieved,
        vec![
            (MusicianLevel::Candidate, true),
            (MusicianLevel::Practice, true),
            (MusicianLevel::YouthService, true),
            (MusicianLevel::OfficialService, false),
            (MusicianLevel::Officialized, false),
        ]
    );
}

#[test]
fn meeting_a_higher_levels_requirements_shows_ready_to_advance() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson("12", "12")],
        &[method_lesson("46", "113")],
    )
    .unwrap();

    let practice: &CheckpointStatus = checkpoint(&assessment, &MusicianLevel::Practice).unwrap();
    assert!(!practice.achieved);
    assert!(!practice.ready_to_advance);

    let youth_service: &CheckpointStatus =
        checkpoint(&assessment, &MusicianLevel::YouthService).unwrap();
    assert!(!youth_service.achieved);
    assert!(youth_service.ready_to_advance);
}

#[test]
fn the_best_alternative_counts_even_when_it_is_not_the_first_listed() {
    let assessment: ProgressAssessment =
        candidate_on(Instrument::Violin, &[method_lesson("0", "6")]).unwrap();

    assert!(assessment.method_relative_percent > 40.0);
}

#[test]
fn official_service_needs_msa_phase_16() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::YouthService,
        Instrument::Violin,
        &[msa_lesson("15", "16")],
        &[method_lesson("67", "162")],
    )
    .unwrap();

    assert!(
        checkpoint(&assessment, &MusicianLevel::OfficialService)
            .unwrap()
            .ready_to_advance
    );
}

#[test]
fn officialization_cannot_be_verified_from_the_lessons_alone() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::OfficialService,
        Instrument::Violin,
        &[msa_lesson("16", "16")],
        &[method_lesson("999", "999")],
    )
    .unwrap();

    let officialized: &CheckpointStatus =
        checkpoint(&assessment, &MusicianLevel::Officialized).unwrap();

    assert!(!officialized.ready_to_advance);
    assert!(!officialized.requirement.method_met);
}

#[test]
fn an_officialized_musician_has_completed_the_whole_journey() {
    let assessment: ProgressAssessment =
        assess(&MusicianLevel::Officialized, Instrument::Violin, &[], &[]).unwrap();

    assert!(assessment.checkpoints.iter().all(|c| c.achieved));
    assert_eq!(assessment.next_level, None);
    assert!((assessment.overall_checkpoint_percent - 100.0).abs() < f64::EPSILON);
    assert!((assessment.combined_percent - 100.0).abs() < f64::EPSILON);
}

#[test]
fn lessons_without_recorded_progress_contribute_nothing() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[Lesson::default()],
        &[Lesson::default()],
    )
    .unwrap();

    assert!(
        is_about(assessment.msa_relative_percent, 0.0),
        "got {}",
        assessment.msa_relative_percent
    );
    assert!(
        is_about(assessment.method_relative_percent, 0.0),
        "got {}",
        assessment.method_relative_percent
    );
    assert!(
        is_about(assessment.combined_percent, 0.0),
        "got {}",
        assessment.combined_percent
    );
}

#[test]
fn unreadable_progress_contributes_nothing() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson("abc", "def")],
        &[],
    )
    .unwrap();

    assert!(
        is_about(assessment.msa_relative_percent, 0.0),
        "got {}",
        assessment.msa_relative_percent
    );
}

#[test]
fn the_furthest_lesson_counts_not_the_latest() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[
            msa_lesson("3", "3"),
            msa_lesson("14", "14"),
            msa_lesson("7", "7"),
        ],
        &[],
    )
    .unwrap();

    assert!((assessment.msa_relative_percent - 100.0).abs() < 0.1);
    assert_eq!(assessment.next_level, Some(MusicianLevel::YouthService));
}

#[test]
fn a_range_counts_up_to_its_upper_bound() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson("11", "13")],
        &[],
    )
    .unwrap();

    assert!((assessment.msa_relative_percent - 100.0).abs() < 0.1);
}

#[test]
fn msa_progress_is_the_share_of_the_required_phase_reached() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson("8", "8")],
        &[],
    )
    .unwrap();

    assert!((assessment.msa_relative_percent - 66.6).abs() < 0.5);
}

#[test]
fn overall_progress_adds_the_share_of_the_next_level_already_covered() {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson("6", "6")],
        &[],
    )
    .unwrap();

    assert!(
        is_about(assessment.combined_percent, 25.0),
        "got {}",
        assessment.combined_percent
    );
    assert!(
        is_about(assessment.overall_checkpoint_percent, 25.0),
        "got {}",
        assessment.overall_checkpoint_percent
    );
}

#[test]
fn an_unknown_level_cannot_be_assessed() {
    let result: Result<ProgressAssessment, AssessError> = assess(
        &MusicianLevel::Unknown("EXÓTICO".to_owned()),
        Instrument::Violin,
        &[msa_lesson("16", "16")],
        &[method_lesson("80", "214")],
    );

    assert_eq!(result, Err(AssessError::UnknownLevel("EXÓTICO".to_owned())));
}

#[test]
fn an_unknown_level_explains_itself_with_what_sam_wrote() {
    let error: AssessError = AssessError::UnknownLevel("EXÓTICO".to_owned());

    assert_eq!(error.to_string(), "unrecognized musician level \"EXÓTICO\"");
}

#[test]
fn an_instrument_without_published_requirements_cannot_be_assessed() {
    let result: Result<ProgressAssessment, AssessError> = assess(
        &MusicianLevel::Candidate,
        Instrument::AltoClarinet,
        &[],
        &[],
    );

    assert_eq!(
        result,
        Err(AssessError::UnpublishedRequirements(
            Instrument::AltoClarinet
        ))
    );
}

#[test]
fn a_page_milestone_is_met_from_the_target_page_on() {
    let at_target: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_lesson("41", "0")]).unwrap();
    let one_short: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_lesson("40", "0")]).unwrap();

    assert_eq!(meets_youth_service_method(&at_target), Some(true));
    assert_eq!(meets_youth_service_method(&one_short), Some(false));
}

#[test]
fn a_lesson_milestone_is_met_from_the_target_lesson_on() {
    let at_target: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_lesson("0", "41")]).unwrap();
    let one_short: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_lesson("0", "40")]).unwrap();

    assert_eq!(meets_youth_service_method(&at_target), Some(true));
    assert_eq!(meets_youth_service_method(&one_short), Some(false));
}

#[test]
fn a_phase_milestone_is_met_from_the_target_phase_on() {
    let at_target: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_phase_lesson("13")]).unwrap();
    let one_short: ProgressAssessment =
        candidate_on(Instrument::Flute, &[method_phase_lesson("12")]).unwrap();

    assert_eq!(meets_youth_service_method(&at_target), Some(true));
    assert_eq!(meets_youth_service_method(&one_short), Some(false));
    assert!(one_short.method_relative_percent > 0.0);
}

#[test]
fn a_page_and_lesson_milestone_needs_both_targets_reached() {
    let both: ProgressAssessment =
        candidate_on(Instrument::Cello, &[method_lesson("34", "80")]).unwrap();
    let page_only: ProgressAssessment =
        candidate_on(Instrument::Cello, &[method_lesson("34", "79")]).unwrap();
    let lesson_only: ProgressAssessment =
        candidate_on(Instrument::Cello, &[method_lesson("33", "80")]).unwrap();

    assert_eq!(meets_youth_service_method(&both), Some(true));
    assert_eq!(
        meets_youth_service_method(&page_only),
        Some(false),
        "the page alone is not enough"
    );
    assert_eq!(
        meets_youth_service_method(&lesson_only),
        Some(false),
        "the lesson alone is not enough"
    );
}

#[test]
fn milestones_that_the_lessons_cannot_measure_are_never_met_and_add_no_progress() {
    let far_along: [Lesson; 1] = [method_lesson("999", "999")];

    let complete: ProgressAssessment = candidate_on(Instrument::Trumpet, &far_along).unwrap();

    assert_eq!(meets_youth_service_method(&complete), Some(false));
    assert!(
        is_about(complete.method_relative_percent, 0.0),
        "got {}",
        complete.method_relative_percent
    );

    let module: ProgressAssessment =
        candidate_on(Instrument::Bassoon, &[method_lesson("0", "999")]).unwrap();
    assert_eq!(meets_youth_service_method(&module), Some(false));

    let exercise_range: ProgressAssessment = assess(
        &MusicianLevel::YouthService,
        Instrument::Trumpet,
        &[msa_lesson("16", "16")],
        &[method_lesson("0", "999")],
    )
    .unwrap();

    assert!(
        !checkpoint(&exercise_range, &MusicianLevel::OfficialService)
            .unwrap()
            .requirement
            .method_met
    );
}

#[test]
fn an_alternative_is_met_only_when_every_component_is() {
    let both: ProgressAssessment =
        candidate_on(Instrument::Viola, &[method_lesson("31", "6")]).unwrap();
    let page_short: ProgressAssessment =
        candidate_on(Instrument::Viola, &[method_lesson("30", "6")]).unwrap();
    let lesson_short: ProgressAssessment =
        candidate_on(Instrument::Viola, &[method_lesson("31", "5")]).unwrap();

    assert_eq!(meets_youth_service_method(&both), Some(true));
    assert_eq!(meets_youth_service_method(&page_short), Some(false));
    assert_eq!(meets_youth_service_method(&lesson_short), Some(false));
}

#[test]
fn an_alternative_with_an_unmeasurable_component_is_never_met_but_its_measurable_part_counts() {
    let assessment: ProgressAssessment =
        candidate_on(Instrument::FrenchHorn, &[method_lesson("0", "73")]).unwrap();

    assert_eq!(meets_youth_service_method(&assessment), Some(false));
    assert!(
        is_about(assessment.method_relative_percent, 100.0),
        "got {}",
        assessment.method_relative_percent
    );
}

#[test]
fn an_alternatives_progress_is_the_average_of_its_measurable_components() {
    let assessment: ProgressAssessment =
        candidate_on(Instrument::Viola, &[method_lesson("0", "3")]).unwrap();

    assert!(
        is_about(assessment.method_relative_percent, 25.0),
        "got {}",
        assessment.method_relative_percent
    );
}
fn candidate_on(
    instrument: Instrument,
    method: &[Lesson],
) -> Result<ProgressAssessment, AssessError> {
    assess(&MusicianLevel::Candidate, instrument, &[], method)
}

fn meets_youth_service_method(assessment: &ProgressAssessment) -> Option<bool> {
    checkpoint(assessment, &MusicianLevel::YouthService)
        .map(|youth_service| youth_service.requirement.method_met)
}

fn is_about(actual: f64, expected: f64) -> bool {
    (actual - expected).abs() < 1e-9
}
