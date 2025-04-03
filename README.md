# hamming-rust 🦀

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/hamming-rust)

A robust implementation of Hamming code error correction and a custom data link layer protocol (GUS Protocol) in Rust 🚀

## Features

- **Hamming Code Implementation**: Provides error detection and single-bit error correction
- **Custom GUS Protocol**: Great Unused Standard Protocol for reliable data transmission
- **Efficient BitVector**: Custom BitVec implementation for bit-level operations
- **CLI Interface**: Easy-to-use command line interface for sending and receiving data
- **Performance Benchmarks**: Includes criterion-based benchmarks for encoding/decoding
- **Error Handling**: Comprehensive error handling with detailed logging

## Protocol Description

The GUS (Great Unused Standard) Protocol is structured as follows:

```
+----------------+--------+----------------+------------------+
| Protocol Name  | Ver.   | Length Fields  |  Payload         |
| (3 bytes)      | (1b)   | (16/32 bytes)  |  (Variable)      |
+----------------+--------+----------------+------------------+
     "GUS"         0x01    Data + Bits Len   Hamming-coded
```

### Frame Details

- **Protocol Name**: Fixed 3-byte identifier "GUS"
- **Version**: 1-byte version number (currently 0x01)
- **Length Fields**: Two size_t values containing:
  - Data length (in bytes)
  - Bits length (total bits in payload)
- **Payload**: Hamming-encoded data with error correction

## Usage

### Building

```bash
cargo build --release
```

### Running

As a sender:

```bash
./hamming_rust -t binary sender -d "01101001"
```

As a receiver:

```bash
./hamming_rust -t binary receiver
```

You can pipe data between sender and receiver:

```bash
./hamming_rust -t binary sender -d "01101001" | ./hamming_rust -t binary receiver
```

### Data Types

The program supports two data types:

- `binary`: Direct binary string input/output
- `text`: ASCII text that gets converted to binary

## Error Correction

The implementation uses Hamming code for error detection and correction:

1. Sender:
   - Encodes the input data using Hamming code
   - Has a 50% chance of introducing a single-bit error (for testing)
   - Wraps the encoded data in the GUS protocol frame

2. Receiver:
   - Extracts the payload from the GUS protocol frame
   - Detects and corrects any single-bit errors
   - Reports if error correction was performed
   - Outputs the original data

## Project Structure

```
hamming-rust/
├── src/
│   ├── cli/            # Command-line interface
│   ├── encoding/       # Hamming & BitVector implementations
│   ├── proto/         # GUS Protocol implementation
│   └── utils/         # Utility functions
├── benches/           # Performance benchmarks
└── tests/            # Unit and integration tests
```

## Performance

The project includes comprehensive benchmarks for:

- Hamming code encoding/decoding
- Various data sizes (128, 1024, 8192, 65536 bits)
- Error correction scenarios

Run benchmarks with:

```bash
cargo bench
```

## Logging

The implementation includes a sophisticated logging system with:

- Multiple log levels (ERROR, WARN, INFO, DEBUG, TRACE)
- Colored output for better visibility
- Timestamp and context information
- Split logging (errors to stderr, other logs to stdout)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details.
