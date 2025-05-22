use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GistCreateRequest {
    pub code: String,
}

impl GistCreateRequest {
    pub fn new(code: String) -> Self {
        Self { code }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GistResponse {
    pub id: String,
    pub url: String,
    pub code: String,
}
