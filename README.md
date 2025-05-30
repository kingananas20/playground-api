# Playground API

[![Crates.io](https://img.shields.io/crates/v/playground-api.svg)](https://crates.io/crates/playground-api)
[![Documentation](https://docs.rs/playground-api/badge.svg)](https://docs.rs/playground-api)
[![Rust Version](https://img.shields.io/badge/rustc-1.56+-blue.svg)](https://blog.rust-lang.org)
[![Downloads](https://img.shields.io/crates/d/playground-api.svg)](https://crates.io/crates/playground-api)
[![License](https://img.shields.io/crates/l/playground-api.svg)](LICENSE)
![Stars](https://img.shields.io/github/stars/kingananas20/playground-api)
![Forks](https://img.shields.io/github/forks/kingananas20/playground-api)

`playground-api` is a Rust crate designed to simplify interaction with the Rust Playground API. It provides an easy-to-use interface for executing Rust code snippets programmatically. It supports async and blocking Clients.

## Features

- Execute Rust code in the Rust Playground directly from your application.
- Retrieve detailed output, including compilation errors or runtime results.
- All while being rusty

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
playground-api = "0.2" # Replace with the latest version
```

## Usage

To use the crate, first ensure you have an internet connection as it interacts with the Rust Playground API. Below is a simple example showcasing the .execute feature:

```rust
// Create a new ExecuteRequest with the default values
let req = ExecuteRequest::default();

// Create a new client with an url to the playground of your choice (here the official one)
let client = Client::new("https://play.rust-lang.org/");

// send the ExecuteRequest to the playground server and receive the result
let res = client.execute(&req).await.unwrap();
```

### Example Output

    Hello, world!

## Supported methods

The following methods are supported

```rust
client.execute                  // Execute code
client.compile                  // Compile code to different targets
client.format                   // Format code using rustfmt
client.clippy                   // Use clippy on your code
client.miri                     // Run your code with the Miri interpreter
client.macro_expansion          // Expand your used macros
client.crates                   // Get the available crates
client.versions                 // Get the current rustc, rustfmt, clippy and miri versions
client.gist_create              // Create a new gist with your code
client.gist_get                 // Get a gist with an id
```

## Documentation

For full documentation, including advanced usage and customization, visit [docs.rs](https://docs.rs/playground-api).

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/kingananas20/playground-api/LICENSE) file for details.