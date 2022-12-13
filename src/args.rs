use std::fmt::Display;

use clap::{Parser, ValueEnum};

use DisplayLevel::*;

#[derive(Parser)]
#[clap(author, version, about)]
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

#[derive(ValueEnum, Clone)]
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
