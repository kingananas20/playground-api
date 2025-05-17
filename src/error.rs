use thiserror::Error;

/// Represents all possible errors that can occur while interacting with the Rust playground API.
#[derive(Error, Debug)]
pub enum Error {
    /// An error originating from the `reqwest` HTTP client.
    ///
    /// This may include network failures, timeout errors, or unexpected HTTP behavior.
    #[error("reqwest error: {0}")]
    ReqWest(#[from] reqwest::Error),

    /// An error occurred while parsing a URL.
    ///
    /// Typically triggered when constructing or joining a malformed URL.
    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    /// The HTTP response status code was not in the 2xx success range.
    ///
    /// Contains the numeric status code of the failed response.
    #[error("request status code was not successful: {0}")]
    NoSuccess(u16),
}
