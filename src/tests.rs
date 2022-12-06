use super::{
  Patterns, Scheme,
  Validity::{self, *},
};

/// Run tests, output result
pub fn run_tests(scheme: Scheme) {
  println!(
    "\n\x1b[3;33mRunning {} tests...\x1b[0m\n",
    scheme.tests.len()
  );

  // Get max length of all words
  let max_word_len = scheme.tests.iter().map(|x| x.1.len()).max().unwrap_or(0);

  // Test each word, tally fails
  let mut fails = 0;
  for (intent, word) in scheme.tests {
    // Validate word against patterns, get reason for invalid
    let reason = validate_test(&word, &scheme.patterns);

    // Check if test result matches intended result
    let passed = !(reason.is_valid() ^ intent);

    // Define reason for test fail
    let fail_reason = if !passed {
      reason.unwrap_or(
        //TODO Change these
        "\x1b[33mMatched when it should have not\x1b[0m".to_string(),
        "No reason given".to_string(),
      )
    } else {
      String::new()
    };

    // Output single result
    println!(
      "  \x1b[33m{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{fail_reason}\x1b[0m",
      result = if passed { "32mpass" } else { "31mFAIL" },
      intent = if intent { "✔" } else { "✗" },
      space = " ".repeat(max_word_len - word.len()),
    );

    // Increase fails tally if failed
    if !passed {
      fails += 1;
    }
  }

  // Output final result
  if fails == 0 {
    println!("\n\x1b[32;3mAll tests pass!\x1b[0m\n");
  } else {
    println!("\n\x1b[31;1;3m{fails} tests failed!\x1b[0m\n");
  }
}

/// Check if string is valid with patterns
fn validate_test(word: &str, patterns: &Patterns) -> Validity {
  // Check for match with every pattern, if not, return reason
  for (should_match, pattern, reason) in patterns {
    // Check if pattern matches, and whether match signifies returning invalid or continuing
    if should_match ^ pattern.is_match(word) {
      return Invalid(reason.clone());
    }
  }

  Valid
}
