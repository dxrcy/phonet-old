use fancy_regex::Regex;

use crate::{
  types::{
    Classes,
    ParseError::{self, *},
    Rule, TestDefinition,
  },
  PhonerResults,
};

struct RawRule {
  pub intent: bool,
  pub pattern: String,
  pub reason_ref: Option<usize>,
  pub line_num: usize,
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
  /// Minified data
  mini: Mini,
}

impl Phoner {
  /// Parse `Phoner` from string
  pub fn parse(file: &str) -> Result<Phoner, ParseError> {
    // Builders
    let mut classes: Classes = Classes::new();
    let mut tests: Vec<TestDefinition> = Vec::new();
    let mut rules: Vec<RawRule> = Vec::new();

    let mut reasons = Vec::new();
    let mut reason_ref: Option<usize> = None;

    // For minify
    let mut mini = Mini::new();

    let class_name_pattern = Regex::new(r"^\w+$").expect("Could not parse static regex");

    // Split at line
    for (line_num, line) in file.lines().enumerate() {
      // Split line at semicolon
      for line in line.split(';') {
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
              let mut split = chars.as_str().split('=');

              // Get name
              let name = match split.next() {
                Some(x) => x.trim(),
                None => return Err(ParseError::NoClassName { line: line_num + 1 }),
              };

              // Check if name is valid
              if !class_name_pattern
                .is_match(name)
                .expect("Failed checking regex match. This error should NEVER APPEAR!")
              {
                return Err(ParseError::InvalidClassName {
                  name: name.to_string(),
                  line: line_num + 1,
                });
              }

              // Get value
              let value = match split.next() {
                Some(x) => x.trim(),
                None => return Err(ParseError::NoClassValue { line: line_num + 1 }),
              };

              // Insert class
              classes.insert(name.to_string(), value.to_string());

              // Add raw line
              mini.classes.push(format!("${}={}", name, value));
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
                line_num,
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
                  return Err(UnknownIntentIdentifier {
                    ch,
                    line: line_num + 1,
                  });
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
            _ => {
              return Err(UnknownLineOperator {
                ch: first,
                line: line_num + 1,
              })
            }
          }
        }
      }
    }

    // Convert rules to regex rules
    let rules = make_regex(rules, &classes)?;

    Ok(Phoner {
      rules,
      tests,
      reasons,
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
  pub fn run(self) -> PhonerResults {
    PhonerResults::run(self)
  }
}

/// Substitute classes in rule and create regex
fn make_regex(raw_rules: Vec<RawRule>, classes: &Classes) -> Result<Vec<Rule>, ParseError> {
  let mut rules: Vec<Rule> = Vec::new();

  for RawRule {
    intent,
    pattern,
    reason_ref,
    line_num,
  } in raw_rules
  {
    let pattern = match Regex::new(&substitute_classes(pattern, classes)?) {
      Ok(x) => x,
      Err(err) => {
        return Err(RegexFail {
          err,
          line: line_num + 1,
        })
      }
    };

    rules.push(Rule {
      intent,
      pattern,
      reason_ref,
    });
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
      rule = rule.replace(&ident, &substitute_classes(value.to_string(), classes)?);
    }
  }

  Ok(rule)
}
