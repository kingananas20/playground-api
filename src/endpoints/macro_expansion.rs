use super::Edition;
use serde::{Deserialize, Serialize};

/// A request to expand macros in a given Rust code snippet.
///
/// Includes the code and the selected Rust edition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpansionRequest {
    /// The Rust code containing macros to expand.
    pub code: String,
    /// The Rust edition to use for macro expansion.
    pub edition: Edition,
}

impl MacroExpansionRequest {
    /// Creates a new [`MacroExpansionRequest`] with the given code and edition.
    ///
    /// # Arguments
    ///
    /// * `code` - The Rust code to analyze.
    /// * `edition` - The Rust edition to use (e.g., 2018, 2021, 2024).
    ///
    /// # Returns
    ///
    /// A new [`MacroExpansionRequest`] instance.
    pub fn new(code: String, edition: Edition) -> Self {
        Self { code, edition }
    }
}

impl Default for MacroExpansionRequest {
    /// Returns a default [`MacroExpansionRequest`] with a simple `println!` example
    /// and the 2024 edition.
    fn default() -> Self {
        Self {
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
            edition: Edition::Edition2024,
        }
    }
}

/// A response from the Rust playground's macro expansion service.
///
/// Contains the macro-expanded output and status information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MacroExpansionResponse {
    /// Indicates whether macro expansion was successful.
    pub success: bool,
    /// Detailed information about the macro expansion process.
    pub exit_detail: String,
    /// The standard output from the macro expansion.
    pub stdout: String,
    /// The standard error from the macro expansion.
    pub stderr: String,
}
