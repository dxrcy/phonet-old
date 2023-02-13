use std::{collections::HashMap, fmt::Display};

use clap::{builder::PossibleValue, ValueEnum};
use fancy_regex::Regex;

pub use crate::error::Error;
pub use crate::run::Reason;

use DisplayLevel::*;

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

/// Setting for controlling which items are outputted in `PhonetResult::display` method
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
