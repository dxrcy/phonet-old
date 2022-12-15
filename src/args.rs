use std::fmt::Display;

use clap::{builder::PossibleValue, Parser, ValueEnum};

use DisplayLevel::*;

#[derive(Parser)]
#[clap(author, version)]
/// A program to validate phonotactic patterns
///
/// More information: https://github.com/darccyy/phoner
pub struct Args {
  /// Custom test, separate with comma (Ignores tests in file)
  pub tests: Option<String>,

  /// Path of file to parse and test
  #[arg(short, long, default_value_t = String::from("phoner"))]
  pub file: String,

  /// Don't display passing tests to output
  #[arg(short, long, default_value_t = ShowAll, value_enum)]
  pub display_level: DisplayLevel,
}

/// Setting for controlling which items are outputted in `PhonerResult::display` method
#[derive(Clone)]
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

// Custom implementation, for shorthand values
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
