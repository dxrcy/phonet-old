use clap::{builder::PossibleValue, Parser, ValueEnum};

use crate::DisplayLevel::{self, *};

#[derive(Parser)]
#[clap(author, version)]
/// A program to validate phonotactic patterns
///
/// More information: https://github.com/darccyy/phoner
pub struct Args {
  /// Custom test, separate with comma (Ignores tests in file)
  #[arg(short, long)]
  pub tests: Option<String>,

  /// Name and path of file to run and test
  ///
  /// Eg. `phoner -f ./myfile.phoner`
  #[arg(short, long, default_value_t = String::from("phoner"))]
  pub file: String,

  /// What types of outputs to display
  ///
  /// Options can be single letter
  ///
  /// Eg. `phoner -d just-fails` or `phoner -df`
  #[arg(short, long, default_value_t = ShowAll, value_enum)]
  pub display_level: DisplayLevel,

  /// Minify file and save
  #[arg(short, long, value_enum)]
  pub minify: Option<Option<WithTests>>,
}

#[derive(Clone, Copy, Debug)]
/// Custom implementation of boolean, for argument aliases
pub enum WithTests {
  Tests,
}

// Custom implementation, for argument aliases
impl ValueEnum for WithTests {
  fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
    Some(match self {
      Self::Tests => PossibleValue::new("tests")
        .aliases(["t"])
        .help("Include tests"),
    })
  }

  fn value_variants<'a>() -> &'a [Self] {
    &[Self::Tests]
  }
}
