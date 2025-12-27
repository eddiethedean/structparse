# formatparse-core

Pure Rust library for parsing strings using Python format() syntax.

This crate contains the core logic for pattern parsing, regex generation, and type definitions. It has **no dependencies on Python or PyO3**, making it suitable for:

- Testing without Python installed
- Use in pure Rust projects
- Integration into other language bindings

## Status

This crate is a work in progress. Currently extracted modules:

- ✅ `types` - FieldType and FieldSpec definitions
- ✅ `types::regex` - Regex pattern generation
- ✅ `parser::regex` - Regex building utilities
- ✅ `error` - Pure Rust error types

## Testing

All tests can run without Python:

```bash
cargo test --package formatparse-core
```

## Usage

This crate is primarily intended for use by the `formatparse-pyo3` crate, which provides Python bindings. However, you can use it directly in Rust projects:

```rust
use formatparse_core::{FieldType, FieldSpec};

let spec = FieldSpec {
    name: Some("age".to_string()),
    field_type: FieldType::Integer,
    width: None,
    precision: None,
    alignment: None,
    sign: None,
    fill: None,
    zero_pad: false,
    strftime_format: None,
    original_type_char: None,
};
```

