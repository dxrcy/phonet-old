use std::fs;

use phonotactics::{parse, run_tests};

//TODO Remove common_macros crate

fn main() {
  let file = fs::read_to_string("./example.phn").expect("Could not read file");
  let scheme = parse(&file).expect("Could not parse file");

  run_tests(scheme);
}
