# Phoner

Phoner is a CLI tool to validate phonotactic patterns for constructed languages.
It is compatible with either romanization and phonetic transcription.

[Download latest version here](https://github.com/darccyy/phoner/releases/latest)

[Syntax Highlighting for VSCode](https://github.com/darccyy/phoner-syntax)

```
Usage: phoner.exe <FILE>

Arguments:
  <FILE>  Path of file to parse and test

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

# File syntax

A `.phoner` file is used to define the patterns, classes, and tests for the program.

The syntax is command based.
A line can start with one of the following line operators:

- `#` Hashtag: A whole line comment
- `$` Dollar: Define a [_class_](#classes)
- `@` Commat: Describe a subsequent pattern. This is used as the _reason_ if a test fails
- `&` Ampersand: Define a [_pattern_](#patterns). Use `+` or `!` to identify _intent_
- - `&+` Ampersand Bang: Defines a _positive pattern_ (Pattern must be followed for word to be valid)
- - `&!` Ampersand Bang: Defines a _negative pattern_ (Pattern must **not** be followed for word to be valid)
- `*` Star: Creates a [_test_](#tests). Use `+` or `!` to identify _intent_
- - `*+` Star Bang: Creates a _passing test_ (Word must be valid to pass)
- - `*!` Star Bang: Creates a _failing test_ (Word must **not** be valid to pass)

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_patterns_](#patterns).

A class must be one capital letter.

## Patterns

Patterns are Regular Expressions used to test if a word is valid.

Patterns must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ pattern must be followed for a word to be valid.
- A _negative_ pattern must **not** be followed for a word to be valid.

To use a [_class_], include the single capital letter name of the class.

## Tests

Tests are checked against all patterns, and the result is displayed in the output.

Like [_patterns_](#patterns), tests must have a defined _intent_, either `+` for _positive_, or `!` for _negative_.

- A _positive_ test will pass if it is valid.
- A _negative_ test will **fail** if it is valid.

## Example

Below is an example of a `.phoner` file.

```phoner
# Consonants
$C [pbtdkgmnfvszcwjl]
# Vowels
$V [iueoa]
# Sibilants
$S [sʃ]

# Comment here
@ 3 or more consonants sequentially
&! C{3}

@ General invalid structure
&+ ^ S? ( C l? V n? )+ $

# Some words to test
*+ tanta
*! pania
*+ panka

# Some other tests
*+spato
*!splatlo
*!lask
```

# TODO

- Add `@@` syntax for reasons which are used by all following patterns (until next reason is defined)
- Change default reasons for invalid
- Recursive class unfolding
- Use error enum
- Add docs !
- Add tests !
