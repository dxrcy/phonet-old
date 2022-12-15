
  // Get max length of all words
  let max_word_len = results
    .iter()
    .map(|test| {
      if let TestType::Test(_, word) = test.0 {
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
  for (test, is_pass, reason) in results {
    let (intent, word) = match test {
      TestType::Note(note) => {
        match display_level {
          ShowAll | NotesAndFails => {
            // Blank line if is first print
            if is_first_print {
              println!();
            }

            // Print note
            println!("\x1b[34m{note}\x1b[0m");
            is_first_print = false;
          }
          _ => (),
        }
        continue;
      }
      TestType::Test(intent, word) => (intent, word),
    };

    // Define reason for test fail
    let reason = if !is_pass {
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
      NotesAndFails | JustFails if !is_passs => true,
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