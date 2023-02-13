use phonet::{self, Phonet};

#[test]
fn tokipona_should_pass() {
    let file = include_str!("../examples/tokipona.phonet");

    assert_eq!(
        Phonet::parse(file)
            .expect("Failed to parse")
            .run()
            .fail_count,
        0,
        "Phonet tests should have passed, but failed"
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
        2,
        "Fail count did not equal 2"
    );
}

#[test]
fn no_tests_to_run() {
    assert_eq!(
        Phonet::parse("@* Some note; !x")
            .expect("Failed to parse")
            .test_count(),
        0,
        "Note counted as test, in scheme test count"
    );

    assert_eq!(
        Phonet::parse("@* Some note; !x")
            .expect("Failed to parse")
            .run()
            .test_count(),
        0,
        "Note counted as test, in result test count"
    );
}

#[test]
fn invalid_syntax() {
    assert!(
        Phonet::parse(")").is_err(),
        "Syntax should not have parsed, but did"
    );
    assert!(
        Phonet::parse("~@@").is_err(),
        "Syntax should not have parsed, but did"
    );
    assert!(
        Phonet::parse("+ (").is_err(),
        "Syntax should not have parsed, but did"
    );
}
