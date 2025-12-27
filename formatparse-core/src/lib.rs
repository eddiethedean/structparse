//! formatparse-core: Core Rust library for parsing strings using Python format() syntax
//!
//! This crate contains the pure Rust logic for pattern parsing, regex generation,
//! and type definitions. It has no dependencies on Python or PyO3.

pub mod error;
pub mod types;
pub mod parser;
// pub mod datetime;  // TODO: Extract pure Rust datetime utilities

pub use types::{FieldType, FieldSpec};
pub use types::regex::strftime_to_regex;
pub use parser::regex::*;

