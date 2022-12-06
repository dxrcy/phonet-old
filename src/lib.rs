use regex::Regex;

use std::collections::HashMap;

use ValidState::*;

/// Run tests, output result
pub fn run_tests(scheme: Scheme) {
  println!(
    "\n\x1b[3;33mRunning {} tests...\x1b[0m\n",
    scheme.tests.len()
  );

  // Get max length of all words
  let max_word_len = scheme.tests.iter().map(|x| x.1.len()).max().unwrap_or(0);

  // Test each word, tally fails
  let mut fails = 0;
  for (intent, word) in scheme.tests {
    // Validate word against patterns, get reason for invalid
    let reason = validate(&word, &scheme.patterns, &scheme.classes);

    // Check if test result matches intended result
    let passed = !(reason.is_valid() ^ intent);

    // Define reason for test fail
    let fail_reason = if !passed {
      reason.unwrap_or("Should NOT have matched".to_string())
    } else {
      String::new()
    };

    // Output single result
    println!(
      "  \x1b[33m{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{fail_reason}\x1b[0m",
      result = if passed { "32mpass" } else { "31mFAIL" },
      intent = if intent { "✔" } else { "✗" },
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
  Invalid(String),
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
  pub fn unwrap_or(self, default: String) -> String {
    if let Invalid(reason) = self {
      return reason;
    }
    default
  }
}

/// Format regex pattern with components
///
/// TODO Recursive class unfolding - break loop and repeat on replace
fn format(pattern: &str, classes: &Classes) -> String {
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
fn validate(word: &str, patterns: &Patterns, classes: &Classes) -> ValidState {
  // Check for match with every pattern, if not, return reason
  for (should_match, pattern, reason) in patterns {
    let re =
      Regex::new(&format(&pattern, classes).replace(" ", "")).expect("Could not parse regex");
    // Check if pattern matches, and whether match signifies returning invalid or continuing
    if should_match ^ re.is_match(word) {
      return Invalid(reason.to_string());
    }
  }

  Valid
}

type Classes = HashMap<char, String>;
type Patterns = Vec<(bool, String, String)>;

pub struct Scheme {
  classes: Classes,
  patterns: Patterns,
  tests: Vec<(bool, String)>,
}

pub fn parse(_file: &str) -> Result<Scheme, String> {
  let classes = common_macros::hash_map! {
    'C' => "(p|b|t|d|k|g|m|n|f|v|s|z|c|w|j|l)".to_string(),
    'V' => "(i|u|e|o|a)".to_string(),
    'S' => "(s|c)".to_string(),
  };

  let patterns = vec![
    (
      false,
      "VSC".to_string(),
      "Invalid syllable structure".to_string(),
    ),
    (
      false,
      "C{3}".to_string(),
      "3 or more consonants sequentially".to_string(),
    ),
    (
      true,
      "^ S? ( C l? V n? )+ $".to_string(),
      "General invalid structure".to_string(),
    ),
  ];

  let tests = vec![
    (true, "tanta".to_string()),
    (true, "panta".to_string()),
    (true, "panka".to_string()),
    (false, "pania".to_string()),
    (true, "spato".to_string()),
    (true, "spato".to_string()),
    (false, "splatlo".to_string()),
    (false, "splanto".to_string()),
    (false, "splasto".to_string()),
    (false, "splantlo".to_string()),
  ];

  Ok(Scheme {
    classes,
    patterns,
    tests,
  })
}
