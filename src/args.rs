// * This file is just for the binary

use clap::{builder::PossibleValue, Parser, ValueEnum};

use phonet::DisplayLevel::{self, *};

#[derive(Parser)]
#[clap(author, version)]
/// A program to validate phonotactic patterns
///
/// More information: https://github.com/darccyy/phonet
pub struct Args {
  /// Custom test, separate with comma (Ignores tests in file)
  #[arg(short, long)]
  pub tests: Option<String>,

  /// Name and path of file to run and test
  ///
  /// Eg. `phonet -f ./myfile.phonet`
  #[arg(short, long, default_value_t = String::from("phonet"))]
  pub file: String,

  /// What types of outputs to display
  ///
  /// Options can be single letter
  ///
  /// Eg. `phonet -d just-fails` or `phonet -df`
  #[arg(short, long, default_value_t = ShowAll, value_enum)]
  pub display_level: DisplayLevel,

  /// Minify file and save
  #[arg(short, long, value_enum)]
  pub minify: Option<Option<WithTests>>,

  /// Generate random words
  ///
  /// Default count 1, specify with number
  #[arg(short, long)]
  pub generate: Option<Option<usize>>,

  /// Set minimum length for generated words
  ///
  /// Use with the `--generate` or `-g` flag
  ///
  /// Note: This increases generation time exponentially
  #[arg(long = "gmin")]
  pub generate_min_len: Option<usize>,

  /// Set maximum length for generated words
  ///
  /// Use with the `--generate` or `-g` flag
  #[arg(long = "gmax")]
  pub generate_max_len: Option<usize>,

  /// Display output in default color
  ///
  /// Use for piping standard output to a file
  #[arg(short, long)]
  pub no_color: bool,
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
