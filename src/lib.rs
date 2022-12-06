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

/// State of pattern match of word
///
/// If invalid, reason can be provided
enum ValidState {
  Valid,
  Invalid(Option<String>),
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

/// Check if string is valid with patterns
fn validate(word: &str, patterns: &Patterns) -> ValidState {
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

type Classes = HashMap<char, String>;
type Patterns = Vec<(bool, String, Option<String>)>;
type Tests = Vec<(bool, String)>;

#[derive(Debug)]
pub struct Scheme {
  patterns: Patterns,
  tests: Tests,
}

pub fn parse(file: &str) -> Result<Scheme, String> {
  let mut classes = Classes::new();
  let mut patterns = Patterns::new();
  let mut pattern_reason: Option<String> = None;
  let mut tests = Tests::new();

  let mut section: u32 = 0;

  for line in file.lines() {
    let line = line.trim();

    if line.starts_with("###") {
      section += 1;
    }

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
        if line.starts_with('@') {
          pattern_reason = Some(remove_first_char(line).trim().to_string());
          continue;
        }

        if line.starts_with('!') {
          patterns.push((
            false,
            remove_first_char(&line).replace(" ", ""),
            pattern_reason,
          ));
        } else {
          patterns.push((true, line.replace(" ", ""), pattern_reason));
        }

        pattern_reason = None;
      }

      // Tests
      2 => {
        let mut chars = line.chars();

        let intent = match chars.next() {
          Some('0') => false,
          Some('1') => true,
          Some(ch) => {
            return Err(format!(
              "Unknown test intent `{ch}`. Must be `0` (invalid) or `1` (valid)"
            ))
          }
          None => continue,
        };

        tests.push((intent, chars.as_str().trim().to_string()));
      }

      _ => return Err("Unknown section. There should only be 3 sections".to_string()),
    }
  }

  for i in &mut patterns {
    i.1 = format(&i.1, &classes);
  }

  Ok(Scheme { patterns, tests })
}

/// Format regex pattern with components
///
/// TODO Recursive class unfolding - break loop and repeat on replace
fn format(pattern: &str, classes: &Classes) -> String {
  let mut new = pattern.to_string();
  for ch in pattern.chars() {
    // Replace class with value if exists
    if ch.is_uppercase() {
      let value = classes.get(&ch).expect(&format!("Unknown class '{ch}'"));
      new = new.replace(ch, &format!("({})", value));
    }
  }

  new
}

/// Remove first character of string slice
fn remove_first_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}
