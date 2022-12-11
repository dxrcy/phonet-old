use std::{collections::HashMap, fmt::Display};

use fancy_regex::Regex;

use super::{Patterns, Tests};
use ParseError::*;

/// Error enum for `Scheme`
pub enum ParseError {
  UnknownIntentIdentifier(char),
  UnknownLineOperator(char),
  UnknownClass(char),
  RegexFail(fancy_regex::Error),
}

impl Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UnknownIntentIdentifier(ch) => write!(
        f,
        "Unknown intent identifier `{ch}`. Must be either `+` or `!`"
      ),
      UnknownLineOperator(ch) => write!(f, "Unknown line operator `{ch}`"),
      UnknownClass(name) => write!(f, "Unknown class `{name}`"),
      RegexFail(err) => write!(f, "Failed to parse Regex: {err}"),
    }
  }
}

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

//TODO Rename this
impl Scheme {
  /// Parse `Scheme` from file
  pub fn parse(file: &str) -> Result<Scheme, ParseError> {
    // Builders
    let mut classes = Classes::new();
    let mut tests = Tests::new();

    let mut patterns_raw = Vec::new();
    let mut pattern_reason: Option<String> = None;
    let mut is_useful_reason = false;

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
            // Use '@@' for reason used by multiple patterns
            if Some('@') == chars.next() {
              chars.next();
              is_useful_reason = true;
            } else {
              is_useful_reason = false;
            }

            // Set reason
            pattern_reason = Some(chars.as_str().trim().to_string());
            continue;
          }

          // Patterns
          '&' => {
            // Check intent
            // `+` for true, `!` for false
            let intent = match chars.next() {
              // Should be INVALID to pass
              Some('+') => true,
              // Should be VALID to pass
              Some('!') => false,

              // Unknown character
              Some(ch) => {
                return Err(UnknownIntentIdentifier(ch));
                // return Err(format!(
                //   "Unknown intent identifier `{ch}`. Must be either `+` or `!`"
                // ))
              }
              // No character
              None => continue,
            };

            // Add pattern
            patterns_raw.push((
              intent,
              chars.as_str().replace(" ", ""),
              pattern_reason.clone(),
            ));

            // Use '@@' for reason used by multiple patterns
            if !is_useful_reason {
              pattern_reason = None;
            }
          }

          // Tests
          '*' => {
            // Check intent
            // `+` for true, `!` for false
            let intent = match chars.next() {
              // Should be INVALID to pass
              Some('+') => true,
              // Should be VALID to pass
              Some('!') => false,

              // Unknown character
              Some(ch) => {
                return Err(UnknownIntentIdentifier(ch));
                // return Err(format!(
                //   "Unknown intent identifier `{ch}`. Must be either `+` or `!`"
                // ))
              }
              // No character
              None => continue,
            };

            // Add test
            let word = chars.as_str().trim().to_string();
            if !word.is_empty() {
              tests.push((intent, word))
            }
          }

          // Unknown
          _ => return Err(UnknownLineOperator(first)),
          // _ => return Err(format!("Unknown line operator `{first}`")),
        }
      }
    }

    // Substitute classes in patterns
    let mut patterns = Patterns::new();
    for (intent, pattern, reason) in patterns_raw {
      let re = match Regex::new(&substitute_classes(&pattern, &classes)?) {
        Ok(x) => x,
        Err(err) => return Err(RegexFail(err)),
      };

      patterns.push((intent, re, reason));
    }

    Ok(Scheme { patterns, tests })
  }
}

/// Substitute class names regex pattern with class values
fn substitute_classes(pattern: &str, classes: &Classes) -> Result<String, ParseError> {
  let mut new = pattern.to_string();
  for ch in pattern.chars() {
    // Replace class with value if exists
    if ch.is_uppercase() {
      // Return error if class does not exist
      let value = match classes.get(&ch) {
        Some(x) => x,
        None => return Err(UnknownClass(ch)),
        // None => return Err(format!("Unknown class `{ch}`")),
      };

      // Replace name with value (surrounded in round brackets to separate from rest of pattern)
      new = new.replace(ch, &format!("({})", value));
    }
  }
  Ok(new)
}
