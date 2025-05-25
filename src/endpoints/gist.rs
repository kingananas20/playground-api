use serde::{Deserialize, Serialize};

/// A request to create a new Gist on the Rust playground.
///
/// Contains the code snippet to be shared.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GistCreateRequest {
    /// The Rust code to include in the Gist.
    pub code: String,
}

impl GistCreateRequest {
    /// Creates a new [`GistCreateRequest`] with the given code.
    ///
    /// # Arguments
    ///
    /// * `code` - The Rust code to be included in the Gist.
    ///
    /// # Returns
    ///
    /// A new [`GistCreateRequest`] containing the provided code.
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

/// A response returned after creating or retrieving a Gist.
///
/// Contains the Gist's unique ID, URL, and the stored code.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GistResponse {
    /// The unique identifier of the Gist.
    pub id: String,
    /// The public URL of the Gist.
    pub url: String,
    /// The Rust code stored in the Gist.
    pub code: String,
}
