use std::collections::HashMap;

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
