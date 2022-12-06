use std::fs;

use phonotactics::{run_tests, Scheme};

fn main() {
  let file = fs::read_to_string("./example.phn").expect("Could not read file");
  let scheme = Scheme::parse(&file).expect("Could not parse file");
  run_tests(scheme);
}
