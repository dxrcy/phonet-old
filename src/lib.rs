pub use scheme::Scheme;
pub use tests::run_tests;
use Validity::*;

mod scheme;
mod tests;

/// Alias for vector of patterns (intent, expression, and invalidity reason)
type Patterns = Vec<(bool, String, Option<String>)>;
/// Alias for vector of tests (intent and value)
type Tests = Vec<(bool, String)>;

/// State of pattern match of word
///
/// If invalid, reason can be provided
enum Validity {
  Valid,
  Invalid(Option<String>),
}

impl Validity {
  /// Returns `true` if state is `Valid`
  pub fn is_valid(&self) -> bool {
    if let Valid = self {
      return true;
    }
    false
  }

  /// Unwrap reason with default
  pub fn unwrap_or(self, if_valid: String, if_none: String) -> String {
    if let Invalid(reason) = self {
      return match reason {
        Some(reason) => reason,
        None => if_none,
      };
    }
    if_valid
  }
}
