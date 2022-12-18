use std::{collections::HashMap, fmt::Display};

use clap::{builder::PossibleValue, ValueEnum};
use fancy_regex::Regex;
use snafu::prelude::*;

pub use crate::run::Reason;
use DisplayLevel::*;

/// Error enum for `Phoner` struct in `parse.rs`
#[derive(Debug, Snafu)]
pub enum Error {
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

  #[snafu(display("Class not found, with name `{name}`, on line {line}"))]
  ClassNotFound { name: String, line: usize },

  #[snafu(display(
    "Unexpected class name opening bracket (`<`), in pattern `{pattern}`, on line {line}"
  ))]
  ClassUnexpectedOpenName { pattern: String, line: usize },

  #[snafu(display(
    "Unexpected class name closing bracket (`>`), in pattern `{pattern}`, on line {line}"
  ))]
  ClassUnexpectedCloseName { pattern: String, line: usize },

  #[snafu(display(
    "Class name was not closed with bracket (`>`) before end of pattern, in pattern `{pattern}`, on line {line}"
  ))]
  ClassUnexpectedEnd { pattern: String, line: usize },

  #[snafu(display(
    "No 'any' class was defined. Define with `$_ = ...`"
  ))]
  MissingAnyClass,
}

#[derive(Debug)]
pub struct Rule {
  pub intent: bool,
  pub pattern: Regex,
  pub reason_ref: Option<usize>,
}

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

/// Setting for controlling which items are outputted in `PhonerResult::display` method
#[derive(Clone, Copy)]
pub enum DisplayLevel {
  /// Show everything (passes, notes, fails)
  ShowAll,
  /// Show most (notes, fails), but not passes
  NotesAndFails,
  /// Show only fails, not passes or notes
  JustFails,
  /// Show nothing: not passes, notes, or fails
  HideAll,
}

// Custom implementation, for argument aliases
impl ValueEnum for DisplayLevel {
  fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
    // `help` values must mirror comments
    Some(match self {
      Self::ShowAll => PossibleValue::new("show-all")
        .aliases(["s", "show", "sa", "showall"])
        .help("Show everything (passes, notes, fails)"),

      Self::NotesAndFails => PossibleValue::new("notes-and-fails")
        .aliases(["n", "notesfails", "notes", "na"])
        .help("Show most (notes, fails), but not passes"),

      Self::JustFails => PossibleValue::new("just-fails")
        .aliases(["j", "f", "fails", "justfails"])
        .help("Show only fails, not passes or notes"),

      Self::HideAll => PossibleValue::new("hide-all")
        .aliases(["h", "hide", "ha", "hideall"])
        .help("Show nothing: not passes, notes, or fails"),
    })
  }

  fn value_variants<'a>() -> &'a [Self] {
    &[
      Self::ShowAll,
      Self::NotesAndFails,
      Self::JustFails,
      Self::HideAll,
    ]
  }
}

impl Default for DisplayLevel {
  fn default() -> Self {
    ShowAll
  }
}

impl Display for DisplayLevel {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        ShowAll => "ShowAll",
        NotesAndFails => "NotesAndFails",
        JustFails => "JustFails",
        HideAll => "HideAll",
      }
    )
  }
}
