use regex::Regex;

use std::collections::HashMap;

use Validity::*;

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
    let reason = validate(&word, &scheme.patterns);

    // Check if test result matches intended result
    let passed = !(reason.is_valid() ^ intent);

    // Define reason for test fail
    let fail_reason = if !passed {
      reason.unwrap_or(
        //TODO Change these
        "\x1b[33mMatched when it should have not\x1b[0m".to_string(),
        "No reason given".to_string(),
      )
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

/// Check if string is valid with patterns
fn validate(word: &str, patterns: &Patterns) -> Validity {
  // Check for match with every pattern, if not, return reason
  for (should_match, pattern, reason) in patterns {
    let re = Regex::new(&pattern.replace(" ", "")).expect("Could not parse regex");
    // Check if pattern matches, and whether match signifies returning invalid or continuing
    if should_match ^ re.is_match(word) {
      return Invalid(reason.clone());
    }
  }

  Valid
}

/// State of pattern match of word
///
/// If invalid, reason can be provided
enum Validity {
  Valid,
  Invalid(Option<String>),
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
  pub fn unwrap_or(self, if_valid: String, if_none: String) -> String {
    if let Invalid(reason) = self {
      return match reason {
        Some(reason) => reason,
        None => if_none,
      };
    }
    if_valid
  }
}

/// Alias for hashmap of class name and value
type Classes = HashMap<char, String>;
/// Alias for vector of patterns (intent, expression, and invalidity reason)
type Patterns = Vec<(bool, String, Option<String>)>;
/// Alias for vector of tests (intent and value)
type Tests = Vec<(bool, String)>;

#[derive(Debug)]
/// Scheme parsed from file
///
/// Holds patterns and tests
pub struct Scheme {
  patterns: Patterns,
  tests: Tests,
}

impl Scheme {
  /// Parse `Scheme` from file
  pub fn parse(file: &str) -> Result<Scheme, String> {
    // Builders
    let mut classes = Classes::new();
    let mut patterns = Patterns::new();
    let mut pattern_reason: Option<String> = None;
    let mut tests = Tests::new();

    let mut section: u8 = 0; // Should be between 0-2
    for line in file.lines() {
      let line = line.trim();

      // Section divider
      if line.starts_with("###") {
        section += 1;
      }

      // Continue for blank or comment line
      if line.is_empty() || line.starts_with('#') {
        continue;
      }

      match section {
        // Classes
        0 => {
          let mut chars = line.chars();
          if let Some(name) = chars.next() {
            let value = chars.as_str().trim();
            classes.insert(name, value.to_string());
          }
        }

        // Patterns
        1 => {
          // Define reason
          if line.starts_with('@') {
            pattern_reason = Some(remove_first_char(line).trim().to_string());
            continue;
          }

          // Bang inverts match intent
          if line.starts_with('!') {
            // Should NOT match
            patterns.push((
              false,
              remove_first_char(&line).replace(" ", ""),
              pattern_reason,
            ));
          } else {
            // Should match
            patterns.push((true, line.replace(" ", ""), pattern_reason));
          }
          pattern_reason = None;
        }

        // Tests
        2 => {
          // Bang inverts validity intent
          if line.starts_with('!') {
            // Should be INVALID to pass
            tests.push((false, remove_first_char(line).trim().to_string()));
          } else {
            // Should be VALID to pass
            tests.push((true, line.to_string()));
          }
        }

        // Unknown section
        _ => return Err("Unknown section. There should only be 3 sections".to_string()),
      }
    }

    // Substitute classes in patterns
    for i in &mut patterns {
      i.1 = substitute_classes(&i.1, &classes)?;
    }

    Ok(Scheme { patterns, tests })
  }
}

/// Substitute class names regex pattern with class values
///
/// TODO Recursive class unfolding - break loop and repeat on replace
fn substitute_classes(pattern: &str, classes: &Classes) -> Result<String, String> {
  let mut new = pattern.to_string();
  for ch in pattern.chars() {
    // Replace class with value if exists
    if ch.is_uppercase() {
      // Return error if class does not exist
      let value = match classes.get(&ch) {
        Some(x) => x,
        None => return Err(format!("Unknown class `{ch}`")),
      };

      // Replace name with value (surrounded in round brackets to separate from rest of pattern)
      new = new.replace(ch, &format!("({})", value));
    }
  }
  Ok(new)
}

/// Remove first character of string slice
fn remove_first_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}
