use phonet::{self, Phonet};

#[test]
fn tokipona_should_pass() {
    let file = include_str!("../examples/tokipona.phonet");

    assert_eq!(
        Phonet::parse(file)
            .expect("Failed to parse")
            .run()
            .fail_count,
        0
    );
}

#[test]
fn example_should_have_2_fails() {
    let file = include_str!("../examples/example.phonet");

    assert_eq!(
        Phonet::parse(file)
            .expect("Failed to parse")
            .run()
            .fail_count,
        2
    );
}

#[test]
fn invalid_syntax() {
    assert!(Phonet::parse(")").is_err());
    assert!(Phonet::parse("~@@").is_err());
    assert!(Phonet::parse("+ (").is_err());
}
