use std::collections::HashMap;
// error::Error, fmt::Display

use fancy_regex::Regex;
use snafu::prelude::*;

pub use crate::run::Reason;

/// Error enum for `Phoner` struct in `parse.rs`
#[derive(Debug, Snafu)]
pub enum ParseError {
  #[snafu(display("Unknown intent identifier `{ch}`. Must be either `+` or `!`, on line {line}"))]
  UnknownIntentIdentifier { ch: char, line: usize },

  #[snafu(display("Unknown line operator `{ch}`, on line {line}"))]
  UnknownLineOperator { ch: char, line: usize },

  #[snafu(display("No class name given, on line {line}"))]
  NoClassName { line: usize },

  #[snafu(display(
    "Invalid class name `{name}`, on {line}. Must only contain characters from [a-zA-Z0-9_]"
  ))]
  InvalidClassName { name: String, line: usize },

  #[snafu(display("No class value given, on line {line}"))]
  NoClassValue { line: usize },

  #[snafu(display("Failed to parse Regex: {err}, on line {line}"))]
  RegexFail {
    err: fancy_regex::Error,
    line: usize,
  },
}

/// Alias for vector of rules (intent, expression, and invalidity reason)
pub type Rules = Vec<(bool, Regex, Option<usize>)>;

/// Alias for hashmap of class name and value
pub type Classes = HashMap<String, String>;

/// Definition of test or note
#[derive(Debug)]
pub enum TestDefinition {
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
