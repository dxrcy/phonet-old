use std::collections::HashMap;

use fancy_regex::Regex;

pub use crate::tests::Reason;

/// Alias for vector of rules (intent, expression, and invalidity reason)
pub type Rules = Vec<(bool, Regex, Option<usize>)>;

/// Alias for hashmap of class name and value
pub type Classes = HashMap<String, String>;

/// Definition of test or note
#[derive(Debug)]
pub enum TestDef {
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

/// Result of test or note
pub enum TestResult {
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
