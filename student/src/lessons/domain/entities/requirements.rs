use crate::lessons::domain::entities::MethodBook;
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
    Unmeasured,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodComponent {
    pub book: MethodBook,
    pub milestone: MethodMilestone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodAlternative {
    pub components: Vec<MethodComponent>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TheoryRequirement {
    pub msa_phase: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestRequirement {
    pub level: MusicianLevel,
    pub method_alternatives: Vec<MethodAlternative>,
    pub theory: TheoryRequirement,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstrumentRequirements {
    pub tests: Vec<TestRequirement>,
}

impl InstrumentRequirements {
    #[must_use]
    pub fn requirement_for(&self, level: &MusicianLevel) -> Option<&TestRequirement> {
        self.tests.iter().find(|test| &test.level == level)
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
            Instrument::AltoSaxophone
            | Instrument::CurvedSopranoSaxophone
            | Instrument::StraightSopranoSaxophone
            | Instrument::TenorSaxophone => Some(Self::saxophone()),
            Instrument::Trumpet | Instrument::Cornet | Instrument::Flugelhorn => {
                Some(Self::trumpet())
            }
            Instrument::FrenchHorn => Some(Self::french_horn()),
            Instrument::Trombone | Instrument::Euphonium => Some(Self::trombone_or_euphonium()),
            Instrument::Tuba => Some(Self::tuba()),
            Instrument::AltoClarinet
            | Instrument::EnglishHorn
            | Instrument::ContraltoViolin
            | Instrument::Unknown(_) => None,
        }
    }

    fn violin() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::Laoureux1, page(35))]),
                        alt(vec![
                            comp(MethodBook::Ccb, page_and_lesson(46, 113)),
                            comp(MethodBook::HansSitt1, lesson(6)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViolin1, page(40))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp(MethodBook::Laoureux1, complete()),
                            comp(MethodBook::Laoureux3, page(15)),
                        ]),
                        alt(vec![
                            comp(MethodBook::Ccb, page_and_lesson(67, 162)),
                            comp(MethodBook::HansSitt1, lesson(14)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViolin1, page(55))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp(MethodBook::Laoureux1, complete()),
                            comp(MethodBook::Laoureux3, unmeasured()),
                        ]),
                        alt(vec![
                            comp(MethodBook::Ccb, complete()),
                            comp(MethodBook::HansSitt1, complete()),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViolin1, complete())]),
                    ],
                ),
            ],
        }
    }

    fn viola() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![
                            comp(MethodBook::BeginningStringsViola, lesson(6)),
                            comp(MethodBook::BertaVolmer1, page(31)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViola1, page(40))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp(MethodBook::BertaVolmer1, page(62)),
                            comp(MethodBook::ATuneADay3, page(16)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViola1, page(55))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp(MethodBook::BertaVolmer1, complete()),
                            comp(MethodBook::ATuneADay3, complete()),
                        ]),
                        alt(vec![comp(MethodBook::BrittenViola1, complete())]),
                    ],
                ),
            ],
        }
    }

    fn cello() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![
                            comp(MethodBook::BeginningStringsCello, lesson(6)),
                            comp(MethodBook::Dotzauer1, page_and_lesson(34, 80)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenCello1, page(40))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![
                            comp(MethodBook::Dotzauer1, complete()),
                            comp(MethodBook::Dotzauer2, page_and_lesson(3, 111)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenCello1, page(52))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![
                            comp(MethodBook::Dotzauer1, complete()),
                            comp(MethodBook::Dotzauer2, page_and_lesson(19, 154)),
                        ]),
                        alt(vec![comp(MethodBook::BrittenCello1, complete())]),
                    ],
                ),
            ],
        }
    }

    fn flute() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::Pares, lesson(41))]),
                        alt(vec![comp(MethodBook::Galli, page(41))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasFlute, phase(13))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::Pares, lesson(62))]),
                        alt(vec![comp(MethodBook::Galli, complete())]),
                        alt(vec![comp(MethodBook::AlmeidaDiasFlute, phase(25))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::Pares, complete())]),
                        alt(vec![comp(MethodBook::Galli, complete())]),
                        alt(vec![comp(MethodBook::AlmeidaDiasFlute, complete())]),
                    ],
                ),
            ],
        }
    }

    fn oboe() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::RubankOboe1, complete())]),
                        alt(vec![comp(MethodBook::GiampieriOboe, page(21))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::RubankOboe2, page(16))]),
                        alt(vec![comp(MethodBook::GiampieriOboe, page(30))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::RubankOboe2, page(30))]),
                        alt(vec![comp(MethodBook::GiampieriOboe, page(50))]),
                    ],
                ),
            ],
        }
    }

    fn bassoon() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriBassoon, page(18))]),
                        alt(vec![comp(MethodBook::Weissenborn, module(12))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriBassoon, page(26))]),
                        alt(vec![comp(MethodBook::Weissenborn, module(18))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriBassoon, page(43))]),
                        alt(vec![comp(MethodBook::Weissenborn, module(22))]),
                    ],
                ),
            ],
        }
    }

    fn clarinet() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, page(28))]),
                        alt(vec![comp(MethodBook::DomingosPecci, page(29))]),
                        alt(vec![comp(MethodBook::GalperClarinet1, unmeasured())]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, page(41))]),
                        alt(vec![comp(MethodBook::DomingosPecci, page(36))]),
                        alt(vec![comp(MethodBook::NaborPiresCamargo, lesson(36))]),
                        alt(vec![
                            comp(MethodBook::GalperClarinet1, complete()),
                            comp(MethodBook::GalperClarinet2, page(18)),
                        ]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, page(63))]),
                        alt(vec![comp(MethodBook::DomingosPecci, complete())]),
                        alt(vec![comp(MethodBook::NaborPiresCamargo, complete())]),
                        alt(vec![
                            comp(MethodBook::GalperClarinet1, complete()),
                            comp(MethodBook::GalperClarinet2, page(29)),
                        ]),
                    ],
                ),
            ],
        }
    }

    fn bass_clarinet() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, page(28))]),
                        alt(vec![comp(MethodBook::GalperAltoBass1, unmeasured())]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, page(36))]),
                        alt(vec![
                            comp(MethodBook::GalperAltoBass1, complete()),
                            comp(MethodBook::GalperAltoBass2, page(18)),
                        ]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriClarinet, complete())]),
                        alt(vec![
                            comp(MethodBook::GalperAltoBass1, complete()),
                            comp(MethodBook::GalperAltoBass2, page(29)),
                        ]),
                    ],
                ),
            ],
        }
    }

    fn saxophone() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriSaxophone, page(21))]),
                        alt(vec![comp(MethodBook::AmadeuRussoSaxophone, page(25))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasSaxophone, phase(13))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriSaxophone, page(30))]),
                        alt(vec![comp(MethodBook::AmadeuRussoSaxophone, page(40))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasSaxophone, phase(25))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::GiampieriSaxophone, page(50))]),
                        alt(vec![comp(MethodBook::AmadeuRussoSaxophone, page(55))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasSaxophone, complete())]),
                    ],
                ),
            ],
        }
    }

    fn trumpet() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![alt(vec![comp(MethodBook::RubankTrumpet, complete())])],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::Getchel2, exercise_range(65, 94))]),
                        alt(vec![comp(MethodBook::AmadeuRussoBrass, page(30))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTrumpet, phase(25))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::Getchel2, complete())]),
                        alt(vec![comp(MethodBook::AmadeuRussoBrass, page(41))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTrumpet, complete())]),
                    ],
                ),
            ],
        }
    }

    fn french_horn() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![alt(vec![
                        comp(MethodBook::RubankElementaryHorn, complete()),
                        comp(MethodBook::AlmeidaDiasHorn, lesson(73)),
                    ])],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![alt(vec![
                        comp(MethodBook::RubankElementaryHorn, complete()),
                        comp(MethodBook::RubankIntermediateHorn, complete()),
                        comp(MethodBook::AlmeidaDiasHorn, lesson(105)),
                    ])],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![alt(vec![
                        comp(MethodBook::RubankElementaryHorn, complete()),
                        comp(MethodBook::RubankIntermediateHorn, complete()),
                        comp(MethodBook::AlmeidaDiasHorn, complete()),
                    ])],
                ),
            ],
        }
    }

    fn trombone_or_euphonium() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::RubankTrombone, page(24))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTrombone, phase(13))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::RubankTrombone, page(37))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTrombone, phase(25))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::RubankTrombone, page(48))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTrombone, complete())]),
                    ],
                ),
            ],
        }
    }

    fn tuba() -> Self {
        Self {
            tests: vec![
                test(
                    MusicianLevel::YouthService,
                    vec![
                        alt(vec![comp(MethodBook::RubankTuba, page(24))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTuba, phase(13))]),
                    ],
                ),
                test(
                    MusicianLevel::OfficialService,
                    vec![
                        alt(vec![comp(MethodBook::RubankTuba, page(37))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTuba, phase(25))]),
                    ],
                ),
                test(
                    MusicianLevel::Officialized,
                    vec![
                        alt(vec![comp(MethodBook::RubankTuba, page(48))]),
                        alt(vec![comp(MethodBook::AlmeidaDiasTuba, complete())]),
                    ],
                ),
            ],
        }
    }
}

const fn theory_for(level: &MusicianLevel) -> TheoryRequirement {
    match level {
        MusicianLevel::Officialized | MusicianLevel::OfficialService => {
            TheoryRequirement { msa_phase: 16 }
        }
        _ => TheoryRequirement { msa_phase: 12 },
    }
}

const fn test(
    level: MusicianLevel,
    method_alternatives: Vec<MethodAlternative>,
) -> TestRequirement {
    TestRequirement {
        theory: theory_for(&level),
        method_alternatives,
        level,
    }
}

const fn alt(components: Vec<MethodComponent>) -> MethodAlternative {
    MethodAlternative { components }
}

const fn comp(book: MethodBook, milestone: MethodMilestone) -> MethodComponent {
    MethodComponent { book, milestone }
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

const fn unmeasured() -> MethodMilestone {
    MethodMilestone::Unmeasured
}
