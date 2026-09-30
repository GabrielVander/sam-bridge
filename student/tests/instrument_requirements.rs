use pretty_assertions::assert_eq;
use student::domain::entities::{
    Instrument, InstrumentRequirements, Lesson, MethodBook, MusicianLevel, ProgressAssessment,
    TestRequirement, assess,
};

#[path = "support/helpers.rs"]
mod support;
use support::{checkpoint, method_lesson_in, msa_lesson};

use MethodBook::*;
use MusicianLevel::{OfficialService, Officialized, YouthService};

#[test]
fn the_youth_meeting_needs_msa_phase_12_and_official_service_phase_16() {
    assert_eq!(theory_met(&YouthService, "12"), Some(true));
    assert_eq!(theory_met(&YouthService, "11"), Some(false));
    assert_eq!(theory_met(&OfficialService, "16"), Some(true));
    assert_eq!(theory_met(&OfficialService, "15"), Some(false));
    assert_eq!(theory_met(&Officialized, "16"), Some(true));
    assert_eq!(theory_met(&Officialized, "15"), Some(false));
}

#[test]
fn violin_thresholds() {
    assert_thresholds(
        &Instrument::Violin,
        &[
            (
                YouthService,
                &[at(Laoureux1, "35", "0")],
                &[at(Laoureux1, "34", "0")],
            ),
            (
                YouthService,
                &[at(Ccb, "46", "113"), at(HansSitt1, "0", "6")],
                &[at(Ccb, "46", "112"), at(HansSitt1, "0", "6")],
            ),
            (
                YouthService,
                &[at(BrittenViolin1, "40", "0")],
                &[at(BrittenViolin1, "39", "0")],
            ),
            (
                OfficialService,
                &[at(Ccb, "67", "162"), at(HansSitt1, "0", "14")],
                &[at(Ccb, "66", "162"), at(HansSitt1, "0", "14")],
            ),
            (
                OfficialService,
                &[at(BrittenViolin1, "55", "0")],
                &[at(BrittenViolin1, "54", "0")],
            ),
        ],
    );
    assert_never_met(
        &Instrument::Violin,
        &Officialized,
        &[Laoureux1, Laoureux3, Ccb, HansSitt1, BrittenViolin1],
    );
}

#[test]
fn viola_thresholds() {
    assert_thresholds(
        &Instrument::Viola,
        &[
            (
                YouthService,
                &[
                    at(BeginningStringsViola, "0", "6"),
                    at(BertaVolmer1, "31", "0"),
                ],
                &[
                    at(BeginningStringsViola, "0", "6"),
                    at(BertaVolmer1, "30", "0"),
                ],
            ),
            (
                YouthService,
                &[at(BrittenViola1, "40", "0")],
                &[at(BrittenViola1, "39", "0")],
            ),
            (
                OfficialService,
                &[at(BertaVolmer1, "62", "0"), at(ATuneADay3, "16", "0")],
                &[at(BertaVolmer1, "62", "0"), at(ATuneADay3, "15", "0")],
            ),
            (
                OfficialService,
                &[at(BrittenViola1, "55", "0")],
                &[at(BrittenViola1, "54", "0")],
            ),
        ],
    );
    assert_never_met(
        &Instrument::Viola,
        &Officialized,
        &[BertaVolmer1, ATuneADay3, BrittenViola1],
    );
}

#[test]
fn cello_thresholds() {
    assert_thresholds(
        &Instrument::Cello,
        &[
            (
                YouthService,
                &[
                    at(BeginningStringsCello, "0", "6"),
                    at(Dotzauer1, "34", "80"),
                ],
                &[
                    at(BeginningStringsCello, "0", "5"),
                    at(Dotzauer1, "34", "80"),
                ],
            ),
            (
                YouthService,
                &[at(BrittenCello1, "40", "0")],
                &[at(BrittenCello1, "39", "0")],
            ),
            (
                OfficialService,
                &[at(BrittenCello1, "52", "0")],
                &[at(BrittenCello1, "51", "0")],
            ),
        ],
    );
    assert_never_met(
        &Instrument::Cello,
        &Officialized,
        &[Dotzauer1, Dotzauer2, BrittenCello1],
    );
}

#[test]
fn flute_thresholds() {
    assert_thresholds(
        &Instrument::Flute,
        &[
            (
                YouthService,
                &[at(Pares, "0", "41")],
                &[at(Pares, "0", "40")],
            ),
            (
                YouthService,
                &[at(Galli, "41", "0")],
                &[at(Galli, "40", "0")],
            ),
            (
                OfficialService,
                &[at(Pares, "0", "62")],
                &[at(Pares, "0", "61")],
            ),
        ],
    );
    assert_never_met(
        &Instrument::Flute,
        &Officialized,
        &[Pares, Galli, AlmeidaDiasFlute],
    );
}

#[test]
fn oboe_thresholds() {
    assert_thresholds(
        &Instrument::Oboe,
        &[
            (
                YouthService,
                &[at(GiampieriOboe, "21", "0")],
                &[at(GiampieriOboe, "20", "0")],
            ),
            (
                OfficialService,
                &[at(RubankOboe2, "16", "0")],
                &[at(RubankOboe2, "15", "0")],
            ),
            (
                OfficialService,
                &[at(GiampieriOboe, "30", "0")],
                &[at(GiampieriOboe, "29", "0")],
            ),
            (
                Officialized,
                &[at(RubankOboe2, "30", "0")],
                &[at(RubankOboe2, "29", "0")],
            ),
            (
                Officialized,
                &[at(GiampieriOboe, "50", "0")],
                &[at(GiampieriOboe, "49", "0")],
            ),
        ],
    );
}

#[test]
fn bassoon_thresholds() {
    assert_thresholds(
        &Instrument::Bassoon,
        &[
            (
                YouthService,
                &[at(GiampieriBassoon, "18", "0")],
                &[at(GiampieriBassoon, "17", "0")],
            ),
            (
                OfficialService,
                &[at(GiampieriBassoon, "26", "0")],
                &[at(GiampieriBassoon, "25", "0")],
            ),
            (
                Officialized,
                &[at(GiampieriBassoon, "43", "0")],
                &[at(GiampieriBassoon, "42", "0")],
            ),
        ],
    );
}

#[test]
fn clarinet_thresholds() {
    assert_thresholds(
        &Instrument::Clarinet,
        &[
            (
                YouthService,
                &[at(GiampieriClarinet, "28", "0")],
                &[at(GiampieriClarinet, "27", "0")],
            ),
            (
                YouthService,
                &[at(DomingosPecci, "29", "0")],
                &[at(DomingosPecci, "28", "0")],
            ),
            (
                OfficialService,
                &[at(GiampieriClarinet, "41", "0")],
                &[at(GiampieriClarinet, "40", "0")],
            ),
            (
                OfficialService,
                &[at(DomingosPecci, "36", "0")],
                &[at(DomingosPecci, "35", "0")],
            ),
            (
                OfficialService,
                &[at(NaborPiresCamargo, "0", "36")],
                &[at(NaborPiresCamargo, "0", "35")],
            ),
            (
                Officialized,
                &[at(GiampieriClarinet, "63", "0")],
                &[at(GiampieriClarinet, "62", "0")],
            ),
        ],
    );
}

#[test]
fn bass_clarinet_thresholds() {
    assert_thresholds(
        &Instrument::BassClarinet,
        &[
            (
                YouthService,
                &[at(GiampieriClarinet, "28", "0")],
                &[at(GiampieriClarinet, "27", "0")],
            ),
            (
                OfficialService,
                &[at(GiampieriClarinet, "36", "0")],
                &[at(GiampieriClarinet, "35", "0")],
            ),
        ],
    );
    assert_never_met(
        &Instrument::BassClarinet,
        &Officialized,
        &[GiampieriClarinet, GalperAltoBass1, GalperAltoBass2],
    );
}

#[test]
fn saxophone_thresholds() {
    assert_thresholds(
        &Instrument::AltoSaxophone,
        &[
            (
                YouthService,
                &[at(GiampieriSaxophone, "21", "0")],
                &[at(GiampieriSaxophone, "20", "0")],
            ),
            (
                YouthService,
                &[at(AmadeuRussoSaxophone, "25", "0")],
                &[at(AmadeuRussoSaxophone, "24", "0")],
            ),
            (
                OfficialService,
                &[at(AmadeuRussoSaxophone, "40", "0")],
                &[at(AmadeuRussoSaxophone, "39", "0")],
            ),
            (
                Officialized,
                &[at(GiampieriSaxophone, "50", "0")],
                &[at(GiampieriSaxophone, "49", "0")],
            ),
            (
                Officialized,
                &[at(AmadeuRussoSaxophone, "55", "0")],
                &[at(AmadeuRussoSaxophone, "54", "0")],
            ),
        ],
    );
}

#[test]
fn trumpet_thresholds() {
    assert_never_met(&Instrument::Trumpet, &YouthService, &[RubankTrumpet]);
    assert_thresholds(
        &Instrument::Trumpet,
        &[
            (
                OfficialService,
                &[at(AmadeuRussoBrass, "30", "0")],
                &[at(AmadeuRussoBrass, "29", "0")],
            ),
            (
                Officialized,
                &[at(AmadeuRussoBrass, "41", "0")],
                &[at(AmadeuRussoBrass, "40", "0")],
            ),
        ],
    );
}

#[test]
fn french_horn_needs_a_completed_book_that_cannot_be_verified() {
    assert_never_met(
        &Instrument::FrenchHorn,
        &YouthService,
        &[RubankElementaryHorn, AlmeidaDiasHorn],
    );
}

#[test]
fn trombone_and_euphonium_thresholds() {
    for instrument in [Instrument::Trombone, Instrument::Euphonium] {
        assert_thresholds(
            &instrument,
            &[
                (
                    YouthService,
                    &[at(RubankTrombone, "24", "0")],
                    &[at(RubankTrombone, "23", "0")],
                ),
                (
                    OfficialService,
                    &[at(RubankTrombone, "37", "0")],
                    &[at(RubankTrombone, "36", "0")],
                ),
                (
                    Officialized,
                    &[at(RubankTrombone, "48", "0")],
                    &[at(RubankTrombone, "47", "0")],
                ),
            ],
        );
    }
}

#[test]
fn tuba_thresholds() {
    assert_thresholds(
        &Instrument::Tuba,
        &[
            (
                YouthService,
                &[at(RubankTuba, "24", "0")],
                &[at(RubankTuba, "23", "0")],
            ),
            (
                OfficialService,
                &[at(RubankTuba, "37", "0")],
                &[at(RubankTuba, "36", "0")],
            ),
            (
                Officialized,
                &[at(RubankTuba, "48", "0")],
                &[at(RubankTuba, "47", "0")],
            ),
        ],
    );
}

#[test]
fn every_saxophone_voice_is_held_to_the_saxophone_sheet() {
    for voice in [
        Instrument::CurvedSopranoSaxophone,
        Instrument::StraightSopranoSaxophone,
        Instrument::TenorSaxophone,
    ] {
        assert_eq!(
            tests_for(&voice),
            tests_for(&Instrument::AltoSaxophone),
            "{voice:?}"
        );
    }
}

#[test]
fn cornet_and_flugelhorn_are_held_to_the_trumpet_sheet() {
    for relative in [Instrument::Cornet, Instrument::Flugelhorn] {
        assert_eq!(
            tests_for(&relative),
            tests_for(&Instrument::Trumpet),
            "{relative:?}"
        );
    }
}

#[test]
fn instruments_without_a_published_sheet_have_no_requirements() {
    for instrument in [
        Instrument::AltoClarinet,
        Instrument::EnglishHorn,
        Instrument::ContraltoViolin,
        Instrument::Unknown("BANDOLIM".to_owned()),
    ] {
        assert_eq!(
            InstrumentRequirements::for_instrument(&instrument),
            None,
            "{instrument:?}"
        );
    }
}

#[test]
fn requirement_for_unpublished_level_is_none() {
    let requirements: Option<InstrumentRequirements> =
        InstrumentRequirements::for_instrument(&Instrument::Violin);

    assert_eq!(
        requirements.and_then(|requirements| requirements
            .requirement_for(&MusicianLevel::Candidate)
            .cloned()),
        None
    );
}

type Threshold<'a> = (MusicianLevel, &'a [Lesson], &'a [Lesson]);

fn assert_thresholds(instrument: &Instrument, thresholds: &[Threshold]) {
    for (level, reached, one_short) in thresholds {
        assert_eq!(
            method_met(instrument, level, reached),
            Some(true),
            "{instrument:?} {level:?} should be met by {reached:?}"
        );
        assert_eq!(
            method_met(instrument, level, one_short),
            Some(false),
            "{instrument:?} {level:?} should not be met by {one_short:?}"
        );
    }
}

fn assert_never_met(instrument: &Instrument, level: &MusicianLevel, books: &[MethodBook]) {
    let far_along: Vec<Lesson> = books.iter().map(|book| at(*book, "999", "999")).collect();

    assert_eq!(
        method_met(instrument, level, &far_along),
        Some(false),
        "{instrument:?} {level:?} cannot be verified from the lessons"
    );
}

fn method_met(instrument: &Instrument, level: &MusicianLevel, lessons: &[Lesson]) -> Option<bool> {
    let assessment: ProgressAssessment =
        assess(&MusicianLevel::Candidate, instrument.clone(), &[], lessons).ok()?;

    checkpoint(&assessment, level).map(|checkpoint| checkpoint.requirement.method_met)
}

fn theory_met(level: &MusicianLevel, phase: &str) -> Option<bool> {
    let assessment: ProgressAssessment = assess(
        &MusicianLevel::Candidate,
        Instrument::Violin,
        &[msa_lesson(phase, phase)],
        &[],
    )
    .ok()?;

    checkpoint(&assessment, level).map(|checkpoint| checkpoint.requirement.msa_met)
}

fn at(book: MethodBook, page: &str, lesson: &str) -> Lesson {
    method_lesson_in(book, page, lesson)
}

fn tests_for(instrument: &Instrument) -> Option<Vec<TestRequirement>> {
    InstrumentRequirements::for_instrument(instrument).map(|requirements| requirements.tests)
}
