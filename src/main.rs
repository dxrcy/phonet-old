use std::fs;

use clap::Parser;
use phoner::{run_tests, Args, Scheme};

fn main() {
  let args = Args::parse();

  let file = fs::read_to_string(&args.file).expect(&format!("Could not read file '{}'", args.file));

  let mut scheme = Scheme::parse(&file).expect("Could not parse file");

  if let Some(tests) = args.tests {
    scheme.tests = tests.split(',').map(|x| (true, x.to_string())).collect();
  }

  run_tests(scheme);
}
