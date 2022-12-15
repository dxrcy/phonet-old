use std::fs;

use clap::Parser;
use phoner::{types::TestDefinition, Args, Phoner, PhonerResults};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let args = Args::parse();

  // Read file
  let file = fs::read_to_string(&args.file)?;

  // Parse file
  let mut scheme = match Phoner::parse(&file) {
    Ok(x) => x,
    Err(err) => panic!("Failed to parse file: {err}"),
  };

  // Use CLI tests if given
  if let Some(tests) = args.tests {
    scheme.tests = tests
      .split(',')
      .map(|x| TestDefinition::Test {
        intent: true,
        word: x.to_string(),
      })
      .collect();
  }

  // Run tests
  let results = PhonerResults::run(scheme);

  // Display tests
  results.display(args.display_level);

  Ok(())
}
