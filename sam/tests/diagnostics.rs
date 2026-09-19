use sam::diagnostics::error_chain;
use thiserror::Error;

#[derive(Error, Debug)]
#[error("inner failure")]
struct Inner;

#[derive(Error, Debug)]
#[error("outer failure")]
struct Outer {
    #[source]
    source: Inner,
}

#[derive(Error, Debug)]
#[error("{0}")]
struct Message(String);

#[test]
fn error_without_source_renders_only_its_own_message() {
    let rendered: String = error_chain(&Inner);

    assert_eq!(rendered, "inner failure");
}

#[test]
fn error_with_source_renders_every_level_outermost_first() {
    let rendered: String = error_chain(&Outer { source: Inner });

    assert_eq!(rendered, "outer failure: inner failure");
}

#[test]
fn very_long_chains_are_truncated_with_an_ellipsis() {
    let rendered: String = error_chain(&Message("x".repeat(2_000)));

    assert!(rendered.chars().count() <= 500, "got {}", rendered.len());
    assert!(rendered.ends_with('…'));
}

#[test]
fn truncation_never_splits_a_multibyte_character() {
    let rendered: String = error_chain(&Message("ç".repeat(2_000)));

    assert!(rendered.chars().count() <= 500);
    assert!(rendered.starts_with('ç'));
}
