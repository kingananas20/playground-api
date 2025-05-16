# Playground API

[![Crates.io](https://img.shields.io/crates/v/playground-api.svg)](https://crates.io/crates/playground-api)
[![Documentation](https://docs.rs/playground-api/badge.svg)](https://docs.rs/playground-api)
[![License](https://img.shields.io/crates/l/playground-api.svg)](LICENSE)

`playground-api` is a Rust crate designed to simplify interaction with the Rust Playground API. It provides an easy-to-use interface for executing Rust code snippets programmatically.

## Features

- Execute Rust code in the Rust Playground directly from your application.
- Customize the execution environment (e.g., cargo tools, edition, etc.).
- Retrieve detailed output, including compilation errors or runtime results.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
playground-api = "0.1.0" # Replace with the latest version
```

## Usage

To use the crate, first ensure you have an internet connection as it interacts with the Rust Playground API. Below is a simple example showcasing the .execute feature:

```rust
// Create a new client with an url to the playground of your choice (here the official one)
let client = Client::new("https://play.rust-lang.org/");

// Create a new ExecuteRequest
let req = ExecuteRequest::new(
    Channel::Stable, 
    Mode::Release, 
    Edition::Edition2024, 
    false, 
    false, 
    "println!(\"Hello, world!\")".to_string()
)

// send the ExecuteRequest to the playground server and receive the result
let res = client.execute(&req).await.unwrap();
```

### Example Output

    Example Output:
    Hello, world!

## Currently supported methods

The following methods are already supported

```rust
client.execute
client.format
```

## Documentation

For full documentation, including advanced usage and customization, visit docs.rs.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/kingananas20/playground-api/LICENSE) file for details.