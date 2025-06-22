# Dev-tools-rs

A **native application** inspired by the [Developer Tools IntelliJ Plugin](https://plugins.jetbrains.com/plugin/21904-developer-tools), built with **Rust** and **egui**.

![til](./blob/master/example.gif)

---

## Features

Some of these features are work in progress.

- **Color Picker**  
  Easily pick colors and copy their values.

- **Encoding and Decoding**

  - JWT (JSON Web Tokens)
  - Base32, Base64, URL Base64, MIME Base64
  - URL encoding

- **Text Utilities**

  - Regular Expression Matcher
  - UUID, ULID, Nano ID, and Password Generator
  - Text Sorting
  - Text Case Transformation
  - Text Diff Viewer
  - Text Format Conversion
  - Text Filter

- **JSON Tools**

  - JSON Path Parser
  - JSON Schema Validator

- **Hashing Tools**  
  Generate hashes using various algorithms.

- **Date and Time Handling**

  - Unix Timestamps
  - Formatting and conversions

- **Unit Converters**

  - Time
  - Data Size
  - Transfer Rates

- **Lorem Ipsum Generator**  
  Quickly generate placeholder text for your projects.

---

## Installation

### Download the latest release

**[Download v0.1.0](https://github.com/chojs23/dev-tools-rs/releases/tag/v0.1.0)**

#### Available Downloads:

- **macOS (ARM64)**: [dev-tools-rs-v0.1.0-macos-arm64.tar.gz](https://github.com/chojs23/dev-tools-rs/releases/download/v0.1.0/dev-tools-rs-v0.1.0-macos-arm64.tar.gz)

> **Note**: Currently only macOS ARM64 binaries are available. Windows and Linux builds will be added in future releases.

### Build from Source

#### Prerequisites

- **Rust**: Version 1.70.0 or later
- **Git**: For cloning the repository

#### Installation Steps

1. **Install Rust**
   
   If you don't have Rust installed, install it via [rustup](https://rustup.rs/):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

2. **Clone the Repository**
   ```bash
   git clone https://github.com/chojs23/dev-tools-rs.git
   cd dev-tools-rs
   ```

3. **Build the Application**
   
   For development build:
   ```bash
   cargo build
   ```
   
   For optimized release build:
   ```bash
   cargo build --release
   ```

4. **Run the Application**
   
   From development build:
   ```bash
   cargo run
   ```
   
   From release build:
   ```bash
   ./target/release/dev-tools-rs
   ```

#### Platform-Specific Notes

- **macOS**: No additional dependencies required
- **Linux**: You may need to install development packages:
  ```bash
  # Ubuntu/Debian
  sudo apt update
  sudo apt install build-essential pkg-config libfontconfig1-dev
  
  # Fedora/RHEL
  sudo dnf install gcc pkg-config fontconfig-devel
  
  # Arch Linux
  sudo pacman -S base-devel pkg-config fontconfig
  ```
- **Windows**: Make sure you have the Microsoft C++ Build Tools installed

#### Development

For development with hot reload:
```bash
cargo run
```

To run tests:
```bash
cargo test
```

To check code formatting:
```bash
cargo fmt --check
```

To run clippy for linting:
```bash
cargo clippy
```

## Usage

Launch the app and select the desired tool from the main menu.
Configure settings and preferences as needed in the Settings section.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
