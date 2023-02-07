mod args;

use std::fs;

use args::Args;
use clap::Parser;
use phonet::{types::TestDefinition, Phonet};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Read file
    let file = fs::read_to_string(&args.file)?;

    // Parse file
    let mut scheme = Phonet::parse(&file)
        .map_err(|err| err.to_string())
        .expect("Failed to parse file");

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

    // Minify file
    if let Some(do_tests) = args.minify {
        fs::write(
            get_min_filename(&args.file),
            scheme.minify(do_tests.is_some()),
        )?;
    }

    // Run tests and display
    if !scheme.tests.is_empty() {
        if args.no_color {
            println!("Running {} tests...", scheme.tests.len());
        } else {
            println!("\x1b[3;33mRunning {} tests...\x1b[0m", scheme.tests.len());
        }
    }
    scheme.run().display(args.display_level, args.no_color);

    // Generate and display random words, if CLI arg given
    if let Some(count) = args.generate {
        let count = count.unwrap_or(1);

        // Min and max length
        let length = args.generate_min_len.unwrap_or(3)..args.generate_max_len.unwrap_or(14);

        if count > 0 {
            if args.no_color {
                println!(
                    "Randomly generated word{s}:",
                    s = if count == 1 { "" } else { "s" }
                );
            } else {
                println!(
                    "\x1b[34mRandomly generated word{s}:\x1b[0m",
                    s = if count == 1 { "" } else { "s" }
                );
            }

            // Generate words
            let words = scheme
                .generate(count, length)
                .map_err(|err| err.to_string())
                .expect("Could not generate words");

            // Print words
            for word in words {
                if args.no_color {
                    println!(" - {}", word);
                } else {
                    println!(" \x1b[36m- \x1b[0;3m{}\x1b[0m", word);
                }
            }
        }
    }

    Ok(())
}

/// Adds '.min' to filename, before last file extension
///
/// Returns empty string if filename is empty
#[allow(dead_code)]
fn get_min_filename(file: &str) -> String {
    let mut split = file.split('.');

    match split.next_back() {
        // None or empty - Empty string
        None | Some("") => String::new(),

        // Some filename
        Some(last) => {
            let rest: Vec<&str> = split.collect();

            if !rest.is_empty() {
                // Filename and extension
                rest.join(".") + ".min." + last
            } else {
                // No extension or only extension (no filename)
                "min.".to_string() + last
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_min_filename_works() {
        assert_eq!(get_min_filename(""), "");
        assert_eq!(get_min_filename("phonet"), "min.phonet");
        assert_eq!(get_min_filename("myfile.phonet"), "myfile.min.phonet");
        assert_eq!(get_min_filename("one.two.phonet"), "one.two.min.phonet");
    }
}
