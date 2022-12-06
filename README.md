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

A `.phn` file is used to define the patterns, classes, and tests for the program.

The syntax is command based.
A line can start with one of the following line operators:

- `#` Hashtag: A whole line comment
- `$` Dollar: Define a [_class_](#classes)
- `@` Commat: Describe a subsequent pattern. This is used as the _reason_ if a test fails
- `&` Ampersand: Define a [_positive pattern_](#patterns)
- - `&!` Ampersand Bang: Defines a _negative pattern_
- `*` Star: Creates a [_passing test_](#tests)
- - `*!` Star Bang: Creates a _failing test_

## Classes

Classes are used as shorthand Regular Expressions, substituted into [_patterns_](#patterns).

A class must be one capital letter.

## Patterns

Patterns are Regular Expressions used to test if a word is valid.

By default patterns are _positive_, so they must be followed for a word to be valid.
A _negative_ pattern will must **not** be followed for a word to be valid.

To use a [_class_], include the single capital letter name of the class.

## Tests

Tests are checked against all patterns, and the result is displayed in the output.

## Example

Below is an example of a `.phn` file.

```phn
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
& ^ S? ( C l? V n? )+ $

# Some words to test
* tanta
* panka
*! pania

# Some other tests
* spato
*!splatlo
*!lask
```

# TODO

- Change default reasons for invalid
- Recursive class unfolding
- Use error enum
- Add docs !
- Add tests !
