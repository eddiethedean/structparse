//! Type system module for formatparse PyO3 bindings
//!
//! This module provides PyO3-specific type conversions.
//! Core types (FieldType, FieldSpec) come from formatparse-core.

pub mod conversion;

// Re-export core types for convenience
pub use formatparse_core::{FieldType, FieldSpec};
pub use formatparse_core::strftime_to_regex;

