/// Generate random word
mod gen;
/// Handles all parsing of `phonet` files
mod parse;
/// Handles running of tests
mod run;
/// Split file into statements
mod statements;
/// Holds simple types and structs
pub mod types;

mod error;

pub use parse::Phonet;
pub use run::{validate_test, PhonetResults, ValidStatus};
pub use types::DisplayLevel;
