use std::collections::HashMap;

use regex::Regex;

use super::{Patterns, Tests};

/// Alias for hashmap of class name and value
type Classes = HashMap<char, String>;

#[derive(Debug)]
/// Scheme parsed from file
///
/// Holds patterns and tests
pub struct Scheme {
  pub patterns: Patterns,
  pub tests: Tests,
}

impl Scheme {
  /// Parse `Scheme` from file
  pub fn parse(file: &str) -> Result<Scheme, String> {
    // Builders
    let mut classes = Classes::new();
    let mut patterns_raw = Vec::new();
    let mut pattern_reason: Option<String> = None;
    let mut tests = Tests::new();

    for line in file.lines() {
      let line = line.trim();

      // Continue for blank
      if line.is_empty() {
        continue;
      }

      let mut chars = line.chars();

      if let Some(first) = chars.next() {
        match first {
          // Comment
          '#' => continue,

          // Classes
          '$' => {
            if let Some(name) = chars.next() {
              let value = chars.as_str().trim();
              classes.insert(name, value.to_string());
            }
          }

          // Define pattern reason
          '@' => {
            pattern_reason = Some(chars.as_str().trim().to_string());
            continue;
          }

          // Patterns
          '&' => {
            // Bang inverts match intent
            if chars.as_str().starts_with('!') {
              chars.next();
              // Should NOT match
              patterns_raw.push((false, chars.as_str().replace(" ", ""), pattern_reason));
            } else {
              // Should match
              patterns_raw.push((true, chars.as_str().replace(" ", ""), pattern_reason));
            }
            pattern_reason = None;
          }

          // Tests
          '*' => {
            // Bang inverts validity intent
            if chars.as_str().starts_with('!') {
              chars.next();
              // Should be INVALID to pass
              tests.push((false, chars.as_str().trim().to_string()));
            } else {
              // Should be VALID to pass
              tests.push((true, chars.as_str().trim().to_string()));
            }
          }

          // Unknown
          _ => return Err(format!("Unknown line operator `{first}`")),
        }
      }
    }

    // Substitute classes in patterns
    let mut patterns = Patterns::new();
    for (intent, pattern, reason) in patterns_raw {
      let re = match Regex::new(&substitute_classes(&pattern, &classes)?) {
        Ok(x) => x,
        Err(err) => return Err(err.to_string()),
      };

      patterns.push((intent, re, reason));
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
fn _remove_first_char<'a>(s: &'a str) -> &'a str {
  let mut chars = s.chars();
  chars.next();
  chars.as_str()
}
