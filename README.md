# AnyLang - Pluggable Language Parser Framework

AnyLang is a flexible language parsing framework that supports dynamically loadable parser modules. Parsers are compiled as separate shared libraries (.so files on Linux) and can be loaded at runtime.

## Architecture

The project consists of two main components:

### 1. Main Program (`anylang`)
The core application that:
- Loads parser libraries dynamically using FFI
- Handles command-line arguments
- Reads input files
- Writes parsed output

### 2. Parser Modules (e.g., `lisp-parser`)
Standalone libraries that:
- Implement language-specific parsing logic
- Export a C-compatible FFI interface
- Can be compiled and distributed independently

## Building

Build everything:
```bash
cargo build
```

Build only the main program:
```bash
cargo build -p anylang
```

Build only a specific parser:
```bash
cargo build -p lisp-parser
```

## Usage

```bash
anylang -p <parser-library> <input-file> [-o <output-file>]
```

### Example: Parsing Lisp

```bash
cargo run -p anylang -- -p target/debug/liblisp_parser.so test.lisp -o output.txt
```

Or using the built binary directly:
```bash
./target/debug/anylang -p ./target/debug/liblisp_parser.so input.lisp
```

### Options

- `-p, --parser <path>`: Path to the parser library (.so file) [Required]
- `-o, --output <path>`: Output file path [Optional]
- `-h, --help`: Show help message

## Creating New Parsers

To create a new parser module:

1. Create a new crate in the workspace:
```bash
cargo new --lib my-parser
```

2. Update `Cargo.toml` to build as a dynamic library:
```toml
[lib]
crate-type = ["cdylib"]
```

3. Implement the required FFI interface:

```rust
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn parser_name() -> *const c_char {
    CString::new("MyParser").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn parser_extensions() -> *const c_char {
    CString::new("ext1,ext2").unwrap().into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn parse(input: *const c_char) -> *mut c_char {
    // Your parsing logic here
    // Return "ERROR: message" on error
    // Return formatted AST on success
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe { let _ = CString::from_raw(s); }
    }
}
```

4. Add the new crate to the workspace members in the root `Cargo.toml`:
```toml
[workspace]
members = ["anylang", "lisp-parser", "my-parser"]
```

## Included Parsers

### Lisp Parser
- **Library**: `liblisp_parser.so`
- **Supported extensions**: `.lisp`, `.lsp`, `.cl`
- **Features**:
  - S-expression parsing
  - Atoms, numbers, and lists
  - Nested expressions

## Benefits of This Architecture

1. **Modularity**: Parsers are completely independent modules
2. **Extensibility**: Add new language parsers without modifying core code
3. **Distribution**: Parsers can be distributed and updated separately
4. **Performance**: Parsers are compiled to native code
5. **Language Flexibility**: Each parser can use its own dependencies

## Project Structure

```
anylang/
├── Cargo.toml              # Workspace configuration
├── anylang/                # Main application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs         # Entry point with dynamic loading
│       └── option.rs       # CLI argument parsing
├── lisp-parser/            # Lisp parser module
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs          # Parser implementation with FFI
└── README.md
```

## License

MIT
