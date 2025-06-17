# Developer Tools App

A **native application** inspired by the [Developer Tools IntelliJ Plugin](https://plugins.jetbrains.com/plugin/21904-developer-tools), built with **Rust** and **egui**.

![til](./blob/master/example.gif)

---

## Features

_These features are a work in progress_

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

will be added

### build from source

1. **Clone the repository**

2. Build the application: Ensure you have Rust and Cargo installed. Then run:
   ```bash
   cargo build --release
   ```
3. Run the application:
   ```bash
    cargo run --release
   ```
4. (Optional) Create a standalone executable: The release build can be found in the target/release directory. Copy this file to a directory in your PATH for easy access.

## Usage

Launch the app and select the desired tool from the main menu.
Configure settings and preferences as needed in the Settings section.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
