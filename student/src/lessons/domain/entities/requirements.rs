use crate::shared::domain::entities::{Instrument, MusicianLevel};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MethodMilestone {
    Page(u32),
    Lesson(u32),
    PageAndLesson { page: u32, lesson: u32 },
    Phase(u32),
    Module(u32),
    ExerciseRange { from: u32, to: u32 },
    Complete,
    Described(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodComponent {
    pub method_name: &'static str,
    pub milestone: MethodMilestone,
}

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
    pub method_alternatives: Vec<MethodAlternative>,
    pub theory: TheoryRequirement,
    pub metric_reading: &'static str,
    pub hymnal: &'static str,
    pub observation: Option<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentRequirements {
    pub instrument: Instrument,
    pub tests: Vec<TestRequirement>,
}

impl InstrumentRequirements {
    #[must_use]
    pub fn requirement_for(&self, level: &MusicianLevel) -> Option<&TestRequirement> {
        self.tests.iter().find(|t| &t.level == level)
    }

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
