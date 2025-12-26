use crate::error;
use regex::Regex;
use pyo3::prelude::*;

/// Build a regex from a pattern string with DOTALL flag
pub fn build_regex(pattern: &str) -> PyResult<Regex> {
    // Pre-allocate with estimated capacity
    let mut regex_with_flags = String::with_capacity(pattern.len() + 4);
    regex_with_flags.push_str("(?s)");
    regex_with_flags.push_str(pattern);
    Regex::new(&regex_with_flags).map_err(|e| error::regex_error(&e.to_string()))
}

/// Build a case-insensitive regex from a pattern string with DOTALL flag
pub fn build_case_insensitive_regex(pattern: &str) -> Option<Regex> {
    // Pre-allocate with estimated capacity
    let mut regex_with_flags = String::with_capacity(pattern.len() + 8);
    regex_with_flags.push_str("(?s)(?i)");
    regex_with_flags.push_str(pattern);
    Regex::new(&regex_with_flags).ok()
}

/// Remove anchors and flags from a regex string for search operations
/// Returns a string slice or owned string as needed
pub fn prepare_search_regex(regex_str: &str) -> String {
    let mut start = 0;
    let mut end = regex_str.len();
    
    // Remove (?s) flag if present
    if regex_str.starts_with("(?s)") {
        start = 4;
    }
    
    // Remove ^ anchor
    if regex_str[start..].starts_with("^") {
        start += 1;
    }
    
    // Remove $ anchor
    if regex_str[..end].ends_with("$") {
        end -= 1;
    }
    
    // Only allocate if we need to modify the string
    if start > 0 || end < regex_str.len() {
        regex_str[start..end].to_string()
    } else {
        regex_str.to_string()
    }
}

/// Build a search regex (without anchors) with optional case sensitivity
pub fn build_search_regex(regex_str: &str, case_sensitive: bool) -> PyResult<Regex> {
    let search_regex_str = prepare_search_regex(regex_str);
    
    // Pre-allocate with estimated capacity
    let capacity = search_regex_str.len() + if case_sensitive { 4 } else { 8 };
    let mut pattern = String::with_capacity(capacity);
    pattern.push_str("(?s)");
    if !case_sensitive {
        pattern.push_str("(?i)");
    }
    pattern.push_str(&search_regex_str);
    
    Regex::new(&pattern).map_err(|e| error::regex_error(&e.to_string()))
}

