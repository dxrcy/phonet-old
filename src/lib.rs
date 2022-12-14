mod args;
mod scheme;
mod tests;

pub use args::{Args, DisplayLevel};
pub use scheme::{ParseError, Scheme, TestType};
pub use tests::run_tests;

use Validity::*;

/// State of rules match of word
///
/// If invalid, reason can be provided
enum Validity {
  Valid,
  Invalid(Option<usize>),
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
  ///
  /// Replaces reference to reason with value
  pub fn unwrap_or<'a>(
    self,
    if_valid: &'a str,
    if_none: &'a str,
    reasons: &'a Vec<String>,
  ) -> &'a str {
    if let Invalid(reason_ref) = self {
      return match reason_ref {
        Some(reason) => match reasons.get(reason) {
          Some(x) => x,
          None => if_none,
        },
        None => if_none,
      };
    }
    if_valid
  }
}
