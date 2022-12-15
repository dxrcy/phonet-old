# Phoner

Phoner is a CLI tool to validate phonotactic patterns for constructed languages.
It is compatible with either romanization and phonetic transcription.

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phoner-syntax)

This program is still in development, and is subject to change.

# Usage

This project can be used as a rust library, or as a binary.

## Binary use

[Download latest version here](https://github.com/darccyy/phoner/releases/latest)

//TODO Add more explanation

Argument syntax:

```
$ phoner -h
Usage: phoner.exe [OPTIONS] [TESTS]

Arguments:
  [TESTS]  Custom test, separate with comma (Ignores tests in file)

Options:
  -f, --file <FILE>                    Path of file to parse and test [default: phoner]
  -d, --display-level <DISPLAY_LEVEL>  Don't display passing tests to output [default: show-all] [possible values: show-all, notes-and-fails, just-fails, hide-all]
  -h, --help                           Print help information (use `--help` for more detail)
  -V, --version                        Print version information
```

## Library use

Add `phoner = {git = "https://github.com/darccyy/phoner.git"}` to your `Crates.toml` file

**Crates.io** and **Docs.rs** coming soon...

//TODO Add example

# File syntax

A `phoner` file is used to define the rules, classes, and tests for the program.

[Syntax Highlighting Extension for VSCode](https://github.com/darccyy/phoner-syntax)

## Statements

The syntax is a statements, each separated by a semicolon `;` or a linebreak.

All whitespace is ignored, except to separate words in [_tests_](#tests).

Each statement must begin with an operator:

- `#` Hashtag: A whole line comment. A semicolon ends the comment
- `$` Dollar: Define a [_class_](#classes)
- `+` **Plus** or `!` **Bang**: Define a [_rule_](#rule)
- `@` Commat: Define a [_reason_](#reasons) if a test fails
- `?` Question: Create a [_test_](#tests)
- `*` Star: Create a test [_note_](#notes)

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_rules_](#rules) at runtime.

Syntax:

- `$` Dollar
- Name - Must be only characters from [a-zA-Z0-9_]
- `=` Equals
- Value - Regular Expression, may contain other _classes_ in angle brackets `<>` (as with [_rules_](#rules))

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
- _Optional_ `*` Star - Use as a note as well
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

## Examples

See the [examples](./examples/) folder for `phoner` file examples.

# TODO

- Clean types, structs, enums
- - Location
- - Names
- - Field / variant names
- Fix max word len with display level
- Fix error handling !!!
- - Return error type that implements `std::error::Error`
- Add more docs !
- Add tests !
- Remove unnecessary `clone`s where possible
