# Phoner

Phoner is a CLI tool and library to validate phonotactic patterns for constructed languages.
It is compatible with either romanization and phonetic transcription.
Words can be randomly generated (see [Argument Syntax](#argument-syntax)).

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phoner-syntax)

# Usage

This project can be used as a rust library, or as a binary.

## Binary use

[Download latest version here](https://github.com/darccyy/phoner/releases/latest)

### Argument Syntax

```
$ phoner --help

Usage: phoner.exe [OPTIONS] [TESTS]

Options:
  -t, --tests <TESTS>
      Custom test, separate with comma (Ignores tests in file)

  -f, --file <FILE>
      Name and path of file to run and test

      Eg. `phoner -f ./myfile.phoner`

      [default: phoner]

  -d, --display-level <DISPLAY_LEVEL>
      What types of outputs to display

      Options can be single letter

      Eg. `phoner -d just-fails` or `phoner -df`

      [default: show-all]

      Possible values:
        - show-all:        Show everything (passes, notes, fails)
        - notes-and-fails: Show most (notes, fails), but not passes
        - just-fails:      Show only fails, not passes or notes
        - hide-all:        Show nothing: not passes, notes, or fails

  -m, --minify [<MINIFY>]
      Minify file and save

      Possible values:
        - tests: Include tests

  -g, --generate [<GENERATE>]
      Generate random words

      Default count 1, specify with number

      --gmin <GENERATE_MIN_LEN>
          Set minimum length for generated words

          Use with the `--generate` or `-g` flag

          Note: This increases generation time exponentially

      --gmax <GENERATE_MAX_LEN>
          Set maximum length for generated words

          Use with the `--generate` or `-g` flag

  -n, --no-color
      Display output in default color

      Use for piping standard output to a file

  -h, --help
      Print help information (use `-h` for a summary)
```

### Example

```bash
# Runs ./phoner
phoner

# Runs ./phoner, with tests: 'some', 'words' (instead of tests in file)
phoner -t some,words

# Runs ./myfile.phoner
phoner -f myfile.phoner

# Runs ./phoner, only showing fails
phoner -df
# Alternatives:
phoner -d just-fails
phoner -d fails

# Runs ./phoner, and minifies to ./min.phoner without tests
phoner -m

# Runs ./myfile.phoner, without outputting any results, and minifies to ./myfile.min.phoner with tests
phoner -f myfile.phoner -dh -mt

# Runs ./phoner, and generates 1 random word
phoner -g

# Runs ./myfile.phoner, and generates 10 random words
phoner -g10 -g myfile.phoner

# Runs ./phoner, with no color, and writes output to ./phoner.txt
phoner -n > phoner.txt

# Runs ./myfile.phoner, with all test output hidden, and generates 3 random words with length 6-8, writes output to ./phoner.txt (with no color)
phoner -f myfile.phoner -nd h -g 3 --gmin 6 --gmax 8 > ./phoner.txt
```

### Create Alias / Path

Replace `<path_to_file>` with the directory of the downloaded binary.

#### Bash

Add alias in `.bashrc` in user directory

```bash
# ~/.bashrc
alias phoner="<path_to_file>/phoner.exe"
```

#### Powershell

Add to `$env:PATH`

```ps1
$env:Path = "$env:Path;<path_to_file>\phoner.exe"
```

## Library use

Add `phoner = "0.6.4"` to your `Crates.toml` file

- [Docs.rs](https://docs.rs/phoner/latest/phoner)
- [Crates.io](https://crates.io/crates/phoner)

Short example:

```rust
use phoner::Phoner;

fn main() {
  let file = std::fs::read_to_string("phoner").unwrap();

  // Parse file
  Phoner::parse(&file).unwrap()
    // Run tests
    .run(scheme)
    // Display results
    .display(Default::default());
}
```

Long example:

```rust
use phoner::{Phoner, DisplayLevel};

fn main() {
  let file = std::fs::read_to_string("phoner").unwrap();

  // Parse file
  let scheme = Phoner::parse(&file).unwrap();

  // Run tests
  let results = scheme.run(scheme);

  // Display results - This could be manually implemented
  results.display(DisplayLevel::ShowAll, false);

  // Generate random words
  let words = scheme.generate(10, 3..14).unwrap();
  println!("{words:?}");
}
```

# File syntax

A `phoner` file is used to define the rules, classes, and tests for the program.

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phoner-syntax)

## Statements

The syntax is a statements, each separated by a semicolon `;` or a linebreak.

Comments will only end with a linebreak.

All whitespace is ignored, except to separate words in [_tests_](#tests).

> Note! This will replace spaces in Regex as well!

Each statement must begin with an operator:

- `#` Hashtag: A whole line comment. A linebreak (not a semicolon) ends the comment
- `$` Dollar: Define a [_class_](#classes)
- `+` **Plus** or `!` **Bang**: Define a [_rule_](#rule)
- `@` Commat: Define a [_reason_](#reasons) if a test fails
- `?` Question: Create a [_test_](#tests)
- `*` Star: Create a test [_note_](#notes) (also with `@*`)
- `~` Tilde: Define the [_mode_](#mode) of the file

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_rules_](#rules) at runtime.

Syntax:

- `$` Dollar
- Name - Must be only characters from [a-zA-Z0-9_]
- `=` Equals
- Value - Regular Expression, may contain other _classes_ in angle brackets `<>` (as with [_rules_](#rules))

The `any` class, defined with `$_ = ...`, is used for random word generation.

Example:

```phoner
# Some consonants
$C = [ptksmn]

# Some vowels
$V = [iueoa]

# Only sibilant consonants
$C_s = [sz]
```

## Rules

Rules are Regular Expressions used to test if a word is valid.

Rules are defined with an _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ rule must be followed for a word to be valid
- A _negative_ rule must **not** be followed for a word to be valid

To use a [_class_](#classes), use the class name, surrounded by angle brackets `<>`.

Syntax:

- `+` Plus or `!` Bang - Plus for _positive_ rule, Bang for _negative_ rule
- Pattern - Regular Expression, may contain [_classes_](#classes) in angle brackets `<>`

Example (with predefined [_classes_](#classes)):

```phoner
# Must be (C)V syllable structure
+ ^ (<C>? <V>)+ $

# Must not have two vowels in a row
! <V>{2}
```

## Tests

Tests are checked against all rules, and the result is displayed in the output.

Tests are ran in the order of definition.

Like [_rules_](#rules), tests must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ test will pass if it is valid
- A _negative_ test will **fail** if it is valid

Syntax:

- `?` Question mark
- `+` Plus or `!` Bang - Plus for _positive_ test, Bang for _negative_ test
- Tests - A word, or multiple words separated by a space

Example (with predefined [_rules_](#rules)):

```phoner
# This should match, to pass
?+ taso
# This test should NOT match, to pass
?! tax
# Each word is a test, all should match to pass
?+ taso sato tasa
```

## Reasons

Reasons are used before [_rules_](#rules) as an explanation if a test fails.

Syntax:

- `@` Commat
- _Optional_ `*` Star - Use as a note as well (a _noted_ reason)
- Text to define reason as (And print, if being used as note)

Example:

```phoner
@ Syllable structure
+ ^ (<C>? <V>)+ $

# This test will NOT match, however it SHOULD (due to the Plus), so it will FAIL, with the above reason
?+ tasto

# This reason has a Star, so it will be used as a note as well
@* Must not have two vowels in a row
! <V>{2}

?+ taso
```

## Notes

Notes are printed to the terminal output, alongside tests.

They can be used to separate tests into sections, however this is only cosmetic.

Syntax:

- `*` Star
- Text to print to terminal

Example (with predefined rules):

```phoner
* Should match
?+ taso

* Should not match
?! tatso
```

## Mode

The mode of a Phoner file can be one of these:

- Romanized: Using `<>`
- Broad transcription: Using `//`
- Narrow transcription: Using `[]`

This can optionally be specified in a file, although it does not add any functionality.

Syntax:

- `~` Tilde
- `<.>`, `/./`, or `[.]` - Mode identifier, with `.` being any string, or blank

Examples:

```phoner
# Specify romanized mode (fish icon)
~<>
```

```phoner
# Specify broad transcription
~ / this is the mode /
```

## Examples

See the [examples](./examples/) folder for `phoner` file examples.

- [Toki Pona](./examples/tokipona.phoner)
- [Ivalingo](./examples/ivalingo.phoner)

## Recommended Syntax Patterns

These formatting tips are not required, but recommended to make the file easier to read.

1. Specify the mode at the very top of the file
2. Define all classes at the top of the file
   - Also define an `any` class first, for word generation
3. Group related rules and tests, using a noted reason
   - Define rules first, then positive tests, then negative tests
4. Indent rules and tests under notes or reasons
   - Rules should use 1 intent, tests use 2

Eg.

```phoner
$_ = [ptkmnswjlaeiou]
$C = [ptkmnswjl]
$V = [aeiou]

@* Invalid letters
  + ^ <_>+ $
    ?+ taso
    ?! tyxo

@* Syllable structure
  + ^ ( <C> <V> ) $
    ?+ taso kili
    ?! ano taaso

* Some more tests
    ?+ silo tila
    ?! aka axe
```

![Phoner Icon](./icon.png)

# TODO

- Check all `.len()` calls on strings, check for non-ascii problems (use `.chars().count()`)
- Add line number traceback to initial class substitution error
- Add more docs
- Add tests !
- Add more info to `Error` variants
- Refactor modules (without breaking api?)
- Remove unnecessary `clone`s where possible

- Move `gen.rs` functionality to `gen` feature
