use std::fs;

use clap::Parser;
use phoner::{Args, Phoner, TestResults, TestType};

fn main() -> Result<(), String> {
  let args = Args::parse();

  // Read file
  //TODO Change this! To expect ?
  let file = fs::read_to_string(&args.file)
    .map_err(|err| format!("Could not read file '{}' - {:?}", args.file, err))?;

  // Parse file
  let mut scheme = Phoner::parse(&file).map_err(|x| format!("Could not parse file: {x}"))?;

  // Use CLI tests if given
  if let Some(tests) = args.tests {
    scheme.tests = tests
      .split(',')
      .map(|x| TestType::Test {
        intent: true,
        word: x.to_string(),
      })
      .collect();
  }

  // Run tests
  let results = TestResults::run(scheme);

  // Display tests
  results.display(args.display_level);

  Ok(())
}
