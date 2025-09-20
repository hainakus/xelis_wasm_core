# XELIS WASM Node.js Bindings

This project provides WebAssembly (WASM) bindings for the XELIS hash function, optimized for Node.js.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- [Node.js](https://nodejs.org/) (v16 or later)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

## Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/hainakus/xelis-wasm-nodejs.git
   cd xelis-wasm-nodejs
   ```

2. Install Node.js dependencies:
   ```bash
   npm install
   ```

## Building the WASM Module

To build the WASM module for Node.js:

```bash
npm run build
```

This will create a `pkg` directory containing the compiled WASM module and JavaScript bindings.

## Usage

### Basic Example

```javascript
import * as wasm from './pkg/wasm_lib.js';

// Initialize the module (required for panic handling)
wasm.init_panic_hook();

// Hash some data
const data = new TextEncoder().encode('Hello, XELIS!');
const hashHex = wasm.xelis_hash_hex(data);
console.log('Hash:', hashHex);
```

### Running the Example

```bash
node --experimental-vm-modules examples/test.js
```

## API Reference

### Functions

- `init_panic_hook()`: Initialize panic hook for better error messages
- `xelis_hash(data: &[u8]) -> Vec<u8>`: Hash data and return as bytes
- `xelis_hash_hex(data: &[u8]) -> String`: Hash data and return as hex string
- `hash_string(input: &str) -> String`: Hash a string and return as hex
- `bytes_to_hex(data: &[u8]) -> String`: Convert bytes to hex string
- `hash_with_metadata(data: &[u8]) -> JsValue`: Get hash with additional metadata
- `batch_hash(data_slices: Array<Uint8Array>) -> Array<Uint8Array>`: Hash multiple inputs at once

## Development

### Testing

Run the test suite:

```bash
npm test
```

### Building for Production

For production builds, use the release profile which enables optimizations:

```bash
wasm-pack build --target nodejs --out-dir pkg --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
