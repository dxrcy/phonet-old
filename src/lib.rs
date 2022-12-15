use std::collections::HashMap;

use fancy_regex::Regex;

mod args;
mod scheme;
mod tests;

pub use args::{Args, DisplayLevel};
pub use scheme::{ParseError, Phoner};
pub use tests::TestResults;

/// Alias for vector of rules (intent, expression, and invalidity reason)
pub type Rules = Vec<(bool, Regex, Option<usize>)>;

/// Alias for vector of tests (intent and value)
type Tests = Vec<TestType>;

/// Alias for hashmap of class name and value
type Classes = HashMap<String, String>;

/// Type of test
///
/// Can be test, or note
#[derive(Debug)]
pub enum TestType {
  /// Display line of text
  Note(String),
  /// Result of test
  Test {
    /// Intent of test passing
    intent: bool,
    /// Word to test
    word: String,
  },
}

/// Result of test, or note
pub enum ResultType {
  /// Display line of text
  Note(String),
  /// Result of test
  Test {
    /// Intent of test passing
    intent: bool,
    /// Word tested
    word: String,
    /// Whether test passed or not
    pass: bool,
    /// Reason for fail
    reason: Reason,
  },
}

/// Reason for failure variants
pub enum Reason {
  /// Test passed, do not display reason
  Passed,
  /// No reason was given for rule for test failing
  NoReasonGiven,
  /// Test matched, but should have not
  ShouldNotHaveMatched,
  /// Custom reason for rule
  Custom(String),
}
