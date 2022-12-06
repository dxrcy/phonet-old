use std::fs;

use clap::Parser;
use phonotactics::{run_tests, Args, Scheme};

fn main() {
  let args = Args::parse();

  let file = fs::read_to_string(args.file).expect("Could not read file");
  let scheme = Scheme::parse(&file).expect("Could not parse file");

  if scheme.tests.len() > 0 {
    run_tests(scheme);
  } else {
    println!("\n\x1b[33mNo tests to run.\x1b[0m\n");
  }
}
