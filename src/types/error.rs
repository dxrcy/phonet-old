use snafu::prelude::*;

/// Error enum for `Phonet` struct in `parse.rs`
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "Unknown intent identifier `{ch}`. Must be either `+` or `!`, at line {line}"
    ))]
    UnknownIntentIdentifier { ch: char, line: usize },

    #[snafu(display("Unknown line operator `{ch}`, at line {line}"))]
    UnknownLineOperator { ch: char, line: usize },

    #[snafu(display("Mode already defined, at line {line}"))]
    ModeAlreadyDefined { line: usize },

    #[snafu(display("Mode is invalid, it must be one of `<>`, `//`, or `[]`, at line {line}"))]
    InvalidMode { line: usize },

    #[snafu(display("No class name given, at line {line}"))]
    NoClassName { line: usize },

    #[snafu(display(
        "Invalid class name `{name}`, on {line}. Must only contain characters from [a-zA-Z0-9_]"
    ))]
    InvalidClassName { name: String, line: usize },

    #[snafu(display("Class already exists with `{name}`, on {line}"))]
    ClassAlreadyExist { name: String, line: usize },

    #[snafu(display("No class value given, with name `{name}`, at line {line}"))]
    NoClassValue { name: String, line: usize },

    #[snafu(display("Failed to parse Regex: {err}, at line {line}"))]
    RegexFail {
        err: fancy_regex::Error,
        line: usize,
    },

    #[snafu(display("Class not found, with name `{name}`, at line {line}"))]
    ClassNotFound { name: String, line: usize },

    #[snafu(display(
        "Unexpected class name opening bracket (`<`), in pattern `{pattern}`, at line {line}"
    ))]
    ClassUnexpectedOpenName { pattern: String, line: usize },

    #[snafu(display(
        "Unexpected class name closing bracket (`>`), in pattern `{pattern}`, at line {line}"
    ))]
    ClassUnexpectedCloseName { pattern: String, line: usize },

    #[snafu(display(
    "Class name was not closed with bracket (`>`) before end of pattern, in pattern `{pattern}`, at line {line}"
  ))]
    ClassUnexpectedEnd { pattern: String, line: usize },

    #[snafu(display("No 'any' class was defined. Define with `$_ = ...`"))]
    MissingAnyClass,
}
