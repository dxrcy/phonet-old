use crate::{
  args::DisplayLevel::{self, *},
  scheme::Phoner,
  Reason::{self, *},
  ResultType, Rules, TestType,
};
use Validity::*;

/// Results from `run_tests` function
pub struct TestResults {
  /// List of results of each test
  list: Vec<ResultType>,
  /// Amount of failed tests
  fail_count: u32,
  /// Length of longest word in tests
  /// TODO Fix with DisplayLevel -- will increase len for passing test, even if not displayed
  max_word_len: usize,
}

impl TestResults {
  /// Create empty `TestResults`
  fn empty() -> Self {
    TestResults {
      list: Vec::new(),
      fail_count: 0,
      max_word_len: 0,
    }
  }

  /// Run tests, return results
  pub fn run(scheme: Phoner) -> TestResults {
    // No tests
    if scheme.tests.len() < 1 {
      return TestResults::empty();
    }

    // Builders
    let mut list = vec![];
    let mut fail_count = 0;
    let mut max_word_len = 0;

    // Loop tests
    for test in scheme.tests {
      match test {
        // Note - simply add to list
        TestType::Note(note) => list.push(ResultType::Note(note)),

        // Test - Validate test, check validity with intent, create reason for failure
        TestType::Test { intent, word } => {
          // Validate test
          let reason = validate_test(&word, &scheme.rules);

          // Check if validity status with test intent
          let pass = !(reason.is_valid() ^ intent);

          // Create reason
          let reason = if !pass {
            // Test failed - Some reason
            match reason {
              // Test was valid, but it should have not matched
              Valid => ShouldNotHaveMatched,

              // Test was invalid, but it should have matched
              Invalid(reason) => match reason {
                // No reason was given for rule
                None => NoReasonGiven,

                // Find rule reason in scheme
                Some(reason) => match scheme.reasons.get(reason) {
                  // Rule found - Custom reason
                  Some(x) => Reason::Custom(x.to_string()),
                  // No rule found
                  // ? this should not happen ever ?
                  None => NoReasonGiven,
                },
              },
            }
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
          list.push(ResultType::Test {
            intent,
            word,
            pass,
            reason,
          });
        }
      }
    }

    TestResults {
      list,
      fail_count,
      max_word_len,
    }
  }

  /// Display results to standard output
  ///
  /// This can be implemented manually
  pub fn display(&self, display_level: DisplayLevel) {
    // No tests
    if self.list.len() < 1 {
      println!("\n\x1b[33mNo tests to run.\x1b[0m");
      return;
    }

    // Header
    println!("\n\x1b[3;33mRunning {} tests...\x1b[0m", self.list.len());

    // Loop result list
    let mut is_first_print = true;
    for item in &self.list {
      match item {
        // Display note
        ResultType::Note(note) => match display_level {
          // Always show
          ShowAll | NotesAndFails => {
            // Blank line for first print
            if is_first_print {
              println!();
              is_first_print = false;
            }

            // Print note
            println!("\x1b[34m{note}\x1b[0m")
          }
          // Else skip
          _ => (),
        },

        // Display test
        ResultType::Test {
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
            ShouldNotHaveMatched => "\x1b[33mMatched, but should have not\x1b[0m",
            NoReasonGiven => "No reason given",
            Custom(reason) => &reason,
          };

          // Blank line for first print
          if is_first_print {
            println!();
            is_first_print = false;
          }

          // Display test status
          println!(
            "  \x1b[{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{reason}\x1b[0m",
            intent = if *intent { "36m✔" } else { "35m✗" },
            space = " ".repeat(self.max_word_len - word.len()),
            result = if *pass { "32mpass" } else { "31mFAIL" },
          );
        }
      }
    }

    // Blank line if there was tests or notes displayed
    if !is_first_print {
      println!();
    }

    // Final print
    if self.fail_count == 0 {
      // All passed
      println!("\x1b[32;1;3mAll tests pass!\x1b[0m");
    } else {
      // Some failed
      println!(
        "\x1b[31;1;3m{fails} test{s} failed!\x1b[0m",
        fails = self.fail_count,
        s = if self.fail_count == 1 { "" } else { "s" },
      );
    }
  }
}

/// State of rules match of word
///
/// If invalid, reason can be provided
enum Validity {
  Valid,
  Invalid(Option<usize>),
}

impl Validity {
  /// Returns `true` if state is `Valid`
  pub fn is_valid(&self) -> bool {
    if let Valid = self {
      return true;
    }
    false
  }

  /// Unwrap reason with default
  ///
  /// Replaces reference to reason with value
  pub fn _unwrap_or<'a>(
    self,
    if_valid: &'a str,
    if_none: &'a str,
    reasons: &'a Vec<String>,
  ) -> &'a str {
    if let Invalid(reason_ref) = self {
      return match reason_ref {
        Some(reason) => match reasons.get(reason) {
          Some(x) => x,
          None => if_none,
        },
        None => if_none,
      };
    }
    if_valid
  }
}

/// Check if string is valid with rules
fn validate_test(word: &str, rules: &Rules) -> Validity {
  // Check for match with every rule, if not, return reason
  for (should_match, rule, reason_ref) in rules {
    // Check if rule matches, and whether match signifies returning invalid or continuing
    if should_match
      ^ rule
        .is_match(word)
        // ? Why is this a result ?
        //TODO Fix this
        .expect("Failed checking match. This error should have been fixed :(")
    {
      return Invalid(*reason_ref);
    }
  }

  Valid
}
