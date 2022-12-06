use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
  /// Path of file to parse and test
  pub file: String,
}
