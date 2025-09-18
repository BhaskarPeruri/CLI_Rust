# Markdown to HTML Converter

A simple command-line tool written in Rust that converts Markdown files to HTML.

## Features

- Converts Markdown files to well-formatted HTML
- Supports common Markdown features including strikethrough
- Customizable output file path
- Clean, semantic HTML output with proper document structure

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system
2. Clone this repository
3. Build the project:
   ```bash
   cargo build --release
   ```

The binary will be available at `target/release/cli_rust` (or `target\release\cli_rust.exe` on Windows)

## Usage

Basic usage:
```bash
cargo run -- -i input.md -o output.html
```

### Arguments

- `-i, --input <INPUT>`: Path to the input Markdown file (required)
- `-o, --output <OUTPUT>`: Path for the output HTML file (optional, will print to console if not provided)

### Example

Convert `sample.md` to `output.html`:
```bash
cargo run -- -i sample.md -o output.html
```

## Dependencies

- [clap](https://crates.io/crates/clap): For command-line argument parsing
- [maud](https://crates.io/crates/maud): For HTML templating
- [pulldown-cmark](https://crates.io/crates/pulldown-cmark): For Markdown to HTML conversion

## Building from Source

1. Clone the repository
2. Run `cargo build` for development or `cargo build --release` for an optimized build
3. The binary will be available in `target/debug/` or `target/release/`

## License

This project is open source and available under the [MIT License](LICENSE).
