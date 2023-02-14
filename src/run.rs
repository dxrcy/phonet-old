use crate::{
    types::{Rule, TestDefinition, TestResult},
    DisplayLevel::{self, *},
    Phonet,
};
use FailReason::*;
use ValidStatus::*;

/// Results from run tests
///
/// Create with `PhonetResults::run()`
pub struct Results {
    /// List of results of each test
    pub list: Vec<TestResult>,
    /// Amount of failed tests
    pub fail_count: u32,
}

impl Results {
    /// Run tests, return results
    pub fn run(scheme: &Phonet) -> Results {
        // No tests
        if scheme.tests.is_empty() {
            return Results {
                list: Vec::new(),
                fail_count: 0,
            };
        }

        // Builders
        let mut list = vec![];
        let mut fail_count = 0;
        let mut max_word_len = 0;

        // Loop tests
        for test in &scheme.tests {
            match test {
                // Note - simply add to list
                TestDefinition::Note(note) => list.push(TestResult::Note(note.to_string())),

                // Test - Validate test, check validity with intent, create reason for failure
                TestDefinition::Test { intent, word } => {
                    // Validate test
                    let validity = validate_test(word, &scheme.rules);

                    // Check if validity status with test intent
                    let pass = !(validity.is_valid() ^ intent);

                    // Create reason
                    let reason = if !pass {
                        // Test failed - Some reason
                        FailReason::from(validity, &scheme.reasons)
                    } else {
                        // Test passed - No reason for failure needed
                        Passed
                    };

                    // Increase fail count if failed
                    if !pass {
                        fail_count += 1;
                    }

                    // Increase max length if word is longer than current max
                    if word.len() > max_word_len {
                        max_word_len = word.len();
                    }

                    // Add test result to list
                    list.push(TestResult::Test {
                        intent: *intent,
                        word: word.to_string(),
                        pass,
                        reason,
                    });
                }
            }
        }

        Results { list, fail_count }
    }

    /// Get maximum length of all test words
    fn max_word_len(&self, display_level: DisplayLevel) -> usize {
        self.list
            .iter()
            .map(|x| match x {
                // Test - Check display level
                TestResult::Test { word, pass, .. } => match display_level {
                    // Always include
                    ShowAll => word.len(),
                    // Only include if failed
                    NotesAndFails | JustFails if !pass => word.len(),
                    // Don't include
                    _ => 0,
                },

                TestResult::Note(_) => 0,
            })
            .max()
            // Default value
            .unwrap_or(10)
    }

    /// Get count of tests in list
    pub fn test_count(&self) -> usize {
        self.list
            .iter()
            .filter(|item| matches!(item, TestResult::Test { .. }))
            .count()
    }

    /// Display results to standard output
    ///
    /// This can be implemented manually
    pub fn display(&self, display_level: DisplayLevel, no_color: bool) {
        // No tests
        if self.test_count() == 0 {
            if no_color {
                println!("No tests ran.");
            } else {
                println!("\x1b[33mNo tests ran.\x1b[0m");
            }
            return;
        }

        // Get maximum length of all test words
        let max_word_len = self.max_word_len(display_level);

        // Loop result list
        for item in &self.list {
            match item {
                // Display note
                TestResult::Note(note) => match display_level {
                    // Always show - Print note
                    ShowAll | NotesAndFails => {
                        if no_color {
                            println!("{note}")
                        } else {
                            println!("\x1b[34m{note}\x1b[0m")
                        }
                    }

                    // Else skip
                    _ => (),
                },

                // Display test
                TestResult::Test {
                    intent,
                    word,
                    pass,
                    reason,
                } => {
                    // Skip if not required by display level
                    if match display_level {
                        // Always show
                        ShowAll => false,
                        // Only show if failed
                        NotesAndFails | JustFails if !pass => false,
                        // Else skip
                        _ => true,
                    } {
                        continue;
                    }

                    // Format reason
                    let reason = match &reason {
                        Passed => "",
                        ShouldBeInvalid => {
                            if no_color {
                                "Valid, but should be invalid"
                            } else {
                                "\x1b[33mValid, but should be invalid\x1b[0m"
                            }
                        }
                        NoReasonGiven => "No reason given",
                        Custom(reason) => reason,
                    };

                    // Display test status
                    if no_color {
                        println!(
                            " {intent} {word}{space}  {result} {reason}",
                            intent = if *intent { "✔" } else { "✗" },
                            space = " ".repeat(max_word_len - word.chars().count()),
                            result = if *pass { "pass" } else { "FAIL" },
                        );
                    } else {
                        println!(
              "  \x1b[{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{reason}\x1b[0m",
              intent = if *intent { "36m✔" } else { "35m✗" },
              space = " ".repeat(max_word_len - word.chars().count()),
              result = if *pass { "32mpass" } else { "31mFAIL" },
            );
                    }
                }
            }
        }

        // Final print
        if self.fail_count == 0 {
            // All passed
            if no_color {
                println!("All tests pass!");
            } else {
                println!("\x1b[32;1;3mAll tests pass!\x1b[0m");
            }
        } else {
            // Some failed
            if no_color {
                println!(
                    "{fails} test{s} failed!",
                    fails = self.fail_count,
                    s = if self.fail_count == 1 { "" } else { "s" },
                );
            } else {
                println!(
                    "\x1b[31;1;3m{fails} test{s} failed!\x1b[0m",
                    fails = self.fail_count,
                    s = if self.fail_count == 1 { "" } else { "s" },
                );
            }
        }
    }
}

/// Reason for failure variants
pub enum FailReason {
    /// Test passed, do not display reason
    Passed,
    /// No reason was given for rule for test failing
    NoReasonGiven,
    /// Test was valid, but should have been invalid
    ShouldBeInvalid,
    /// Custom reason for rule
    Custom(String),
}

impl FailReason {
    fn from(validity: ValidStatus, reasons: &[String]) -> Self {
        match validity {
            // Test was valid, but it should have been invalid
            Valid => ShouldBeInvalid,

            // Test was invalid, but it should have been valid
            Invalid(reason) => match reason {
                // No reason was given for rule
                None => NoReasonGiven,

                // Find rule reason in scheme
                Some(reason) => match reasons.get(reason) {
                    // Rule found - Custom reason
                    Some(x) => FailReason::Custom(x.to_string()),
                    // No rule found
                    // ? this should not happen ever ?
                    None => NoReasonGiven,
                },
            },
        }
    }
}

/// State of rules match of word
///
/// If invalid, reason reference can be provided
pub enum ValidStatus {
    /// String matches
    Valid,
    /// String does not match
    Invalid(Option<usize>),
}

impl ValidStatus {
    /// Returns `true` if state is `Valid`
    pub fn is_valid(&self) -> bool {
        if let Valid = self {
            return true;
        }
        false
    }
}

/// Check if string is valid with rules
pub fn validate_test(word: &str, rules: &Vec<Rule>) -> ValidStatus {
    // Check for match with every rule, if not, return reason
    for Rule {
        intent,
        pattern,
        reason_ref,
    } in rules
    {
        // Check if rule matches, and whether match signifies returning invalid or continuing
        if intent
            ^ pattern
                .is_match(word)
                .expect("Failed checking regex match. This error should NEVER APPEAR!")
        {
            return Invalid(*reason_ref);
        }
    }

    Valid
}
