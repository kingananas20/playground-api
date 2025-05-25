//! playground-api: Rust Playground API Client
//! ===========================================
//!
//! `playground-api` is a Rust crate designed to simplify interaction with the
//! Rust Playground API. It provides both an asynchronous client by default and
//! an optional blocking client when compiled with the `blocking` feature.
//!
//! ## Features
//!
//! - **Async client** (default): uses `reqwest::Client` under the hood to send
//!   non-blocking HTTP requests.  
//! - **Blocking client** (`blocking` feature): enables `reqwest::blocking::Client`
//!   for environments where async is not desired or available.
//!
//! ## Installation
//!
//! Add to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! playground-api = "0.3.0"  # or latest
//! ```
//!
//! To enable the blocking client as well:
//!
//! ```toml
//! [dependencies]
//! playground-api = { version = "0.3.0", features = ["blocking"] }
//! ```
//!
//! ## Usage
//!
//! ### Async (default)
//!
//! ```rust
//! use playground_api::{Client, ExecuteRequest, Channel, Mode, Edition, Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Error> {
//!     // Uses the official https://play.rust-lang.org/ by default
//!     let client = Client::default();
//!
//!     let req = ExecuteRequest::new(
//!         Channel::Stable,
//!         Mode::Release,
//!         Edition::Edition2021,
//!         false,
//!         false,
//!         r#"println!("Hello, async world!");"#.into(),
//!     );
//!
//!     let res = client.execute(&req).await?;
//!     println!("{}", res.stdout);
//!     Ok(())
//! }
//! ```
//!
//! ### Blocking (with `blocking` feature)
//!
//! ```rust
//! use playground_api::{blocking::Client, ExecuteRequest, Channel, Mode, Edition, Error};
//!
//! fn main() -> Result<(), Error> {
//!     // Compile your crate with `--features blocking`
//!     let client = Client::default();
//!
//!     let req = ExecuteRequest::new(
//!         Channel::Stable,
//!         Mode::Release,
//!         Edition::Edition2021,
//!         false,
//!         false,
//!         r#"println!("Hello, blocking world!");"#.into(),
//!     );
//!
//!     let res = client.execute(&req)?;
//!     println!("{}", res.stdout);
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License.

#[cfg(feature = "blocking")]
pub mod blocking;

mod client;
pub mod endpoints;
mod error;

pub use client::Client;
pub use error::Error;
