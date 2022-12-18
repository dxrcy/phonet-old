use std::collections::HashMap;

use fancy_regex::Regex;

use crate::{
  types::{
    Classes,
    Error::{self, *},
    Rule, TestDefinition,
  },
  PhonerResults,
};

struct RawRule {
  pub intent: bool,
  pub pattern: String,
  pub reason_ref: Option<usize>,
  pub line: usize,
}

/// Holds data for minify
#[derive(Debug)]
struct Mini {
  /// Classes defined
  classes: Vec<String>,
  /// Rules defined
  rules: Vec<String>,
  /// Positive tests defined
  tests_pos: Vec<String>,
  /// Negative tests defined
  tests_neg: Vec<String>,
}

impl Mini {
  /// Create empty struct
  pub fn new() -> Self {
    Mini {
      classes: Vec::new(),
      rules: Vec::new(),
      tests_pos: Vec::new(),
      tests_neg: Vec::new(),
    }
  }
}

/// Scheme parsed from file
///
/// Holds rules and tests
#[derive(Debug)]
pub struct Phoner {
  /// Defined rules
  pub rules: Vec<Rule>,
  /// Tests to run
  pub tests: Vec<TestDefinition>,
  /// Defined reasons values for rules
  pub reasons: Vec<String>,
  /// Classes
  pub classes: Classes,
  /// Minified data
  mini: Mini,
}

impl Phoner {
  /// Parse `Phoner` from string
  pub fn parse(file: &str) -> Result<Phoner, Error> {
    // Builders
    let mut raw_classes: Classes = HashMap::new();
    let mut tests: Vec<TestDefinition> = Vec::new();
    let mut rules: Vec<RawRule> = Vec::new();

    let mut reasons = Vec::new();
    let mut reason_ref: Option<usize> = None;

    // For minify
    let mut mini = Mini::new();

    let class_name_pattern = Regex::new(r"^\w+$").expect("Could not parse static regex");

    // Split at line
    for (line, line_statements) in file.lines().enumerate() {
      // Line number (as in file)
      let line = line + 1;

      // Split line at semicolon
      for statement in line_statements.split(';') {
        let statement = statement.trim();

        // Continue for blank
        if statement.is_empty() {
          continue;
        }

        let mut chars = statement.chars();

        if let Some(first) = chars.next() {
          match first {
            // Comment
            '#' => continue,

            // Class
            '$' => {
              let mut split = chars.as_str().split('=');

              // Get name
              let name = match split.next() {
                Some(x) => x.trim().to_string(),
                None => return Err(Error::NoClassName { line }),
              };

              // Check if name is valid
              if !class_name_pattern
                .is_match(&name)
                .expect("Failed checking regex match. This error should NEVER APPEAR!")
              {
                return Err(Error::InvalidClassName { name, line });
              }

              // Get value
              let value = match split.next() {
                Some(x) => x.trim(),
                None => return Err(Error::NoClassValue { name, line }),
              };

              // Check that class does not already exist
              if raw_classes.get(&name).is_some() {
                return Err(Error::ClassAlreadyExist { name, line });
              }

              // Wrap value in brackets (just in case)
              let value = format!("({})", value.replace(' ', ""));

              // Add raw line
              mini.classes.push(format!("${}={}", name, value));

              // Insert class
              raw_classes.insert(name.to_string(), value);
            }

            // Rule
            '+' | '!' => {
              // `+` for true, `!` for false
              let intent = first != '!';

              let pattern = chars.as_str().replace(' ', "");

              // Add rule for minify
              mini.rules.push(first.to_string() + &pattern);

              // Add rule
              rules.push(RawRule {
                intent,
                pattern,
                reason_ref,
                line,
              });
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
                  return Err(UnknownIntentIdentifier { ch, line });
                }
                // No character
                None => continue,
              };

              // Split at space
              let words = chars.as_str().split_whitespace();
              for word in words {
                let word = word.trim().to_string();

                // Add test for minify
                if intent {
                  mini.tests_pos.push(word.clone());
                } else {
                  mini.tests_neg.push(word.clone());
                }

                // Add test
                if !word.is_empty() {
                  tests.push(TestDefinition::Test { intent, word });
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
                tests.push(TestDefinition::Note(chars.as_str().trim().to_string()));
              }

              // Add reason
              reasons.push(chars.as_str().trim().to_string());
              reason_ref = Some(reasons.len() - 1);
            }

            // Note
            '*' => {
              let msg = chars.as_str().trim().to_string();
              if !msg.is_empty() {
                tests.push(TestDefinition::Note(msg));
              }
            }

            // Unknown
            _ => return Err(UnknownLineOperator { ch: first, line }),
          }
        }
      }
    }

    //TODO Add line number
    let mut classes = Classes::new();
    for (name, value) in &raw_classes {
      classes.insert(
        name.to_string(),
        substitute_classes(value, &raw_classes, 0)?,
      );
    }
    let classes = classes;

    // Convert rules to regex rules
    let rules = make_regex(rules, &classes)?;

    Ok(Phoner {
      rules,
      tests,
      reasons,
      classes,
      mini,
    })
  }

  /// Minify Phoner scheme as string
  pub fn minify(&self, do_tests: bool) -> String {
    let s = ';';
    let c = self.mini.classes.join(";");
    let r = self.mini.rules.join(";");

    if do_tests {
      // Include tests
      format!(
        "{c}{s}{r}{s}?+{tp}{s}?!{tn}",
        tp = self.mini.tests_pos.join(" "),
        tn = self.mini.tests_neg.join(" "),
      )
    } else {
      // Don't include tests
      format!("{c}{s}{r}")
    }
  }

  /// Run tests, return results
  pub fn run(&self) -> PhonerResults {
    PhonerResults::run(self)
  }
}

/// Substitute classes in rule and create regex
fn make_regex(raw_rules: Vec<RawRule>, classes: &Classes) -> Result<Vec<Rule>, Error> {
  let mut rules: Vec<Rule> = Vec::new();

  for RawRule {
    intent,
    pattern,
    reason_ref,
    line,
  } in raw_rules
  {
    let pattern = match Regex::new(&substitute_classes(&pattern, classes, line)?) {
      Ok(x) => x,
      Err(err) => return Err(RegexFail { err, line }),
    };

    rules.push(Rule {
      intent,
      pattern,
      reason_ref,
    });
  }

  Ok(rules)
}

/// Substitute class names regex rule with class values (recursively)
fn substitute_classes(pattern: &str, classes: &Classes, line: usize) -> Result<String, Error> {
  let mut output = String::new();

  // Build class name
  let mut name_build: Option<String> = None;

  // All previously checked characters in string (for check for lookbehind)
  let mut prev = String::new();

  // Loop characters
  for ch in pattern.chars() {
    match ch {
      // Open class name
      // Check that not in lookbehind
      '<' if !prev.ends_with("(?") => {
        if name_build.is_some() {
          // Name is already building - Another opening bracket should not be there
          return Err(Error::ClassUnexpectedOpenName {
            pattern: pattern.to_string(),
            line,
          });
        }

        // Start building name
        name_build = Some(String::new());
      }

      // Close class name
      '>' => {
        // Get class name
        let name = match name_build {
          Some(x) => x,
          None => {
            // No name is building - Closing bracket should not be there
            return Err(Error::ClassUnexpectedCloseName {
              pattern: pattern.to_string(),
              line,
            });
          }
        };

        // Get class value
        let value = match classes.get(&name) {
          Some(x) => x,
          None => {
            // Class name was not found
            return Err(Error::ClassNotFound {
              name: name.to_string(),
              line,
            });
          }
        };

        // Add value to output (recursively)
        output.push_str(&substitute_classes(value, classes, line)?);
        // Finish building name
        name_build = None;
      }

      // Normal character
      _ => {
        prev.push(ch);

        if let Some(name) = &mut name_build {
          // Name is building - push to name
          name.push(ch);
        } else {
          // Name is not building - push to regular output
          output.push(ch);
        }
      }
    }
  }

  // Class name was not finished building, before end of end of pattern
  if name_build.is_some() {
    return Err(Error::ClassUnexpectedEnd {
      pattern: pattern.to_string(),
      line,
    });
  }

  Ok(output)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn substitute_classes_works() {
    let classes = Classes::from([
      ("C".to_string(), "[ptk]".to_string()),
      ("Vowels".to_string(), "[aio]".to_string()),
      ("_".to_string(), "[<C><Vowels>]".to_string()),
    ]);

    assert_eq!(
      substitute_classes("<C>", &classes, 0).unwrap(),
      "[ptk]".to_string()
    );

    assert_eq!(
      substitute_classes("<C>-<Vowels>", &classes, 0).unwrap(),
      "[ptk]-[aio]".to_string()
    );

    assert_eq!(
      substitute_classes("<_>", &classes, 0).unwrap(),
      "[[ptk][aio]]".to_string()
    );

    assert!(match substitute_classes("<c>", &classes, 0) {
      Err(Error::ClassNotFound { .. }) => true,
      _ => false,
    });

    assert!(match substitute_classes("a>b", &classes, 0) {
      Err(Error::ClassUnexpectedCloseName { .. }) => true,
      _ => false,
    });

    assert!(match substitute_classes("<a<b>c>", &classes, 0) {
      Err(Error::ClassUnexpectedOpenName { .. }) => true,
      _ => false,
    });

    assert!(match substitute_classes("a<b", &classes, 0) {
      Err(Error::ClassUnexpectedEnd { .. }) => true,
      _ => false,
    });
  }
}
