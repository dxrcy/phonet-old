use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Args {
  /// Custom test, separate with comma (Ignores tests in file)
  pub tests: Option<String>,

  /// Path of file to parse and test
  #[arg(short, long, default_value_t = String::from(".phoner"))]
  pub file: String,
}
