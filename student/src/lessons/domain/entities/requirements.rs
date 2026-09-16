//! Test-eligibility requirements per instrument and level, as published by the
//! music commission (Comissão Musical, Formulário M09 - Março/2023). This is
//! reference data: which methods (and how far into them) a student must have
//! completed, together with the shared MSA/theory, metric-reading and hymnal
//! requirements, before they may sit a given level's test.
use crate::shared::domain::entities::{Instrument, MusicianLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodMilestone {
    Page(u32),
    Lesson(u32),
    PageAndLesson {
        page: u32,
        lesson: u32,
    },
    Phase(u32),
    Module(u32),
    ExerciseRange {
        from: u32,
        to: u32,
    },
    Complete,
    /// Escape hatch for milestones that don't fit a clean numeric shape
    /// (e.g. "até pág. 24 e da pág. 44 a 53"), kept verbatim from the sheet.
    Described(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodComponent {
    /// A human-readable label sourced from Formulário M09, kept short and
    /// scoped to one instrument on purpose.
    ///
    /// It is *not* meant to be matched against SAM's own free-text `método`
    /// field on a recorded lesson: a wide exploratory sample (~200 real
    /// lesson records across every instrument, see
    /// `discovers_method_names_actually_used_per_instrument` in
    /// `sam/tests/sam_http_capabilities_and_behaviour.rs`) showed that field
    /// is one shared, unscoped dropdown - the same generic entries (and even
    /// a music-theory book) turned up recorded under unrelated instruments
    /// far too often to be occasional instrument-switch history. `assess`
    /// deliberately never reads `Lesson.method`; this field exists purely
    /// for display.
    pub method_name: &'static str,
    pub milestone: MethodMilestone,
}

/// One acceptable path to satisfy a level's method requirement.
///
/// All of its components must be completed together (they are joined by "+"
/// on the sheet); different alternatives for the same requirement are joined
/// by "OU" and any single one suffices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodAlternative {
    pub components: Vec<MethodComponent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheoryRequirement {
    pub msa_phase: u32,
    pub note: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestRequirement {
    pub level: MusicianLevel,
    /// Any one of these alternatives satisfies the instrument-method part of
    /// the requirement.
    pub method_alternatives: Vec<MethodAlternative>,
    pub theory: TheoryRequirement,
    pub metric_reading: &'static str,
    pub hymnal: &'static str,
    /// Free-form observation from the sheet (e.g. clef/voice notes).
    pub observation: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentRequirements {
    pub instrument: Instrument,
    /// One entry per testable level (`YouthService`, `OfficialService`,
    /// `Officialized`). Candidate/Practice have no test to sit.
    pub tests: Vec<TestRequirement>,
}

impl InstrumentRequirements {
    #[must_use]
    pub fn requirement_for(&self, level: &MusicianLevel) -> Option<&TestRequirement> {
        self.tests.iter().find(|t| &t.level == level)
    }

    /// Looks up the published requirements for an instrument. Returns `None`
    /// when the commission has not yet published requirements for it (e.g.
    /// Alto Clarinet, English Horn or Contralto Violin, as of Formulário
    /// M09).
    ///
    /// Note for whoever revisits `EnglishHorn`: a real SAM method-lesson
    /// sample showed Oboe's own method book listed as "GIAMPIERI - OBOÉ,
    /// OBOÉ D'AMORE, CORNE INGLES", i.e. it's also used to teach English
    /// Horn in practice. That's a fact about which *book* is shared, not
    /// about *test eligibility* - Formulário M09 has no footnote extending
    /// Oboe's testing track to English Horn the way it explicitly does for
    /// Trumpet/Cornet/Flugelhorn, so this intentionally still returns `None`
    /// pending an actual policy confirmation from the music commission.
    #[must_use]
    pub fn for_instrument(instrument: &Instrument) -> Option<Self> {
        match instrument {
            Instrument::Violin => Some(Self::violin()),
            Instrument::Viola => Some(Self::viola()),
            Instrument::Cello => Some(Self::cello()),
            Instrument::Flute => Some(Self::flute()),
            Instrument::Oboe => Some(Self::oboe()),
            Instrument::Bassoon => Some(Self::bassoon()),
            Instrument::Clarinet => Some(Self::clarinet()),
            Instrument::BassClarinet => Some(Self::bass_clarinet()),
            Instrument::Saxophone => Some(Self::saxophone()),
            Instrument::Trumpet => Some(Self::trumpet()),
            Instrument::FrenchHorn => Some(Self::french_horn()),
            Instrument::Trombone => Some(Self::trombone_or_euphonium(Instrument::Trombone)),
            Instrument::Euphonium => Some(Self::trombone_or_euphonium(Instrument::Euphonium)),
            Instrument::Tuba => Some(Self::tuba()),
            Instrument::AltoClarinet
            | Instrument::EnglishHorn
            | Instrument::ContraltoViolin
            | Instrument::Unknown(_) => None,
        }
    }

    #[must_use]
    pub fn violin() -> Self {
        Self {
            instrument: Instrument::Violin,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("N. Laourex Vol. 1", page(35))]),
                        alt(vec![
                            comp("CCB", page_and_lesson(46, 113)),
                            comp("H. Sitt Vol. 1", lesson(6)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(40))]),
                    ],
                    Some("Hinos 431 a 480 soprano no natural"),
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp("N. Laourex Vol. 1", complete()),
                            comp("N. Laourex Vol. 3", page(15)),
                        ]),
                        alt(vec![
                            comp("CCB", page_and_lesson(67, 162)),
                            comp("H. Sitt Vol. 1", lesson(14)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(55))]),
                    ],
                    Some("Hinário completo soprano 8ª acima"),
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp("N. Laourex Vol. 1", complete()),
                            comp(
                                "N. Laourex Vol. 3",
                                described("até pág. 24 e da pág. 44 a 53"),
                            ),
                        ]),
                        alt(vec![
                            comp("Método CCB", complete()),
                            comp("H. Sitt Op. 32 Vol. 1", complete()),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", complete())]),
                    ],
                    Some("Hinário completo soprano 8ª acima e contralto natural"),
                ),
            ],
        }
    }

    #[must_use]
    pub fn viola() -> Self {
        Self {
            instrument: Instrument::Viola,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![
                            comp("Beginning Strings", lesson(6)),
                            comp("Berta Volmer Vol. 1", page(31)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(40))]),
                    ],
                    Some("Hinos 431 a 480 tenor no natural"),
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp("Berta Volmer Vol. 1", page(62)),
                            comp("A Tune a Day (C.P. Herfurth) Vol. 3", page(16)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(55))]),
                    ],
                    Some("Hinário completo tenor no natural"),
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp("Berta Volmer Vol. 1", complete()),
                            comp("A Tune a Day (C.P. Herfurth) Vol. 3", complete()),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", complete())]),
                    ],
                    Some("1ª a 3ª posições, hinário completo, tenor no natural"),
                ),
            ],
        }
    }

    #[must_use]
    pub fn cello() -> Self {
        Self {
            instrument: Instrument::Cello,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![
                            comp("Beginning Strings", lesson(6)),
                            comp("Dotzauer Vol. 1", page_and_lesson(34, 80)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(40))]),
                    ],
                    Some("Hinos 431 a 480 baixo no natural"),
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp("Dotzauer Vol. 1", complete()),
                            comp("Dotzauer Vol. 2", page_and_lesson(3, 111)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", page(52))]),
                    ],
                    Some("Hinário completo baixo no natural"),
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp("Dotzauer Vol. 1", complete()),
                            comp("Dotzauer Vol. 2", page_and_lesson(19, 154)),
                        ]),
                        alt(vec![comp("Método Facilitado (Ed. Britten)", complete())]),
                    ],
                    Some("Hinário completo baixo no natural"),
                ),
            ],
        }
    }

    #[must_use]
    pub fn flute() -> Self {
        Self {
            instrument: Instrument::Flute,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Parès", lesson(41))]),
                        alt(vec![comp("Galli", page(41))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(13))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Parès", lesson(62))]),
                        alt(vec![comp("Galli", complete())]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(25))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Parès", complete())]),
                        alt(vec![comp("Galli", complete())]),
                        alt(vec![comp("Método Prático (Almeida Dias)", complete())]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn oboe() -> Self {
        Self {
            instrument: Instrument::Oboe,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Rubank Vol. 1", complete())]),
                        alt(vec![comp("Giampieri (ou similar)", page(21))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Rubank Vol. 2", page(16))]),
                        alt(vec![comp("Giampieri (ou similar)", page(30))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Rubank Vol. 2", page(30))]),
                        alt(vec![comp("Giampieri (ou similar)", page(50))]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn bassoon() -> Self {
        Self {
            instrument: Instrument::Bassoon,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Giampieri", page(18))]),
                        alt(vec![comp("Weissenborn (ou similar)", module(12))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Giampieri", page(26))]),
                        alt(vec![comp("Weissenborn (ou similar)", module(18))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Giampieri", page(43))]),
                        alt(vec![comp("Weissenborn (ou similar)", module(22))]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn clarinet() -> Self {
        Self {
            instrument: Instrument::Clarinet,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Giampieri", page(28))]),
                        alt(vec![comp("Domingos Pecci", page(29))]),
                        alt(vec![comp(
                            "Galper Book 1",
                            described("Lição 26 - até exercício 110"),
                        )]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Giampieri", page(41))]),
                        alt(vec![comp("Domingos Pecci", page(36))]),
                        alt(vec![comp("Nabor Pires Camargo", lesson(36))]),
                        alt(vec![
                            comp("Galper Book 1", complete()),
                            comp("Galper Book 2", page(18)),
                        ]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Giampieri", page(63))]),
                        alt(vec![comp("Domingos Pecci", complete())]),
                        alt(vec![comp("Nabor Pires Camargo", complete())]),
                        alt(vec![
                            comp("Galper Book 1", complete()),
                            comp("Galper Book 2", page(29)),
                        ]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn bass_clarinet() -> Self {
        Self {
            instrument: Instrument::BassClarinet,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Giampieri", page(28))]),
                        alt(vec![comp(
                            "Galper Book 1",
                            described("Lição 26 - até exercício 110"),
                        )]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Giampieri", page(36))]),
                        alt(vec![
                            comp("Galper Book 1", complete()),
                            comp("Galper Book 2", page(18)),
                        ]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Giampieri", complete())]),
                        alt(vec![
                            comp("Galper Book 1", complete()),
                            comp("Galper Book 2", page(29)),
                        ]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn saxophone() -> Self {
        Self {
            instrument: Instrument::Saxophone,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Giampieri", page(21))]),
                        alt(vec![comp("Amadeu Russo", page(25))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(13))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Giampieri", page(30))]),
                        alt(vec![comp("Amadeu Russo", page(40))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(25))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Giampieri", page(50))]),
                        alt(vec![comp("Amadeu Russo", page(55))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", complete())]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn trumpet() -> Self {
        Self {
            instrument: Instrument::Trumpet,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![alt(vec![comp(
                        "Rubank Elementary Method for Cornet or Trumpet",
                        complete(),
                    )])],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(
                            "Robert W. Getchel - Second Book of Practical Studies",
                            exercise_range(65, 94),
                        )]),
                        alt(vec![comp("Amadeu Russo", page(30))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(25))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(
                            "Robert W. Getchel - Second Book of Practical Studies",
                            complete(),
                        )]),
                        alt(vec![comp("Amadeu Russo", page(41))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", complete())]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn french_horn() -> Self {
        Self {
            instrument: Instrument::FrenchHorn,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![alt(vec![
                        comp("Rubank Elementary", complete()),
                        comp("Método Prático para Trompa", lesson(73)),
                    ])],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![alt(vec![
                        comp("Rubank Elementary", complete()),
                        comp("Rubank Intermediate", complete()),
                        comp("Método Prático para Trompa", lesson(105)),
                    ])],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![alt(vec![
                        comp("Rubank Elementary", complete()),
                        comp("Rubank Intermediate", complete()),
                        comp("Método Prático para Trompa", complete()),
                    ])],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn trombone_or_euphonium(instrument: Instrument) -> Self {
        Self {
            instrument,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Rubank Elementary for Trombone", page(24))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(13))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Rubank Elementary for Trombone", page(37))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(25))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Rubank Elementary for Trombone", page(48))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", complete())]),
                    ],
                    None,
                ),
            ],
        }
    }

    #[must_use]
    pub fn tuba() -> Self {
        Self {
            instrument: Instrument::Tuba,
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp("Rubank Elementary for Tuba", page(24))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(13))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp("Rubank Elementary for Tuba", page(37))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", phase(25))]),
                    ],
                    None,
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp("Rubank Elementary for Tuba", page(48))]),
                        alt(vec![comp("Método Prático (Almeida Dias)", complete())]),
                    ],
                    None,
                ),
            ],
        }
    }
}

const fn theory_for(level: &MusicianLevel) -> TheoryRequirement {
    match level {
        MusicianLevel::Officialized => TheoryRequirement {
            msa_phase: 16,
            note: Some("Com repasse na leitura métrica a partir da lição 56"),
        },
        MusicianLevel::OfficialService => TheoryRequirement {
            msa_phase: 16,
            note: None,
        },
        _ => TheoryRequirement {
            msa_phase: 12,
            note: None,
        },
    }
}

const fn metric_reading_for(level: &MusicianLevel) -> &'static str {
    match level {
        MusicianLevel::YouthService => "Hinos 431 a 480",
        _ => "Todos os Hinos",
    }
}

const fn hymnal_for(level: &MusicianLevel) -> &'static str {
    match level {
        MusicianLevel::YouthService => "431 a 480 - Voz principal + Voz alternativa",
        _ => "Completo - Voz principal + Voz alternativa",
    }
}

const fn test(
    level: MusicianLevel,
    method_alternatives: Vec<MethodAlternative>,
    observation: Option<&'static str>,
) -> TestRequirement {
    TestRequirement {
        theory: theory_for(&level),
        metric_reading: metric_reading_for(&level),
        hymnal: hymnal_for(&level),
        method_alternatives,
        observation,
        level,
    }
}

const fn alt(components: Vec<MethodComponent>) -> MethodAlternative {
    MethodAlternative { components }
}

const fn comp(method_name: &'static str, milestone: MethodMilestone) -> MethodComponent {
    MethodComponent {
        method_name,
        milestone,
    }
}

const fn page(n: u32) -> MethodMilestone {
    MethodMilestone::Page(n)
}

const fn lesson(n: u32) -> MethodMilestone {
    MethodMilestone::Lesson(n)
}

const fn page_and_lesson(page: u32, lesson: u32) -> MethodMilestone {
    MethodMilestone::PageAndLesson { page, lesson }
}

const fn phase(n: u32) -> MethodMilestone {
    MethodMilestone::Phase(n)
}

const fn module(n: u32) -> MethodMilestone {
    MethodMilestone::Module(n)
}

const fn exercise_range(from: u32, to: u32) -> MethodMilestone {
    MethodMilestone::ExerciseRange { from, to }
}

const fn complete() -> MethodMilestone {
    MethodMilestone::Complete
}

const fn described(text: &'static str) -> MethodMilestone {
    MethodMilestone::Described(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn violin_matches_known_youth_and_official_thresholds() {
        let requirements = InstrumentRequirements::for_instrument(&Instrument::Violin)
            .expect("violin requirements are published");

        let youth = requirements
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

        let official = requirements
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

        let officialized = requirements
            .requirement_for(&MusicianLevel::Officialized)
            .expect("officialization requirement exists");
        assert_eq!(officialized.theory.msa_phase, 16);
        assert_eq!(
            officialized.theory.note,
            Some("Com repasse na leitura métrica a partir da lição 56")
        );
    }

    #[test]
    fn french_horn_has_a_single_mandatory_combined_alternative() {
        let requirements = InstrumentRequirements::for_instrument(&Instrument::FrenchHorn)
            .expect("french horn requirements are published");
        let youth = requirements
            .requirement_for(&MusicianLevel::YouthService)
            .expect("youth service requirement exists");

        assert_eq!(youth.method_alternatives.len(), 1);
        assert_eq!(youth.method_alternatives[0].components.len(), 2);
    }

    #[test]
    fn trombone_and_euphonium_share_the_same_requirement_shape() {
        let trombone = InstrumentRequirements::for_instrument(&Instrument::Trombone)
            .expect("trombone requirements are published");
        let euphonium = InstrumentRequirements::for_instrument(&Instrument::Euphonium)
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
        let requirements = InstrumentRequirements::for_instrument(&Instrument::Violin)
            .expect("violin requirements are published");
        assert_eq!(
            requirements.requirement_for(&MusicianLevel::Candidate),
            None
        );
    }
}
