use std::fmt::Display;

use fancy_regex::Regex;

use crate::types::{Classes, Rules, TestDef};

use ParseError::*;

/// Error enum for `Phoner` struct
pub enum ParseError {
  UnknownIntentIdentifier(char),
  UnknownLineOperator(char),
  UnknownClass(char),
  NoClassName,
  NoClassValue,
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
      NoClassName => write!(f, "No class name given"),
      NoClassValue => write!(f, "No class value given"),
      RegexFail(err) => write!(f, "Failed to parse Regex: {err}"),
    }
  }
}

#[derive(Debug)]
/// Scheme parsed from file
///
/// Holds rules and tests
pub struct Phoner {
  pub rules: Rules,
  pub tests: Vec<TestDef>,
  pub reasons: Vec<String>,
}

//TODO Rename this
impl Phoner {
  /// Parse `Scheme` from file
  pub fn parse(file: &str) -> Result<Phoner, ParseError> {
    // Builders
    let mut classes = Classes::new();
    let mut tests = Vec::new();
    let mut rules = Vec::new();

    let mut reasons = Vec::new();
    let mut reason_ref: Option<usize> = None;

    // Split at semicolon or line
    for line in file.replace(";", "\n").lines() {
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

          // Class
          '$' => {
            let mut split = chars.as_str().split("=");

            let name = match split.next() {
              Some(x) => x.trim(),
              None => return Err(ParseError::NoClassName),
            };
            let value = match split.next() {
              Some(x) => x.trim(),
              None => return Err(ParseError::NoClassValue),
            };

            classes.insert(name.to_string(), value.to_string());
          }

          // Rule
          '+' | '!' => {
            // `+` for true, `!` for false
            let intent = first != '!';

            // Add rule
            rules.push((intent, chars.as_str().replace(" ", ""), reason_ref));
          }

          // Test
          '?' => {
            // Remove spaces
            while chars.as_str().starts_with(' ') {
              chars.next();
            }

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
              }
              // No character
              None => continue,
            };

            // Split at space
            let words = chars.as_str().split_whitespace();
            for word in words {
              // Add test
              let word = word.trim().to_string();
              if !word.is_empty() {
                tests.push(TestDef::Test { intent, word });
              }
            }
          }

          // Reason
          '@' => {
            // Remove spaces
            while chars.as_str().starts_with(' ') {
              chars.next();
            }

            // Reason note
            if chars.as_str().starts_with('*') {
              chars.next();
              tests.push(TestDef::Note(chars.as_str().trim().to_string()));
            }

            // Add reason
            reasons.push(chars.as_str().trim().to_string());
            reason_ref = Some(reasons.len() - 1);
          }

          // Note
          '*' => {
            let msg = chars.as_str().trim().to_string();
            if !msg.is_empty() {
              tests.push(TestDef::Note(msg));
            }
            continue;
          }

          // Unknown
          _ => return Err(UnknownLineOperator(first)),
        }
      }
    }

    Ok(Phoner {
      rules: make_regex(rules, &classes)?,
      tests,
      reasons,
    })
  }
}

/// Substitute classes in rule and create regex
fn make_regex(
  raw_rules: Vec<(bool, String, Option<usize>)>,
  classes: &Classes,
) -> Result<Rules, ParseError> {
  let mut rules = Rules::new();

  for (intent, rule, reason) in raw_rules {
    let re = match Regex::new(&substitute_classes(rule, classes)?) {
      Ok(x) => x,
      Err(err) => return Err(RegexFail(err)),
    };

    rules.push((intent, re, reason));
  }

  Ok(rules)
}

/// Substitute class names regex rule with class values
// TODO Optimize this! Sub all classes first, then sub rules
// TODO Change to not using blind replace - check for /<.>/ then check if class exists
fn substitute_classes(rule: String, classes: &Classes) -> Result<String, ParseError> {
  let mut rule = rule;

  for (name, value) in classes {
    let ident = format!("<{}>", name);
    if rule.contains(&ident) {
      rule = rule.replace(&ident, &substitute_classes(value.to_string(), &classes)?);
    }
  }

  Ok(rule)
}
