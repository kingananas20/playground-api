use super::Edition;
use serde::{Deserialize, Serialize};

/// A request structure for running Rust code under Miri, the Rust interpreter for detecting undefined behavior.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MiriRequest {
    /// The Rust source code to be interpreted.
    pub code: String,

    /// The Rust edition to use (e.g., 2021, 2024).
    pub edition: Edition,

    /// Whether the request is for running tests instead of the main function.
    pub tests: bool,

    /// Optional memory aliasing model used by Miri (e.g., `stacked` or `tree`).
    #[serde(rename = "aliasingModel")]
    pub aliasing_model: Option<AliasingModel>,
}

impl MiriRequest {
    /// Creates a new [`MiriRequest`] with the provided parameters.
    ///
    /// # Parameters
    /// - `code`: The Rust source code.
    /// - `edition`: The Rust edition to use.
    /// - `tests`: Whether to run tests.
    /// - `aliasing_model`: Optional aliasing model for memory interpretation.
    pub fn new(
        code: String,
        edition: Edition,
        tests: bool,
        aliasing_model: Option<AliasingModel>,
    ) -> Self {
        Self {
            code,
            edition,
            tests,
            aliasing_model,
        }
    }
}

impl Default for MiriRequest {
    /// Returns a default [`MiriRequest`] using `Edition2024`, no tests,
    /// and the `Stacked` aliasing model.
    fn default() -> Self {
        Self {
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
            edition: Edition::Edition2024,
            tests: false,
            aliasing_model: Some(AliasingModel::Stacked),
        }
    }
}

/// The response returned after executing a Miri request.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MiriResponse {
    /// Indicates whether execution was successful.
    pub success: bool,

    /// Additional detail about how the process exited.
    pub exit_detail: String,

    /// The standard output from the Miri execution.
    pub stdout: String,

    /// The standard error from the Miri execution.
    pub stderr: String,
}

/// The aliasing model used by Miri to simulate pointer and memory behavior.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AliasingModel {
    /// Uses the Stacked Borrows model for aliasing checks.
    Stacked,

    /// Uses the Tree Borrows model for aliasing checks.
    Tree,
}
