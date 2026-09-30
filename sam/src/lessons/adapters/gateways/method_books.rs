use student::domain::entities::MethodBook;

pub fn recognize_method_books(sam_name: &str) -> Vec<MethodBook> {
    let normalized: String = sam_name.split_whitespace().collect::<Vec<&str>>().join(" ");

    match normalized.as_str() {
        "NICOLAS LAOUREUX VOL. I - VIOLINO" => vec![MethodBook::Laoureux1],
        "NICOLAS LAOUREUX VOL. III - VIOLINO" => vec![MethodBook::Laoureux3],
        "MÉTODO CCB - SCHIMOLL - VIOLINO" | "2° SCHMOLL" | "SCHMOLL" => vec![MethodBook::Ccb],
        "HANS SITT - VIOLINO" => vec![MethodBook::HansSitt1],
        "ED. BRITTEN - MÉTODO FACILITADO - VIOLINO - VOLUME 1" => vec![MethodBook::BrittenViolin1],
        "BEGINNING STRINGS - VIOLA" => vec![MethodBook::BeginningStringsViola],
        "A TUNE A DAY - VIOLA" => vec![MethodBook::ATuneADay3],
        "ED. BRITTEN - MÉTODO FACILITADO - VIOLA - VOLUME 1" => vec![MethodBook::BrittenViola1],
        "BEGINNING STRINGS - VIOLONCELLO" => vec![MethodBook::BeginningStringsCello],
        "DOTZAUER - VIOLONCELLO - VOLUME 1" => vec![MethodBook::Dotzauer1],
        "DOTZAUER" => vec![MethodBook::Dotzauer1, MethodBook::Dotzauer2],
        "ED. BRITTEN - MÉTODO FACILITADO - VIOLONCELO - VOLUME 1" => {
            vec![MethodBook::BrittenCello1]
        }
        "PARÉS - FLAUTA" => vec![MethodBook::Pares],
        "GALLI - FLAUTA" => vec![MethodBook::Galli],
        "ALMEIDA DIAS - MÉTODO FACILITADO - FLAUTA" => vec![MethodBook::AlmeidaDiasFlute],
        "GIAMPIERI - OBOÉ , OBOÉ D'AMORE , CORNE INGLES" => vec![MethodBook::GiampieriOboe],
        "GIAMPIERI - FAGOTE" => vec![MethodBook::GiampieriBassoon],
        "WEISSENBORN - FAGOTE" => vec![MethodBook::Weissenborn],
        "GIAMPIERI - CLARINETE SIB / LA" => vec![MethodBook::GiampieriClarinet],
        "GALPER - CLARINETE SIB / LA" => {
            vec![MethodBook::GalperClarinet1, MethodBook::GalperClarinet2]
        }
        "NABOR PIRES CAMARGO - CLARINETE SIB / LA" => vec![MethodBook::NaborPiresCamargo],
        "GALPER - CLARINETE ALTO MIB / CLARINETE BAIXO SIB" => {
            vec![MethodBook::GalperAltoBass1, MethodBook::GalperAltoBass2]
        }
        "AMADEU RUSSO - PADRONIZAÇÃO DE ENSINO - SAXOFONES SOPRANO, ALTO, TENOR, BARÍTONO" => {
            vec![MethodBook::AmadeuRussoSaxophone]
        }
        "ALMEIDA DIAS - MÉTODO PRÁTICO - SAXOFONES SOPRANO, ALTO, TENOR, BARÍTONO" => {
            vec![MethodBook::AlmeidaDiasSaxophone]
        }
        "RUBANK - TROMPETE, CORNET, FLUGELHORN" => vec![MethodBook::RubankTrumpet],
        "AMADEU RUSSO - TROMPETE, CORNET, FLUGELHORN, TROMBONE, EUPHONIO, BARÍTONO DE PISTO" => {
            vec![MethodBook::AmadeuRussoBrass]
        }
        "ALMEIDA DIAS - MÉTODO PRÁTICO - TROMPETE, CORNET, FLUGELHORN" => {
            vec![MethodBook::AlmeidaDiasTrumpet]
        }
        "RUBANK ELEMENTARY- TROMPA FA / SIB" => vec![MethodBook::RubankElementaryHorn],
        "ALMEIDA DIAS - MÉTODO PRÁTICO - TROMPA FA / SIB" => vec![MethodBook::AlmeidaDiasHorn],
        "RUBANK - TROMBONE, TROMBONITO, BARÍTONO, EUPHONIUM" => vec![MethodBook::RubankTrombone],
        "ALMEIDA DIAS - MÉTODO PRÁTICO - TROMBONE, TROMBONITO, BARÍTONO, EUPHONIUM" => {
            vec![MethodBook::AlmeidaDiasTrombone]
        }
        "ALMEIDA DIAS - MÉTODO PRÁTICO - TUBA" => vec![MethodBook::AlmeidaDiasTuba],
        _ => Vec::new(),
    }
}
