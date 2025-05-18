use super::{Channel, CrateType, Edition};
use serde::{Deserialize, Serialize};

/// Represents a request to run Clippy (Rust linter) on the given Rust code.
///
/// Contains configuration options like the Rust channel, crate type, edition,
/// and the actual source code to analyze.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ClippyRequest {
    /// The Rust release channel to use (stable, beta, nightly).
    pub channel: Channel,

    /// The type of crate to analyze (binary or library).
    #[serde(rename = "crateType")]
    pub crate_type: CrateType,

    /// The Rust edition to use (2015, 2018, 2021, 2024).
    pub edition: Edition,

    /// The Rust source code to be linted by Clippy.
    pub code: String,
}

impl ClippyRequest {
    /// Creates a new `ClippyRequest`.
    ///
    /// # Arguments
    ///
    /// * `channel` - The Rust channel to run Clippy on.
    /// * `crate_type` - The crate type (binary or library).
    /// * `edition` - The Rust edition to compile with.
    /// * `code` - The source code to analyze.
    ///
    /// # Returns
    ///
    /// A `ClippyRequest` instance configured with the given parameters.
    pub fn new(channel: Channel, crate_type: CrateType, edition: Edition, code: String) -> Self {
        Self {
            channel,
            crate_type,
            edition,
            code,
        }
    }
}

impl Default for ClippyRequest {
    /// Provides a default `ClippyRequest` configuration.
    ///
    /// Defaults to:
    /// - `channel`: `Stable`
    /// - `crate_type`: `Binary`
    /// - `edition`: `2024`
    /// - `code`: A basic "Hello, world!" program
    ///
    /// # Returns
    ///
    /// A `ClippyRequest` instance with default values for linting basic Rust code.
    fn default() -> Self {
        Self {
            channel: Channel::Stable,
            crate_type: CrateType::Binary,
            edition: Edition::Edition2024,
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
        }
    }
}

/// Represents the response from running Clippy on submitted code.
///
/// Includes success status, exit details, and output streams.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClippyResponse {
    /// Whether Clippy completed successfully without errors.
    pub success: bool,

    /// Details about the process exit (exit code, signals, etc.).
    pub exit_detail: String,

    /// Standard output from Clippy (usually empty or informational).
    pub stdout: String,

    /// Standard error output containing Clippy warnings, errors, and suggestions.
    pub stderr: String,
}
