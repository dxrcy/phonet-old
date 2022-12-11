use std::fs;

use clap::Parser;
use phoner::{run_tests, Args, Scheme};

fn main() -> Result<(), String> {
  let args = Args::parse();

  let file = fs::read_to_string(&args.file)
    .map_err(|err| format!("Could not read file '{}' - {:?}", args.file, err))?;

  let mut scheme = Scheme::parse(&file).map_err(|x| format!("Could not parse file: {x}"))?;

  if let Some(tests) = args.tests {
    scheme.tests = tests.split(',').map(|x| (true, x.to_string())).collect();
  }

  run_tests(scheme);

  Ok(())
}
