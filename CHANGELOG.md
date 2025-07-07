# Changelog

All notable changes to this project will be documented in this file.

## [0.2.0] - 2025-07-07

### Added

- **Cryptography Module**: Complete cryptographic operations support
  - **Symmetric Encryption**: AES (128/192/256-bit), DES, and Triple DES algorithms
  - **Asymmetric Encryption**: RSA (512/1024/2048/3072/4096-bit) and ECDSA support
  - **Cipher Modes**: ECB and CBC modes for symmetric algorithms
  - **Key Management**: Random key and IV generation with proper validation
  - **Encoding Options**: Hex and Base64 output encoding for encrypted data
  - **Digital Signatures**: RSA signing and verification capabilities
  - **Asynchronous Key Generation**: Non-blocking RSA key pair generation
  - **Error Handling**: Comprehensive error handling for cryptographic operations

### Enhanced

- **User Interface**: Improved cryptography panel with intuitive controls
  - Algorithm selection (AES, DES, Triple DES, RSA)
  - Operation selection (Encrypt, Decrypt, Sign, Verify)
  - Key size selection for AES and RSA
  - Cipher mode selection (ECB, CBC)
  - Output encoding selection (Hex, Base64)
  - Random key/IV generation buttons
  - Async key generation with progress indicators

### Added - Testing

- **Comprehensive Test Suite**: Full test coverage for cryptographic operations
  - Unit tests for all symmetric encryption algorithms
  - Unit tests for all asymmetric encryption algorithms
  - Integration tests for the cryptography processor
  - Key generation validation tests
  - Error handling and edge case tests
  - Performance benchmarks for RSA operations
  - Async key generation tests

### Technical Improvements

- **Code Quality**: Enhanced code structure and documentation
- **Performance**: Optimized cryptographic operations
- **Security**: Proper random number generation for keys and IVs
- **Modularity**: Well-structured crypto module with separate symmetric/asymmetric components

## [0.1.0] - 2025-06-22

### Added

- Initial release of Dev-tools-rs
- Color Picker with copy-to-clipboard functionality
- Encoding and Decoding tools:
  - JWT (JSON Web Tokens)
  - Base32, Base64, URL Base64, MIME Base64
  - URL encoding
- Text Utilities:
  - Regular Expression Matcher
  - UUID, ULID, Nano ID, and Password Generator
- Date and Time Handling:
  - Unix Timestamps
  - Formatting and conversions
