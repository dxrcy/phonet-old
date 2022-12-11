# Phoner

Phoner is a CLI tool to validate phonotactic patterns for constructed languages.
It is compatible with either romanization and phonetic transcription.

[Syntax Highlighting for VSCode](https://github.com/darccyy/phoner-syntax)

This program is still in development, and is subject to change.

# Usage

This project can be used as a rust library, or as a binary.

## Library use

Add `phoner = {git = "https://github.com/darccyy/phoner.git"}` to your `Crates.toml` file

**Crates.io** and **Docs.rs** coming soon...

## Binary use

[Download latest version here](https://github.com/darccyy/phoner/releases/latest)

Argument syntax:

```
$ phoner -h
Usage: phoner.exe [OPTIONS] [TESTS]

Arguments:
  [TESTS]  Custom test, separate with comma (Ignores tests in file)

Options:
  -f, --file <FILE>  Path of file to parse and test [default: .phoner]
  -h, --help         Print help information
  -V, --version      Print version information
```

# File syntax

A `.phoner` file is used to define the rules, classes, and tests for the program.

The syntax is command based.
A line can start with one of the following line operators:

- `#` Hashtag: A whole line comment
- `$` Dollar: Define a [_class_](#classes)
- `@` Commat: Describe a subsequent rule. This is used as the _reason_ if a test fails
- - `@@` Double Commat: A _useful_ reason, that is used by every following rule before the next reason
- `&` Ampersand: Define a [_rule_](#rule). Use `+` or `!` to identify _intent_
- - `&+` Ampersand Plus: Defines a _positive rule_ (Rule must be followed for word to be valid)
- - `&!` Ampersand Bang: Defines a _negative rule_ (Rule must **not** be followed for word to be valid)
- `*` Star: Creates a [_test_](#tests). Use `+` or `!` to identify _intent_
- - `*+` Star Plus: Creates a _passing test_ (Word must be valid to pass)
- - `*!` Star Bang: Creates a _failing test_ (Word must **not** be valid to pass)

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_rules_](#rules).

A class must be one capital letter.

## Rules

Rules are Regular Expressions used to test if a word is valid.

Rules must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ rule must be followed for a word to be valid.
- A _negative_ rule must **not** be followed for a word to be valid.

To use a [_class_](#classes), include the single capital letter name of the class.

## Tests

Tests are checked against all rules, and the result is displayed in the output.

Like [_rules_](#rules), tests must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ test will pass if it is valid.
- A _negative_ test will **fail** if it is valid.

## Example

Below is an example of a `.phoner` file.

```phoner
$V [iueoa]
$C [pbtdkgmnfvszcwjl]

@ Contains invalid letters
&+ ^ [VC]+ $

$S [sc]
$N [mn]

@ Basic structure
&+ ^ S? (C? V K?)+   $

*+ pono
*+ stono
*+ slono
*+ sonto
*+ ato

@ Any repeated letter
# Backreference
&! (.)\1{1,}

*+ tanta
*! atta
*! taata

@ More than 2 seq. consonants
&! C{3}

*+ apa
*! aspla
*! assa
*! asssa

@ Sibilant before anything other than [ptkmnl]
&! S[bdgfvszcwj]

*+ spono
*+ stono
*! sbono
*+ skono
*+ smono
*+ snono
*+ cnono
*+ cpono
*! cgono

@@ j|w beside i|u respectively
&! ji
&! ij
&! wu
&! uw

*! aji
*+ aju
*! awu

# Some more tests

*+ telo
*+ tenlo
*! telno
```

# TODO

- Fix error handling
- Add docs !
- Add tests !

## Experimental syntax

```
#* Reason
&@ Reason

+ Positive rule
! Negative rule

% Message (for tests)
#* Message
```
