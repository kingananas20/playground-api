use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("reqwest error: {0}")]
    ReqWest(#[from] reqwest::Error),

    #[error("url parse error: {0}")]
    Url(#[from] url::ParseError),

    #[error("request status code was not successful: {0}")]
    NoSuccess(u16),
}
