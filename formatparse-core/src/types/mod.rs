//! Type definitions for field specifications

pub mod definitions;
pub mod regex;

pub use definitions::{FieldType, FieldSpec};
pub use regex::strftime_to_regex;
