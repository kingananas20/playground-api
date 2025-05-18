use super::{Channel, CrateType, Edition};
use serde::{Deserialize, Serialize};

/// Request structure to format Rust source code via the playground.
///
/// Specifies formatting options and the source code to format.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatRequest {
    /// The Rust release channel to use for formatting (stable, beta, nightly).
    pub channel: Channel,

    /// The crate type (binary or library) of the code to format.
    pub crate_type: CrateType,

    /// The Rust edition to apply for formatting rules.
    pub edition: Edition,

    /// The Rust source code that needs formatting.
    pub code: String,
}

impl FormatRequest {
    /// Creates a new `FormatRequest`.
    ///
    /// # Arguments
    ///
    /// * `channel` - Rust release channel.
    /// * `crate_type` - Crate type (binary or library).
    /// * `edition` - Rust edition to format for.
    /// * `code` - Source code to be formatted.
    ///
    /// # Returns
    ///
    /// A `FormatRequest` initialized with the given parameters.
    pub fn new(channel: Channel, crate_type: CrateType, edition: Edition, code: String) -> Self {
        Self {
            channel,
            crate_type,
            edition,
            code,
        }
    }
}

impl Default for FormatRequest {
    /// Provides a default `FormatRequest` configuration.
    ///
    /// Defaults to:
    /// - `channel`: `Stable`
    /// - `crate_type`: `Binary`
    /// - `edition`: `2024`
    /// - `code`: A simple "Hello, world!" program
    ///
    /// # Returns
    ///
    /// A `FormatRequest` instance with sensible defaults for formatting Rust code.
    fn default() -> Self {
        Self {
            channel: Channel::Stable,
            crate_type: CrateType::Binary,
            edition: Edition::Edition2024,
            code: "fn main() { println!(\"Hello, world!\"); }".to_owned(),
        }
    }
}

/// Response structure returned after formatting Rust code.
///
/// Contains success status, exit details, and the formatted code.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormatResponse {
    /// Indicates whether formatting was successful.
    pub success: bool,

    /// Details about the formatting process exit (exit code, etc.).
    #[serde(rename = "exitDetail")]
    pub exit_detail: String,

    /// The resulting formatted Rust source code.
    pub code: String,
}
