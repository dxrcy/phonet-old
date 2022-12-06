use regex::Regex;

use std::collections::HashMap;

use ValidState::*;

/// Run tests, output result
pub fn run_tests(
  tests: &[(&str, bool)],
  patterns: &[(bool, &str, &'static str)],
  classes: &HashMap<char, &str>,
) {
  println!("\n\x1b[3;33mRunning {} tests...\x1b[0m\n", tests.len());

  // Get max length of all words
  let max_word_len = tests.iter().map(|x| x.0.len()).max().unwrap_or(0);

  // Test each word, tally fails
  let mut fails = 0;
  for (word, intent) in tests {
    // Validate word against patterns, get reason for invalid
    let reason = validate(word, patterns, classes);

    // Check if test result matches intended result
    let passed = !(reason.is_valid() ^ intent);

    // Define reason for test fail
    let fail_reason = if !passed {
      reason.unwrap_or("Should NOT have matched")
    } else {
      ""
    };

    // Output single result
    println!(
      "  \x1b[33m{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{fail_reason}\x1b[0m",
      result = if passed { "32mpass" } else { "31mFAIL" },
      intent = if intent == &true { "✔" } else { "✗" },
      space = " ".repeat(max_word_len - word.len()),
    );

    // Increase fails tally if failed
    if !passed {
      fails += 1;
    }
  }

  // Output final result
  if fails == 0 {
    println!("\n\x1b[32;3mAll tests pass!\x1b[0m");
  } else {
    println!("\n\x1b[31;1;3m{fails} tests failed!\x1b[0m");
  }
}

/// State of pattern match of word
///
/// If invalid, reason must be provided
enum ValidState {
  Valid,
  Invalid(&'static str),
}

impl ValidState {
  /// Returns `true` if state is `Valid`
  pub fn is_valid(&self) -> bool {
    if let Valid = self {
      return true;
    }
    false
  }

  /// Unwrap reason with default
  pub fn unwrap_or(&self, default: &'static str) -> &'static str {
    if let Invalid(reason) = self {
      return reason;
    }
    default
  }
}

/// Format regex pattern with components
///
/// TODO Recursive class unfolding - break loop and repeat on replace
fn format(pattern: &str, classes: &HashMap<char, &str>) -> String {
  let mut new = pattern.to_string();
  for ch in pattern.chars() {
    // Replace class with value if exists
    if ch.is_uppercase() {
      new = new.replace(
        ch,
        classes.get(&ch).expect(&format!("Unknown class '{ch}'")),
      );
    }
  }

  new
}

/// Check if string is valid with patterns
fn validate(
  word: &str,
  patterns: &[(bool, &str, &'static str)],
  classes: &HashMap<char, &str>,
) -> ValidState {
  // Check for match with every pattern, if not, return reason
  for (should_match, pattern, reason) in patterns {
    let re = Regex::new(&format(pattern, classes).replace(" ", "")).expect("Could not parse regex");
    // Check if pattern matches, and whether match signifies returning invalid or continuing
    if should_match ^ re.is_match(word) {
      return Invalid(reason);
    }
  }

  Valid
}
