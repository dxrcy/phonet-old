/// Error enum for `Phoner` struct in `scheme.rs`
#[derive(Debug)]
pub enum ParseError {
  UnknownIntentIdentifier(char),
  UnknownLineOperator(char),
  UnknownClass(char),
  NoClassName,
  NoClassValue,
  RegexFail(fancy_regex::Error),
}

impl error::Error for ParseError {}

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      UnknownIntentIdentifier(ch) => write!(
        f,
        "Unknown intent identifier `{ch}`. Must be either `+` or `!`"
      ),
      UnknownLineOperator(ch) => write!(f, "Unknown line operator `{ch}`"),
      UnknownClass(name) => write!(f, "Unknown class `{name}`"),
      NoClassName => write!(f, "No class name given"),
      NoClassValue => write!(f, "No class value given"),
      RegexFail(err) => write!(f, "Failed to parse Regex: {err}"),
    }
  }
}
