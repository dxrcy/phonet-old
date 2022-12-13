use crate::{
  args::DisplayLevel::{self, *},
  scheme::{Rules, Scheme, TestType},
  Validity::{self, *},
};

/// Run tests, output result
pub fn run_tests(scheme: Scheme, display_level: DisplayLevel) {
  if scheme.tests.len() < 1 {
    println!("\n\x1b[33mNo tests to run.\x1b[0m");
    return;
  }

  println!("\n\x1b[3;33mRunning {} tests...\x1b[0m", scheme.tests.len());

  // Get max length of all words
  let max_word_len = scheme
    .tests
    .iter()
    .map(|test| {
      if let TestType::Test(_, word) = test {
        word.len()
      } else {
        0
      }
    })
    .max()
    .unwrap_or(0);

  // Test each word, tally fails
  let mut fails = 0;
  let mut is_first_print = true;
  for test in scheme.tests {
    let (intent, word) = match test {
      TestType::Note(msg) => {
        match display_level {
          ShowAll | NotesAndFails => println!("\x1b[34m{msg}\x1b[0m"),
          _ => (),
        }
        continue;
      }
      TestType::Test(intent, word) => (intent, word),
    };

    // Validate word against rules, get reason for invalid
    let reason = validate_test(&word, &scheme.rules);

    // Check if test result matches intended result
    let passed = !(reason.is_valid() ^ intent);

    // Define reason for test fail
    let reason = if !passed {
      reason.unwrap_or(
        "\x1b[33mMatched, but should have not\x1b[0m",
        "No reason given",
        &scheme.reasons,
      )
    } else {
      ""
    };

    // Check if should output
    if match display_level {
      // Always show
      ShowAll => true,
      // Only show if failed
      NotesAndFails | JustFails if !passed => true,
      // Else skip
      _ => false,
    } {
      // Blank line if is first print
      if is_first_print {
        println!();
      }

      // Output single result
      println!(
        "  \x1b[{intent}\x1b[0m {word}{space}  \x1b[1;{result} \x1b[0;3;1m{reason}\x1b[0m",
        intent = if intent { "36m✔" } else { "35m✗" },
        result = if passed { "32mpass" } else { "31mFAIL" },
        space = " ".repeat(max_word_len - word.len()),
      );
      is_first_print = false;
    }

    // Increase fails tally if failed
    if !passed {
      fails += 1;
    }
  }

  // Blank line if there was test print
  if !is_first_print {
    println!();
  }

  // Output final result
  if fails == 0 {
    println!("\x1b[32;1;3mAll tests pass!\x1b[0m");
  } else {
    println!(
      "\x1b[31;1;3m{fails} test{s} failed!\x1b[0m",
      s = if fails == 1 { "" } else { "s" }
    );
  }
}

/// Check if string is valid with rules
fn validate_test(word: &str, rules: &Rules) -> Validity {
  // Check for match with every rule, if not, return reason
  for (should_match, rule, reason_ref) in rules {
    // Check if rule matches, and whether match signifies returning invalid or continuing
    if should_match
      ^ rule
        .is_match(word)
        // ? Why is this a result ?
        //TODO Fix this
        .expect("Failed checking match. This error should have been fixed :(")
    {
      return Invalid(*reason_ref);
    }
  }

  Valid
}
